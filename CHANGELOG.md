# 1.0.2

- Fix: reject empty and non-numeric input in `valid()` instead of accepting them as valid (#7, binggao1230@).
- Fix: remove undocumented `println!` stdout side effect on non-digit characters in validation (#7, binggao1230@).
- Perf: make `valid()` zero-allocation by traversing digits right-to-left via `DoubleEndedIterator` without intermediate allocations (#8).
- Overflow safety: reduce running sum modulo 10 during iteration to prevent integer overflow on arbitrarily long inputs (#8).

# 1.0.1

- Bump minor version to sync README.md with crates.io.

# 1.0.0

## Breaking changes

- Rename `luhn::luhn_valid` to `luhn::valid` (asayers@)

## New features

- Add `luhn::checksum` method (asayers@)
