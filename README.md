Validates strings and computes check digits using the Luhn algorithm.

# luhn-rs

<a href="https://crates.io/crates/luhn"><img src="https://img.shields.io/crates/v/luhn.svg" /></a>
<a href="https://travis-ci.org/jeffcarp/luhn-rs"><img src="https://api.travis-ci.org/jeffcarp/luhn-rs.svg" /></a>

It's not a great checksum, but it's used in a bunch of places (credit
card numbers, ISIN codes, etc.).  More information is available on
[wikipedia](https://en.wikipedia.org/wiki/Luhn_algorithm).

## Usage

Add `luhn` under `[dependencies]` in your `Cargo.toml`:

```toml
[dependencies]
luhn = "1.0.2"
```

Use the validator!

```rust
luhn::validate_slice(b"4111111111111111"); // true
```

Append check digits to your strings and make them Luhn-valid!

```rust
// A string which doesn't validate
let mut s = "11111111".to_string();
assert!(!luhn::valid(&s));

// Let's fix that
s.push(luhn::checksum(s.as_bytes()) as char);
assert_eq!(s, "111111118");
assert!(luhn::valid(&s));
```
