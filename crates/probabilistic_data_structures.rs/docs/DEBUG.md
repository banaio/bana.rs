# `DEBUG`

![WORK-IN-PROGRESS](https://img.shields.io/badge/DEBUG-WORK--IN--PROGRESS-red?style=for-the-badge&logo=markdown&maxAge=604800&cacheSeconds=604800)

---

## Debug

Run one of these shell commands then read [`Run Command`](#run-command) section:

* **Turn on**:

```sh
export CARGO_TERM_VERBOSE=1 CARGO_TERM_COLOR="always" VERBOSE="--verbose"
make run -- --log_level trace
```

* **Turn off**:

```sh
export  CARGO_TERM_VERBOSE=0 CARGO_TERM_COLOR="auto" VERBOSE=""
make run -- --log_level trace
```
