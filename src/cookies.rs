use {Request, Response};
use plugin::{Plugin, Pluggable};
use typemap::Key;
use hyper::header;
use rand::{self, Rng};

use cookie::CookieJar;

#[derive(Clone)]
// Let's not derive `Copy` as that seems like a bad idea for key data
pub struct SecretKey(pub [u8; 32]);

impl SecretKey {
    pub fn new<T: AsRef<[u8]>>(arr: T) -> Result<SecretKey, &'static str> {
        let arr = arr.as_ref();
        if arr.len() != 32 { return Err("Key length must be 32") }

        let mut key = [0; 32];
        for idx in 0..32 {
            key[idx] = arr[idx]
        }

        Ok(SecretKey(key))
    }
}

// Plugin boilerplate
pub struct CookiePlugin;
impl Key for CookiePlugin { type Value = CookieJar<'static>; }

impl<'mw, 'conn, D> Plugin<Request<'mw, 'conn, D>> for CookiePlugin
where D: KeyProvider {
    type Error = ();

    fn eval(req: &mut Request<D>) -> Result<CookieJar<'static>, ()> {
        let key = req.data().key();
        let jar = match req.origin.headers.get::<header::Cookie>() {
            Some(c) => c.to_cookie_jar(&key.0),
            None => CookieJar::new(&key.0)
        };

        Ok(jar)
    }
}

impl<'a, 'b, 'k, D> Plugin<Response<'a, D>> for CookiePlugin
where D: KeyProvider {
    type Error = ();

    fn eval(res: &mut Response<'a, D>) -> Result<CookieJar<'static>, ()> {
        // Schedule the cookie to be written when headers are being sent
        res.on_send(|res| {
            let header = {
                let jar = res.get_ref::<CookiePlugin>().unwrap();
                header::SetCookie::from_cookie_jar(jar)
            };
            res.set(header);
        });

        let key = res.data().key();
        Ok(CookieJar::new(&key.0))
    }
}

/// Trait to whitelist access to `&'mut CookieJar` via the `Cookies` trait.
pub trait AllowMutCookies {}
impl<'a, D> AllowMutCookies for Response<'a, D> {}

/// Provide the key used for decoding secure CookieJars
///
/// Cookies require a random key for their signed and encrypted cookies to be
/// used.
///
/// Implementors should aim to provide a stable key between server reboots so
/// as to minimize data loss in client cookies.
///
/// # Implementing
///
/// The `secure_cookies` feature needs to be enabled for this to be implementable
pub trait KeyProvider {
    fn key(&self) -> SecretKey {
        lazy_static! {
            static ref CACHED_SECRET: SecretKey = {
                let mut rng = rand::thread_rng();
                let bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();

                SecretKey::new(bytes).unwrap()
            };
        };

        CACHED_SECRET.clone()
    }
}

#[cfg(feature = "secure_cookies")]
impl KeyProvider for () {}

#[cfg(not(feature = "secure_cookies"))]
impl<T> KeyProvider for T {
    fn key(&self) -> SecretKey {
        SecretKey([0; 32])
    }
}

/// Provides access to a `CookieJar`.
///
/// Access to cookies for a `Request` is read-only and represents the cookies
/// sent from the client.
///
/// The `Response` has access to a mutable `CookieJar` when first accessed.
/// Any cookies added to this jar will be sent as `Set-Cookie` response headers
/// when the `Response` sends it's `Headers` to the client.
///
/// #Examples
/// See `examples/cookies_example.rs`.
pub trait Cookies {
    /// Provides access to an immutable CookieJar.
    ///
    /// Currently requires a mutable reciever, hopefully this can change in future.
    fn cookies(&mut self) -> &CookieJar<'static>;

    /// Provides access to a mutable CookieJar.
    fn cookies_mut(&mut self) -> &mut CookieJar<'static> where Self: AllowMutCookies;
}

impl<'mw, 'conn, D> Cookies for Request<'mw, 'conn, D>
where D: KeyProvider {
    fn cookies(&mut self) -> &<CookiePlugin as Key>::Value {
        self.get_ref::<CookiePlugin>().unwrap()
    }

    fn cookies_mut(&mut self) -> &mut <CookiePlugin as Key>::Value where Self: AllowMutCookies {
        unreachable!()
    }
}

impl<'mw, D> Cookies for Response<'mw, D>
where D: KeyProvider {
    fn cookies(&mut self) -> &<CookiePlugin as Key>::Value {
        self.get_ref::<CookiePlugin>().unwrap()
    }

    fn cookies_mut(&mut self) -> &mut <CookiePlugin as Key>::Value where Self: AllowMutCookies {
        self.get_mut::<CookiePlugin>().unwrap()
    }
}
