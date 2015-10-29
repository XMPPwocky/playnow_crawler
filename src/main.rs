extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate redis;

mod webapi;
mod serverqueries;

fn main() {
    println!("Hello, world!");
    let mut c = webapi::ApiClient::new();
    println!("{:?}", c.get_player_summaries(&[76561197970498549]).unwrap());
    println!("{:?}", c.get_player_server(76561197970498549).unwrap());
}
