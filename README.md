# eiffel

[![Build & Test](https://github.com/matthiasg/eiffel/actions/workflows/build.yml/badge.svg)](https://github.com/matthiasg/eiffel/actions/workflows/build.yml)
[![Build & Test against latest dependencies](https://github.com/matthiasg/eiffel/actions/workflows/check-latest-deps.yml/badge.svg)](https://github.com/matthiasg/eiffel/actions/workflows/check-latest-deps.yml)

> This is a personal test project only but works. A similar crate with direct expressions is
> [contracts](https://docs.rs/contracts/latest/contracts/)
> and of course the original [LibHoare](https://crates.io/crates/hoare).
>
> Please refer to them for production usage.

This crate provides a set of macros inspired by the Eiffel programming language's
features for invariant checking. The Eiffel language's options for invariant checking serve as the basis for the design and functionality of the macros in this crate.

Please note that this crate is still a work in progress. As such, some features may not be fully
implemented or may undergo significant changes in future updates.
Contributions and feedback are always welcome, but the crate alternatives mentioned above are better targets for collaboration I assume.

## Usage ?

The [Eiffel language](www.eiffel.org) is known for very robust design by contract and assertions and I immediately loved that even back in the 90s when I first saw it. 

Since Rust has a much better [Macro Environment](https://doc.rust-lang.org/book/ch19-06-macros.html) than C etc it was immediately obvious we could extend Rust in this way.

A contrived eiffel example:

```eiffel
  set_second (s: INTEGER)
      -- Set the second from `s'.
    require
      valid_argument_for_second: 0 <= s and s <= 59
    do
      second := s
    ensure
      second_set: second = s
    end
```

in rust:

```rust
use eiffel::contract;

struct UnitBetween1And20 {
  val: u8
}

impl UnitBetween1And20 {
  fn new() -> Self {
    Self { val: 1 }
  }

  fn is_valid_val(&self) -> bool {
    self.val >= 1 && self.val <= 20
  }

  #[contract(is_valid_val)]
  pub fn inc(&mut self) {
    self.val += 1;
  }

  #[contract(is_valid_val)]
  pub fn set(&mut self, new_value: u8) {
    self.val = new_value;
  }
}

let mut a = UnitBetween1And20::new();
a.inc();

// Note: Inside the crate we could still do an unchecked
// direct access.
a.val = 20;

// This would fail with a panic! now:
// 
// a.inc();
```

## Todo

This crate needs to more closely align with Eiffel still. `ensure` is missing for example and the requirement for a seperate funciton should 
go away and just allow an in-place expression that returns bool.

It would be great if it could also add additional documentation for the function it marks for the 'contract' to be visible in the docs. Sadly not possible with macros yet it seems.

## Development Workflow

- [Install rust](https://www.rust-lang.org/tools/install)
- Configure stable toolchain
  - `rustup update stable && rustup default stable`
- Build project against local machine
  -  `cargo build --workspace`
- Test
  - `cargo test --workspace`
  - `cargo test --workspace --doc`
- Install cargo watch tool for continuous rebuild etc.
  - `cargo install cargo-watch`
  - `cargo watch --clear -x "build"` or `cargo watch --clear -x "test --workspace --doc"`

[Cargo.toml](Cargo.toml) contains the links to the subcrates which are required as Rust cannot have the different macros live in the same crate per default. That is why we re-expose.

Sadly that mean we have to manually update Cargo.toml dependent versions before publishing to main.

I will try to work with [release-plz](https://release-plz.ieni.dev/) to automate pushing new versions, but I am not happy with it yet.