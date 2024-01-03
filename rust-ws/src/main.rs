use serde_json::json;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;

mod server;
use self::server::MyWebSocket;

async fn index() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "message": "Hello from the server!",
        "data": {
            "key1": "value1",
            "key2": 42
        }
    }))
}

/// WebSocket handshake and start `MyWebSocket` actor.
async fn echo_ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(MyWebSocket::new(), &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:3333");

    HttpServer::new(|| {
        App::new()
            // WebSocket UI HTML file
            .service(web::resource("/").to(index))
            // websocket route
            .service(web::resource("/ws").route(web::get().to(echo_ws)))
            // enable logger
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 3333))?
    .run()
    .await
}