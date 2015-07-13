use {Response, Cookies};
use cookie::Cookie;
use plugin::{Plugin, Pluggable};
use typemap::Key;
use std::marker::PhantomData;
use std::any::Any;
use time::{Timespec, Duration, self};
use serialize::{Encodable, Decodable, json};
use byteorder::{ByteOrder, BigEndian};
use std::error::Error;
use std::str;
use std::fmt::Debug;

static COOKIE_KEY : &'static str = "__SESSION";

pub trait SessionStore {
    type Store: Encodable + Decodable + Default + Debug;

    fn timeout() -> Duration { Duration::minutes(60) }
}

// Plugin boilerplate
pub struct SessionPlugin<T: 'static + Any>(PhantomData<T>);
impl<T: 'static + Any> Key for SessionPlugin<T> { type Value = T; }

impl<'a, 'k, D, T> Plugin<Response<'a, 'k, D>> for SessionPlugin<T>
where Response<'a, 'k, D> : Cookies,
      T: 'static + Any + Encodable + Decodable + Default + Debug,
      D: SessionStore<Store=T>  {
    type Error = ();

    fn eval(res: &mut Response<'a, 'k, D>) -> Result<T, ()> {
        // Ensure our dependencies register their on_send
        // FIXME: would be nice if this was more robust, but at least for now
        // this minimizes the 'bug potential' to the library author rather than
        // the library user.
        let _ = res.cookies_mut().encrypted();

        // Schedule the session to be written when headers are being sent
        res.on_send(|res| {
            let encoded = {
                let session = res.session();
                encode_data(session)
            };

            let jar = res.cookies_mut().encrypted();
            let mut cookie = Cookie::new(COOKIE_KEY.into(), encoded);
            cookie.httponly = true;
            jar.add(cookie);
        });

        let jar = res.cookies_mut().encrypted();
        let data = jar.find(COOKIE_KEY).and_then(|cookie| {
            let timeout = <D as SessionStore>::timeout();
            match decode_data(&*cookie.value, timeout) {
                Ok(data) => Some(data),
                Err(e) => {
                    println!("Error parsing session: {:?}", e);
                    None
                }
            }
        });

        // Any error should reset the session
        Ok(data.unwrap_or_else(|| T::default()))
    }
}

fn decode_data<T: Decodable + Default>(raw: &str, timeout: Duration) -> Result<T, Box<Error + Send + Sync>> {
    use serialize::base64::FromBase64;

    let timestamp_and_plaintext = try!(raw.from_base64());

    let len = timestamp_and_plaintext.len();
    let (plaintext, timestamp) = timestamp_and_plaintext.split_at(len - 8);

    let timestamp = BigEndian::read_i64(timestamp);
    let plaintext = try!(str::from_utf8(plaintext));
    let timestamp = Timespec::new(timestamp, 0);

    if timestamp + timeout > time::now().to_timespec() {
        let decoded = try!(json::decode(plaintext));
        Ok(decoded)
    } else {
        // Reset the session, not an error
        Ok(T::default())
    }
}

fn encode_data<T: Encodable + Debug>(data: &T) -> String {
    use serialize::base64::{ToBase64, STANDARD};

    // TODO: log if this fails
    let json = match json::encode(data) {
        Ok(json) => json,
        Err(e) => {
            println!("Failed to encode '{:?}' as json: {:?}", data, e);
            return "".into()
        },
    };

    let mut raw = json.into_bytes();

    let mut timestamp = [0u8; 8];
    BigEndian::write_i64(&mut timestamp, time::now().to_timespec().sec);
    raw.extend(timestamp.iter().cloned());

    raw.to_base64(STANDARD)
}

pub trait Session {
    type Store;

    /// Provides access to an immutable Session.
    ///
    /// Currently requires a mutable reciever, hopefully this can change in future.
    fn session(&mut self) -> &Self::Store;

    /// Provides access to a mutable Session.
    fn session_mut(&mut self) -> &mut Self::Store;
}

impl<'a, 'k, D> Session for Response<'a, 'k, D>
where Response<'a, 'k, D> : Cookies,
      D: SessionStore,
      D::Store: 'static + Any + Encodable + Decodable + Default + Debug {
    type Store = D::Store;

    fn session(&mut self) -> &Self::Store {
        // Unwrap is safe as we reset the session on bad parses
        self.get_ref::<SessionPlugin<D::Store>>().unwrap()
    }

    fn session_mut(&mut self) -> &mut Self::Store {
        // Unwrap is safe as we reset the session on bad parses
        self.get_mut::<SessionPlugin<D::Store>>().unwrap()
    }
}
