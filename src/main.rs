extern crate serde;
extern crate serde_json;
extern crate steamwebapi;
extern crate hyper;
extern crate redis;
extern crate websocket;

use std::thread;
use websocket::{Server, Message, Sender, Receiver};
use websocket::message::Type;

mod serverqueries;

fn get_apikey() -> String {
    match std::env::var("STEAM_APIKEY") {
        Ok(key) => key,
        Err(_) => panic!("No Steam API key found. Set the STEAM_APIKEY environment variable.")
    }
}
fn main() {
    let steam_apikey = get_apikey();
    let mut ws_server = Server::bind("127.0.0.1:2794").unwrap();

    for connection in ws_server {
        if let Ok(connection) = connection {
            let steam_apikey = steam_apikey.clone();
            thread::spawn(move || {
                handler(connection, steam_apikey);
            });
        } else if let Err(e) = connection {
            println!("Error: {:?}", e);
            break;
        }
    }
}

type WebSocketConnection = websocket::server::Connection<
websocket::stream::WebSocketStream,
websocket::stream::WebSocketStream>;

fn handler(mut connection: WebSocketConnection, steam_apikey: String) {
    use websocket::server::request::RequestUri;
    use redis::Commands;
    use std::str::FromStr;

    println!("connect");

    let webapi = steamwebapi::ApiClient::new(steam_apikey);
    let redis_cli = redis::Client::open("redis://127.0.0.1/").unwrap();
    let redis_con = redis_cli.get_connection();
    // FIXME: unwraps probably DoS

    let request = connection.read_request().unwrap();
    request.validate().unwrap();
    //let headers = request.headers.clone();

    println!("{:?}", request.url);
    let sessionid = match request.url {
        RequestUri::AbsolutePath(ref path) => {
            if path.len() > 1 {
                Some(u64::from_str(&path[1..]))
            } else {
                None
            }
        },
        _ => None
    }.and_then(|sessionid| sessionid.ok()); 
    println!("{:?}", sessionid);

    let steamid = match sessionid.map(|sessionid| {
        let sessionid = format!("session:{:?}", sessionid);
        println!("{:?}", sessionid);
        let x = redis_cli.get(sessionid);
        println!("{:?}", x);
        x.ok()
    }) {
        Some(Some(steamid)) => steamid,
        _ => {
            request.fail().send().unwrap();
            return;
        }
    };

    let response = request.accept();
    let mut client = response.send().unwrap();

    // check in with redis here. check session, get SteamID...

    let (mut sender, mut receiver) = client.split();

    for message in receiver.incoming_messages() {
        let message: Result<Message, _> = message;
        if let Ok(message) = message {
            match message.opcode {
                Type::Close => {
                    let message = Message::close();
                    sender.send_message(&message).unwrap();
                    return;
                },
                Type::Ping => {
                    let message = Message::pong(message.payload);
                    sender.send_message(&message).unwrap();
                }
                Type::Text => {
                    let message = Message::text(format!("{:?}", webapi.get_player_summary(steamid)));
                    sender.send_message(&message).unwrap();
                }
                _ => sender.send_message(&message).unwrap(),
            }
        }
    }
}
