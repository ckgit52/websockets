use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse, Error};
use actix_web_actors::ws;
use actix_cors::Cors;
use std::time::Instant;
use actix::prelude::*;
use std::time::Duration;

use serde_json::json;
struct MyWebSocket {
    hb: Instant,
}

impl MyWebSocket {
    fn new() -> Self {
        Self { hb: Instant::now() }
    }
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.send_hello_kartik(ctx);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Ping(msg)) = msg {
            self.hb = Instant::now();
            ctx.pong(&msg);
        }
    }
}
impl MyWebSocket {
    fn send_hello_kartik(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_later(Duration::from_millis(100), |actor, ctx| {
            let message = json!({ "event": "message", "data": "hello kartik" }).to_string();
            ctx.text(message);
        });
    }
}

async fn ws_handler(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(MyWebSocket::new(), &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors) // Enable CORS
            .route("/ws", web::get().to(ws_handler))
    })
    .bind("0.0.0.0:8082")?
    .run()
    .await
}
