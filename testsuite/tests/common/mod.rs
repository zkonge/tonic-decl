#![allow(dead_code)]

use tokio_stream::{adapters::Map, Stream, StreamExt};
use tonic::{Request, Response, Result, Status, Streaming};
use tonic_decl_tests::{
    attrs::Attrs,
    echo::Echo,
    pb::{EchoReply, EchoRequest},
};

pub struct MyEcho;

pub fn req(message: &str) -> EchoRequest {
    EchoRequest {
        message: message.into(),
    }
}

fn reply(message: String) -> EchoReply {
    EchoReply { message }
}

fn echo(req: Result<EchoRequest>) -> Result<EchoReply> {
    req.map(|req| reply(req.message))
}

impl Echo for MyEcho {
    type BidiStreamingStream =
        Map<Streaming<EchoRequest>, fn(Result<EchoRequest>) -> Result<EchoReply>>;

    async fn unary(&self, req: Request<EchoRequest>) -> Result<Response<EchoReply>> {
        Ok(Response::new(reply(req.into_inner().message)))
    }

    // no `as` for this method, so the stream type can capture a closure
    async fn server_streaming(
        &self,
        req: Request<EchoRequest>,
    ) -> Result<Response<impl Stream<Item = Result<EchoReply>> + Send + 'static>> {
        let message = req.into_inner().message;
        let replies = tokio_stream::iter(0..3).map(move |i| Ok(reply(format!("{message} {i}"))));

        Ok(Response::new(replies))
    }

    async fn client_streaming(
        &self,
        req: Request<Streaming<EchoRequest>>,
    ) -> Result<Response<EchoReply>> {
        let mut stream = req.into_inner();
        let mut message = String::new();
        while let Some(req) = stream.message().await? {
            message.push_str(&req.message);
        }

        Ok(Response::new(reply(message)))
    }

    async fn bidi_streaming(
        &self,
        req: Request<Streaming<EchoRequest>>,
    ) -> Result<Response<Self::BidiStreamingStream>> {
        let stream = req.into_inner().map(echo as fn(_) -> _);

        Ok(Response::new(stream))
    }
}

pub struct MyAttrs;

impl Attrs for MyAttrs {
    async fn ready(&self, req: Request<EchoRequest>) -> Result<Response<EchoReply>> {
        Ok(Response::new(reply(req.into_inner().message)))
    }

    async fn old(&self, _: Request<EchoRequest>) -> Result<Response<EchoReply>> {
        Err(Status::failed_precondition("deprecated"))
    }

    async fn bidi(
        &self,
        req: Request<Streaming<EchoRequest>>,
    ) -> Result<Response<impl Stream<Item = Result<EchoReply>> + Send + 'static>> {
        let suffix = String::from("!");
        let stream = req
            .into_inner()
            .map(move |req| req.map(|req| reply(req.message + &suffix)));

        Ok(Response::new(stream))
    }
}
