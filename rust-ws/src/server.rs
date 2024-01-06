use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// websocket connection is long running connection, it easier
/// to handle with an actor
pub struct MyWebSocket {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
}

impl MyWebSocket {
    pub fn new() -> Self {
        Self { hb: Instant::now() }
    }

    /// helper method that sends ping to client every 5 seconds (HEARTBEAT_INTERVAL).
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        //  the client should send ping/pong message within the  CLIENT_TIMEOUT period, 
        // because the heatbeat is checking, if it expires, the server will close the context. 
        // each time client send a ping (maybe also pong) message, it reset the expire time.
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

/// Handler for `ws::Message`
/// Handler for `ws::Message`
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // process websocket messages
        println!("WS: {msg:?}");
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                // Trigger file chunk sending based on a text message (replace with your trigger)
                if text == "send_data" {
                    // let file_path = "path/to/your/file"; // Replace with actual file path
                    // self.send_file_chunks(file_path, ctx);
                    // todo send corresponding data
                } else {
                    ctx.text(text);
                }
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }

    // async fn send_file_chunks(&mut self, file_path: &str, ctx: &mut Self::Context) {
    //     let mut file = tokio::fs::File::open(file_path).await.expect("Failed to open file");
    //     let mut buffer = [0; 10240]; // 10KB chunk size

    //     loop {
    //         let bytes_read = file.read(&mut buffer).await.expect("Failed to read file");
    //         if bytes_read == 0 {
    //             break; // End of file
    //         }

    //         ctx.binary(buffer[..bytes_read].to_vec()).await.expect("Failed to send chunk");
    //         // Potentially add a delay or await further instructions here
    //     }
    // }
}
