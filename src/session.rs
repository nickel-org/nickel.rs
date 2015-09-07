use {Request, Response, Cookies, cookies};
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

pub trait Store : cookies::KeyProvider {
    type Session: Encodable + Decodable + Default + Debug;

    fn timeout() -> Duration { Duration::minutes(60) }
}

// Plugin boilerplate
pub struct SessionPlugin<T: 'static + Any>(PhantomData<T>);
impl<T: 'static + Any> Key for SessionPlugin<T> { type Value = Option<T>; }

impl<'mw, D, T> Plugin<Response<'mw, D>> for SessionPlugin<T>
where T: 'static + Any + Encodable + Decodable + Default + Debug,
      D: Store<Session=T> {
    type Error = ();

    fn eval(response: &mut Response<'mw, D>) -> Result<Option<T>, ()> {
        // Ensure our dependencies register their on_send
        // FIXME: would be nice if this was more robust, but at least for now
        // this minimizes the 'bug potential' to the library author rather than
        // the library user.
        let _ = response.cookies_mut().encrypted();

        // Schedule the session to be written when headers are being sent
        response.on_send(|response| {
            let encoded = {
                // This should only ever get called after an initial setup, so these
                // unwraps should be fine!
                let session = response.get_mut::<SessionPlugin<D::Session>>()
                                      .unwrap();

                // Todo: track when a write has occurred and only create the cookie in
                // that case
                encode_data(session.as_ref().unwrap())
            };

            let jar = response.cookies_mut().encrypted();
            let mut cookie = Cookie::new(COOKIE_KEY.into(), encoded);
            cookie.httponly = true;
            jar.add(cookie);
        });

        Ok(None)
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
            println!("[Session] Failed to encode '{:?}' as json: {:?}", data, e);
            return "".into()
        },
    };

    let mut raw = json.into_bytes();

    let mut timestamp = [0u8; 8];
    BigEndian::write_i64(&mut timestamp, time::now().to_timespec().sec);
    raw.extend(timestamp.iter().cloned());

    raw.to_base64(STANDARD)
}

pub trait Session<D> where D: Store {
    /// Provides access to a mutable Session.
    fn get_mut<'a>(&mut Request<D>, &'a mut Response<D>) -> &'a mut D::Session;
}

pub struct CookieSession;

impl<D> Session<D> for CookieSession
where D: Store,
      D::Session : 'static + Any + Encodable + Decodable + Default + Debug {
    fn get_mut<'a>(req: &mut Request<D>, res: &'a mut Response<D>) -> &'a mut D::Session {
        let cached_session = res.get_mut::<SessionPlugin<D::Session>>().unwrap();
        if let Some(ref mut session) = *cached_session {
            return session
        }

        let jar = req.cookies().encrypted();
        let data = jar.find(COOKIE_KEY).and_then(|cookie| {
            let timeout = <D as Store>::timeout();
            match decode_data(&*cookie.value, timeout) {
                Ok(data) => Some(data),
                Err(e) => {
                    println!("Error parsing session: {:?}", e);
                    None
                }
            }
        });

        // Any error resets the session
        *cached_session = data.or_else(|| Some(<D::Session>::default()));
        cached_session.as_mut().unwrap()
    }
}
