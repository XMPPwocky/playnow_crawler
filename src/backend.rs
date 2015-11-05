use steamid::SteamId;
use steamwebapi;
use redis;
use redis::Commands;
use postgres;
use QueueStatus;
use GameServerId;

quick_error! {
    #[derive(Debug)]
    pub enum BackendError {
        Redis(err: redis::RedisError) {
            from()
            cause(err)
            description(err.description())
        }
        Postgres(err: postgres::error::DbError) {
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

    postgres: postgres::Connection
}

impl Backend {
    pub fn new() -> Backend {
        let webapi = steamwebapi::ApiClient::new(::get_apikey());
        let redis_client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let redis = redis_client.get_connection().unwrap();
        let postgres = postgres::Connection::connect(
            &::get_postgres_url() as &str,
            &postgres::SslMode::None
            ).unwrap();

        Backend {
            steam_webapi: webapi,
            redis: redis,
            postgres: postgres
        }
    }

    pub fn auth_request(&mut self, sessionid: &str) -> BackendResult<Option<SteamId>> { 
        // FIXME: delete the sessionid too.

        let result: Option<u64> = try!(self.redis.get(format!("session_steamid:{}", sessionid)));

        let steamid = result.map(SteamId::from_u64);
        
        debug_assert_eq!(steamid.and_then(SteamId::get_universe), Some(::steamid::Universe::Public));
        debug_assert_eq!(steamid.and_then(SteamId::get_type), Some(::steamid::AccountType::Individual));

        Ok(result.map(SteamId::from_u64))
    }

    pub fn start_playing(&mut self, steamid: SteamId) -> BackendResult<()> {
        // okay, what are my favorite servers?
        let preferred_servers = try!(
            self.get_player_preferred_servers(steamid)
            );
        // search servers, maybe put in favorite server and return
        // now check all these servers. first, query Redis...
        // if not found, query the server directly, put in Redis,
        // use a reasonable TTL.
        // now, of those servers... are any OK?
        // if so... put them in there
        // if not... put in fallback server. then, put in queue...
        //
        // maybe put in fallback server and queue for favorites
        unimplemented!()
    }

    pub fn get_player_preferred_servers(&mut self, steamid: SteamId)
        -> BackendResult<Vec<GameServerId>> {
            unimplemented!()
        }


    pub fn get_queue_status(&mut self, steamid: SteamId) -> BackendResult<QueueStatus> {
        unimplemented!()
    }

    pub fn leave_queue(&mut self, steamid: SteamId) -> BackendResult<()> {
        unimplemented!()
    }
}
