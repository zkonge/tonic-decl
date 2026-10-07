mod common;

use tokio_stream::StreamExt;
use tonic::{
    server::NamedService,
    service::{interceptor::InterceptedService, Routes},
    transport::{
        server::{Router, TcpIncoming},
        Channel, Server,
    },
    Code, Request, Status,
};
use tonic_decl_tests::echo::{EchoClient, EchoServer, MissingClient};
use tower_service::Service;

use self::common::{req, MyEcho};

async fn connect(router: Router) -> Channel {
    let incoming = TcpIncoming::bind("127.0.0.1:0".parse().unwrap()).unwrap();
    let addr = incoming.local_addr().unwrap();
    tokio::spawn(router.serve_with_incoming(incoming));

    Channel::from_shared(format!("http://{addr}"))
        .unwrap()
        .connect()
        .await
        .unwrap()
}

async fn echo_client() -> EchoClient<Channel> {
    let svc = EchoServer::new(MyEcho).into_service();
    EchoClient::new(connect(Server::builder().add_service(svc)).await)
}

#[tokio::test]
async fn unary() {
    let mut client = echo_client().await;

    let reply = client.unary(req("hello")).await.unwrap().into_inner();
    assert_eq!(reply.message, "hello");
}

#[tokio::test]
async fn server_streaming() {
    let mut client = echo_client().await;

    let mut stream = client
        .server_streaming(req("hi"))
        .await
        .unwrap()
        .into_inner();
    let mut replies = Vec::new();
    while let Some(reply) = stream.message().await.unwrap() {
        replies.push(reply.message);
    }
    assert_eq!(replies, ["hi 0", "hi 1", "hi 2"]);
}

#[tokio::test]
async fn client_streaming() {
    let mut client = echo_client().await;

    let requests = tokio_stream::iter([req("a"), req("b"), req("c")]);
    let reply = client
        .client_streaming(requests)
        .await
        .unwrap()
        .into_inner();
    assert_eq!(reply.message, "abc");
}

#[tokio::test]
async fn bidi_streaming() {
    let mut client = echo_client().await;

    let requests = tokio_stream::iter([req("x"), req("y")]);
    let stream = client.bidi_streaming(requests).await.unwrap().into_inner();
    let replies = stream
        .map(|reply| reply.unwrap().message)
        .collect::<Vec<_>>()
        .await;
    assert_eq!(replies, ["x", "y"]);
}

#[tokio::test]
async fn unknown_method() {
    let svc = EchoServer::new(MyEcho).into_service();
    let channel = connect(Server::builder().add_service(svc)).await;

    let status = MissingClient::new(channel)
        .missing(req("x"))
        .await
        .unwrap_err();
    assert_eq!(status.code(), Code::Unimplemented);
}

#[tokio::test]
async fn routes() {
    let routes = Routes::new(EchoServer::new(MyEcho).into_service());
    let mut client = EchoClient::new(connect(Server::builder().add_routes(routes)).await);

    let reply = client.unary(req("routes")).await.unwrap().into_inner();
    assert_eq!(reply.message, "routes");
}

fn check_token(req: Request<()>) -> Result<Request<()>, Status> {
    match req.metadata().get("authorization") {
        Some(token) if token == "secret" => Ok(req),
        _ => Err(Status::unauthenticated("bad token")),
    }
}

#[tokio::test]
async fn interceptor() {
    let svc = InterceptedService::new(EchoServer::new(MyEcho).into_service(), check_token);
    let mut client = EchoClient::new(connect(Server::builder().add_service(svc)).await);

    let status = client.unary(req("x")).await.unwrap_err();
    assert_eq!(status.code(), Code::Unauthenticated);

    let mut request = Request::new(req("x"));
    request
        .metadata_mut()
        .insert("authorization", "secret".parse().unwrap());
    let reply = client.unary(request).await.unwrap().into_inner();
    assert_eq!(reply.message, "x");
}

#[tokio::test]
async fn max_decoding_message_size() {
    let svc = EchoServer::new(MyEcho)
        .max_decoding_message_size(8)
        .into_service();
    let mut client = EchoClient::new(connect(Server::builder().add_service(svc)).await);

    // 1 byte tag + 1 byte length + 5 bytes payload
    let reply = client.unary(req("short")).await.unwrap().into_inner();
    assert_eq!(reply.message, "short");

    let status = client.unary(req("too long message")).await.unwrap_err();
    assert_eq!(status.code(), Code::OutOfRange);
}

#[tokio::test]
async fn direct_call() {
    fn name<S: NamedService>(_: &S) -> &'static str {
        S::NAME
    }

    let mut svc = EchoServer::new(MyEcho).into_service();
    assert_eq!(name(&svc), "test.Echo");

    for path in [
        "/test.Echo/Nope",
        "/test.Echo/Unary/",
        "/test.EchoX/Unary",
        "/other.Echo/Unary",
        "/test.Echo",
    ] {
        let request = http::Request::builder()
            .uri(path)
            .body(tonic::body::Body::empty())
            .unwrap();
        let response = svc.call(request).await.unwrap();
        assert_eq!(response.headers()["grpc-status"], "12", "{path}");
    }
}
