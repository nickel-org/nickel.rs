// HACK: Need cargo support to run `#[test]`s witin examples, this imitates it.

#![allow(dead_code)]
#![allow(unused_attributes)]
include!("../../examples/integration_testing.rs");
