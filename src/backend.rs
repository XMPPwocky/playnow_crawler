use steamid::SteamId;
use steamwebapi;
use redis;
use redis::Commands;
use QueueStatus;

quick_error! {
    #[derive(Debug)]
    pub enum BackendError {
        Redis(err: redis::RedisError) {
            from()
            cause(err)
            description(err.description())
        }
    }
}
pub type BackendResult<T> = Result<T, BackendError>;

pub struct Backend {
    steam_webapi: steamwebapi::ApiClient,

    redis: redis::Connection,
}

impl Backend {
    pub fn new() -> Backend {
        let webapi = steamwebapi::ApiClient::new(::get_apikey());
        let redis_client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let redis = redis_client.get_connection().unwrap();

        Backend {
            steam_webapi: webapi,
            redis: redis
        }
    }

    pub fn auth_request(&mut self, sessionid: &str) -> BackendResult<Option<SteamId>> { 
        let result: Option<u64> = try!(self.redis.get(format!("session_steamid:{}", sessionid)));

        Ok(result.map(SteamId::from_u64))
    }

    pub fn get_queue_status(&mut self, steamid: SteamId) -> BackendResult<QueueStatus> {
        unimplemented!()
    }

    pub fn leave_queue(&mut self, steamid: SteamId) -> BackendResult<()> {
        unimplemented!()
    }
}
