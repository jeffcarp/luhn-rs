# luhn-rs

<a href="https://crates.io/crates/luhn"><img src="https://img.shields.io/crates/v/luhn.svg" /></a>
<a href="https://travis-ci.org/jeffcarp/luhn-rs"><img src="https://api.travis-ci.org/jeffcarp/luhn-rs.svg" /></a>

Validates strings and computes check digits using the Luhn algorithm.

## Usage

Add `luhn` under `[dependencies]` in your `Cargo.toml`:

```toml
[dependencies]
luhn = "1.0.1"
```

Use the validator!

```rust
luhn::valid("4111111111111111"); // true
```

Append check digits to your strings and make them Luhn-valid!

```rust
// A string which doesn't validate
let mut s = "11111111".to_string();
assert!(!valid(&s));

// Let's fix that
s.push(luhn::checksum(s.as_bytes()) as char);
assert_eq!(s, "111111118");
assert!(valid(&s));
```
