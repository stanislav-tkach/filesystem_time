# filesystem_time

## Overview

Get/set file time.

## Documentation

TODO: docs.rs.

## Usage

To use this crate, add `filesystem_time` as a dependency to your project's `Cargo.toml`:

```toml
[dependencies]
filesystem_time = "0.1"
```

Or, if you want [Chrono](https://github.com/chronotope/chrono) or
[time](https://github.com/rust-lang-deprecated/time) support,
include the features like this:

```toml
[dependencies]
chrono = { version = "0.1", features = ["chrono", "time"] }
```

## Example

```rust,no_run
// TODO, FIXME.
```

## Testing

Tests should run as `cargo.exe test --features "chrono time"`.

## License

Licensed under the MIT license (see the `LICENSE` file).
