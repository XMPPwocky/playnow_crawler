use serde_json;
use std;
use std::env;
use std::io::prelude::*;
use hyper;

/// Somewhat expensive. Cache this.
/// May panic.
fn get_apikey() -> String {
	match env::var("STEAM_APIKEY") {
		Ok(key) => key,
		Err(_) => panic!("No Steam API key found. Set the STEAM_APIKEY environment variable.")
    }
}

pub enum Error {
    HttpError(hyper::error::Error),
    BadBody(serde_json::Error),
    Io(std::io::Error)
}
impl std::convert::From<hyper::error::Error> for Error {
    fn from(e: hyper::error::Error) -> Error {
        Error::HttpError(e)
    }
}
impl std::convert::From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::BadBody(e)
    }
}
impl std::convert::From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::Io(e)
    }
}

pub struct ApiClient {
    http: hyper::client::Client,
    apikey: String
}
impl ApiClient {
    pub fn new() -> ApiClient {
        ApiClient {
            http: hyper::client::Client::new(),
            apikey: get_apikey()
        }
    }
    pub fn get_player_summaries(&self, steamids: &[u64]) -> Result<serde_json::Value, Error> { 
        let steamids_str = {
            let mut steamids_str = String::new();
            for steamid in steamids {
                steamids_str = steamids_str + "," + &steamid.to_string();
            }
            steamids_str
        };
        let endpoint = {
            let mut endpoint = hyper::Url::parse("http://api.steampowered.com/ISteamUser/GetPlayerSummaries/v0002/").unwrap();
            endpoint.set_query_from_pairs(vec![
                                          ("key", &self.apikey as &str),
                                          ("steamids", &steamids_str)
                                          ].into_iter());
            endpoint
        };

        let body = {
            let mut response = try!(self.http.get(endpoint).send());
            let mut body = String::new();
            try!(response.read_to_string(&mut body));
            body
        };
        // why doesn't try! work?
        serde_json::from_str(&body).map_err(std::convert::Into::into)
    }
}

