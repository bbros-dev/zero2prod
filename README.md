# Zero-2-production

Zero 2 Production: Newsletter project


[![codecov](https://codecov.io/gh/bbros-dev/zero2prod/branch/main/graph/badge.svg?token=Z6OG67ZIVZ)](https://codecov.io/gh/bbros-dev/zero2prod)

## Git Hooks

To setup git hooks once all lefthook scripts are in place:

    lefthook install

## CI/CD

## Quality and Standards

### Code Coverage

We use [`tarpaulin`](https://github.com/xd009642/tarpaulin#travis-ci-and-coverage-sites).

    // Only supports x86_64 architectures, circa 2020
    cargo install cargo-tarpaulin

### Linting

The `clippy` tool is builtin to the rustup `default` profile, but NOT the
`minimal` profile. Some CI environments use the minimal profile, if so:

    rustup component add clippy

This is a no-op when `clippy` is present.
To fail lint checking on any warning:

    cargo clippy -- -D warnings

### Formatting

If missing:

    rustup component add rustfmt

To ensure consistent formatting, fail on unformatted code:

    cargo fmt -- --check

### Security Vulnerabilities

#### Cargo Audit

    cargo install cargo-audit
    cargo audit

#### Cargo Deny

Lint your project's dependency graph to ensure dependencies conform to
expectations and requirements.

To setup:

    cargo install --locked cargo-deny
    cargo deny init

To check crates:

    cargo deny check
