# luhn-rs <a href="https://travis-ci.org/jeffcarp/luhn-rs"><img src="https://api.travis-ci.org/jeffcarp/luhn-rs.svg" /></a>

Validates strings and computes check digits using the Luhn algorithm.

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
