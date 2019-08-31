# luhn-rs <a href="https://travis-ci.org/jeffcarp/luhn-rs"><img src="https://api.travis-ci.org/jeffcarp/luhn-rs.svg" /></a>

A first attempt at making a Luhn validator for Rust

## Usage

Add `luhn` under `[dependencies]` in your `Cargo.toml`:

```toml
[dependencies]
luhn = "0.1.0"
```

Use the validator!

```rust
luhn::valid("4111111111111111"); // true
```
