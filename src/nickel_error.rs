use std::str::SendStr;

#[deriving(Show)]
pub struct NickelError {
    kind: NickelErrorKind,
    message: SendStr
}

impl NickelError {
    pub fn new<T: IntoMaybeOwned<'static>>(message: T, kind: NickelErrorKind) -> NickelError {
        NickelError {
            message: message.into_maybe_owned(),
            kind: kind
        }
    }
}

// Not quite sure yet how we should use this properly.
// I think there would be a popular demand for custom error types
// defined by apps using nickel. How would that work?

#[deriving(Show)]
pub enum NickelErrorKind {
    Other
}