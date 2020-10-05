# `httputil_reverseproxy.rs`

![GitHub Workflows rust_stable_beta_nightly_1_45_2](https://github.com/banaio/httputil_reverseproxy.rs/workflows/rust_stable_beta_nightly_1_45_2/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/httputil_reverseproxy.svg)](https://crates.io/crates/httputil_reverseproxy)
[![Docs.rs](https://docs.rs/httputil_reverseproxy/badge.svg)](https://docs.rs/httputil_reverseproxy)

---

![WORK-IN-PROGRESS](https://img.shields.io/badge/httputil__reverseproxy.rs-WORK--IN--PROGRESS-red?style=for-the-badge&logo=rust&maxAge=604800&cacheSeconds=604800)

`httputil_reverseproxy.rs`: The implementation is based on Go's `net/http/httputil.ReverseProxy`, see [`type ReverseProxy` in `net/http/httputil`](https://golang.org/pkg/net/http/httputil/#ReverseProxy).

## Run

### Usage/Help menu

```sh
cargo run -- --help
```

### Run Command

```sh
make run -- --input '.vscode/settings.json' --out-type 'stdout' --log-level trace
# or
cargo run -- --input '.vscode/settings.json' --out-type 'stdout' --log-level trace
```

```sh
make run_test -- --log-level trace
```

### Test

See [./docs/TESTS.md](./docs/TESTS.md) page.

### Debug

See [./docs/DEBUG.md](./docs/DEBUG.md) page.

## Docs

* [./docs/TESTS.md](./docs/TESTS.md).
* [./docs/DEBUG.md](./docs/DEBUG.md).
* [./docs/TODO.md](./docs/TODO.md).

## Links

* **`Crates.io`:** [https://crates.io/crates/httputil_reverseproxy](https://crates.io/crates/httputil_reverseproxy).
* **`Docs.rs`:** [https://docs.rs/httputil_reverseproxy](https://docs.rs/httputil_reverseproxy).

## References or Links

* [Golang's `type ReverseProxy` in `net/http/httputil`](https://golang.org/pkg/net/http/httputil/#ReverseProxy).
* [Golang's `net/http/httputil` package](https://golang.org/pkg/net/http/httputil).
