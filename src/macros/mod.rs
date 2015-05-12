#[macro_use] mod middleware;
#[macro_use] mod router;

#[macro_export]
macro_rules! try_with {
    ($res:expr, $exp:expr) => {{
        match $exp {
            ::std::result::Result::Ok(val) => val,
            ::std::result::Result::Err(e) => {
                return Err(From::from(($res, e)))
            }
        }
    }};
}
