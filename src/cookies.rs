use Response;
use plugin::{Plugin, Pluggable};
use typemap::Key;
use hyper::header;

use cookie::CookieJar;

pub struct SecretKey(pub [u8; 32]);

// Plugin boilerplate
pub struct CookiePlugin;
impl Key for CookiePlugin { type Value = CookieJar<'static>; }

impl<'a, 'k, D> Plugin<Response<'a, 'k, D>> for CookiePlugin
        where D: AsRef<SecretKey> {
    type Error = ();

    fn eval(res: &mut Response<'a, 'k, D>) -> Result<CookieJar<'static>, ()> {
        // Schedule the cookie to be written when headers are being sent
        res.on_send(|res| {
            let header = {
                let jar = res.get_ref::<CookiePlugin>().unwrap();
                header::SetCookie::from_cookie_jar(jar)
            };
            res.set(header);
        });

        // Initialise the CookieJar from our Request
        let key = res.data().as_ref();
        let jar = match res.request.origin.headers.get::<header::Cookie>() {
            Some(c) => c.to_cookie_jar(&key.0),
            None => CookieJar::new(&key.0)
        };

        Ok(jar)
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
    fn cookies_mut(&mut self) -> &mut CookieJar<'static>;
}

impl<'a, 'k, D: AsRef<SecretKey>> Cookies for Response<'a, 'k, D> {
    fn cookies(&mut self) -> &<CookiePlugin as Key>::Value {
        self.get_ref::<CookiePlugin>().unwrap()
    }

    fn cookies_mut(&mut self) -> &mut <CookiePlugin as Key>::Value {
        self.get_mut::<CookiePlugin>().unwrap()
    }
}
