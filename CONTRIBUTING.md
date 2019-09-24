# Deployment checklist

- Add new entry to `CHANGELOG.md`
- Update version in `Cargo.toml`
- Update version in `README.md`
- `git add .`
- `git commit -m "Version 0.0.0"`
- `git tag -m "0.0.0" 0.0.0`
- `git push`
- `git push --tags`
- `cargo publish --dry-run`
- `cargo publish`
