use std::time::{ Duration, Instant };
use redis::Commands;
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
    redis_con: Option<redis::Connection>,
}

impl MyWebSocket {
    pub fn new() -> Self {
        Self { hb: Instant::now(), redis_con: None }
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

///
// One connection per actor:

// Pros:

// Isolation: Each actor has its own dedicated connection, ensuring isolation and avoiding potential race conditions or interference.
// Simplicity: Easier to manage and debug as connections are not shared.
// Scalability: Can handle a higher number of concurrent actors as each has its own connection.

// Cons:

// Resource usage: Creates more connections, leading to higher memory and CPU consumption on the Redis server and your application.
// Connection overhead: Establishing and maintaining multiple connections can add overhead, impacting performance.
// Single connection for all actors:

// Pros:

// Resource efficiency: Uses only one connection, reducing memory and CPU overhead.
// Lower connection overhead: Less time spent establishing and maintaining connections.

// Cons:

// Complexity: Sharing a connection requires careful synchronization and error handling to avoid conflicts and race conditions.
// Scalability limitations: Might reach the maximum allowed connections on the Redis server if the number of actors grows significantly.
// Performance bottlenecks: Shared connections could become bottlenecks during high concurrency, impacting performance for all actors.

// Redis nodes can have up to either 10,000 simultaneous connections
// or 4 simultaneous connections per megabyte of memory, whichever is larger.
///

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        // Establish Redis connection here
        let client = redis::Client
            ::open("redis://localhost:6379")
            .expect("Failed to connect to Redis");
        let con = client.get_connection().expect("Failed to get Redis connection");

        // Store the connection in a field for later use
        self.redis_con = Some(con);
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

                // Access the Redis connection

                const REDIS_PREFIX: &str = "redis://";

                if text.starts_with(REDIS_PREFIX) {
                    let redis_key = &text[REDIS_PREFIX.len()..];

                    let animation_names: Vec<&str> = redis_key.split(",").collect();

                    if let Some(con) = &mut self.redis_con {
                        for animation_name in animation_names {
                            // Access and use each part here
                            // Perform Redis operations as needed
                            let value: String = con.get(&animation_name).unwrap();

                            println!(
                                "received text size {} from redis key {}",
                                value.as_bytes().len(),
                                animation_name
                            );

                            // Concatenation here:
                            let message = format!("{}::{}", animation_name, value);

                            println!(
                                "Sending text size {} to websocket: {}",
                                message.as_bytes().len(),
                                message
                            );

                            ctx.text(message); // Send the concatenated message
                        }
                    } else {
                        // Handle the case where the connection is not established
                        println!("Redis connection not available");
                    }
                } else {
                    println!("received text {}", text)
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
}
