extern crate serde;
extern crate serde_json;
extern crate steamwebapi;
extern crate hyper;
extern crate redis;

mod serverqueries;

fn get_apikey() -> String {
	match std::env::var("STEAM_APIKEY") {
		Ok(key) => key,
		Err(_) => panic!("No Steam API key found. Set the STEAM_APIKEY environment variable.")
    }
}
fn main() {
    let c = steamwebapi::ApiClient::new(get_apikey());
    println!("{:?}", c.get_player_summary(76561197970498549).unwrap());
    println!("{:?}", c.get_player_server(76561197970498549).unwrap());
}
