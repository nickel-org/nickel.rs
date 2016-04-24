# Contributing

## Before you begin

Make sure to follow this [commit message convention](https://github.com/conventional-changelog/conventional-changelog-angular/blob/master/convention.md) because we will auto generate a changelog with [clog](https://github.com/thoughtram/clog) in the future.

# Getting started

The easiest way to get started is to get nickel running and play around with it. Let's do that real quick!

##Clone the repository

```shell
git clone https://github.com/nickel-org/nickel.git
```

##Build nickel

```shell
cargo build --release
```

##Run the tests

```shell
cargo test
```

##Run the example

```shell
cargo run --example example
```

Then try `localhost:6767/user/4711` and `localhost:6767/bar`.
