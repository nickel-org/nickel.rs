use {Request, Response};
use plugin::{Plugin, Pluggable, Extensible};
use typemap::Key;
use hyper::header;

use cookie::CookieJar;

pub struct SecretKey(pub [u8; 32]);

// Plugin boilerplate
struct CookiePlugin;
impl Key for CookiePlugin { type Value = CookieJar<'static>; }

impl<'a, 'b, 'k, D> Plugin<Request<'a, 'b, 'k, D>> for CookiePlugin
        where D: AsRef<SecretKey> {
    type Error = ();

    fn eval(req: &mut Request<D>) -> Result<CookieJar<'static>, ()> {
        let key = req.data().as_ref();
        let jar = match req.origin.headers.get::<header::Cookie>() {
            Some(c) => c.to_cookie_jar(&key.0),
            None => CookieJar::new(&key.0)
        };

        Ok(jar)
    }
}

impl<'a, 'b, 'k, D> Plugin<Response<'a, D>> for CookiePlugin
        where D: AsRef<SecretKey> {
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

        let key = res.data().as_ref();
        Ok(CookieJar::new(&key.0))
    }
}

/// Trait to whitelist access to `&'mut CookieJar` via the `Cookies` trait.
pub trait AllowMutCookies {}
impl<'a, D> AllowMutCookies for Response<'a, D> {}

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
pub trait Cookies : Pluggable + Extensible
        where CookiePlugin: Plugin<Self, Value=CookieJar<'static>, Error=()> {
    /// Provides access to an immutable CookieJar.
    ///
    /// Currently requires a mutable reciever, hopefully this can change in future.
    fn cookies(&mut self) -> &CookieJar {
        self.get_ref::<CookiePlugin>().unwrap()
    }

    /// Provides access to a mutable CookieJar.
    fn cookies_mut(&mut self) -> &mut CookieJar<'static> where Self: AllowMutCookies {
        self.get_mut::<CookiePlugin>().unwrap()
    }
}

impl<'a, 'b, 'k, D: AsRef<SecretKey>> Cookies for Request<'a, 'b, 'k, D> {}

impl<'a, D: AsRef<SecretKey>> Cookies for Response<'a, D> {}
