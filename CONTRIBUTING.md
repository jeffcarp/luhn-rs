# Contributing & Release Guide

## Development

Run tests, linter, and formatting checks:

```bash
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt -- --check
```

## Release Checklist

1. **Update changelog:** Add release notes under the new version header in `CHANGELOG.md`.
2. **Bump version:** Update the version in `Cargo.toml` and the dependency snippet in `README.md`.
3. **Pre-flight verification:** Ensure all checks and packaging pass before committing or tagging:
   ```bash
   cargo test
   cargo clippy --all-targets -- -D warnings
   cargo fmt -- --check
   cargo publish --dry-run --allow-dirty
   ```
4. **Commit and tag:**
   ```bash
   git commit -am "Version X.Y.Z"
   git tag -a vX.Y.Z -m "Release vX.Y.Z"
   ```
5. **Push:**
   ```bash
   git push origin master --follow-tags
   ```
6. **Publish to crates.io:**
   ```bash
   cargo publish
   ```
7. **Create GitHub Release:**
   ```bash
   gh release create vX.Y.Z --title "vX.Y.Z" --notes-from-tag
   ```
