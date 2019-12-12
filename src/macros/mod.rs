#[macro_use] mod middleware;
#[macro_use] mod router;

/// Variant of the `try!` macro which takes ownership of a Response on error.
///
/// See the `IntoError` documentation for usage examples.
#[macro_export]
macro_rules! try_with {
    ($res:expr, $exp:expr) => {{
        match $exp {
            ::std::result::Result::Ok(val) => val,
            ::std::result::Result::Err(e) => {
                return Err($crate::IntoError::into(e, $res))
            }
        }
    }};
}
