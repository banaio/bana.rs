# `TODO`

![WORK-IN-PROGRESS](https://img.shields.io/badge/TODO-WORK--IN--PROGRESS-red?style=for-the-badge&logo=markdown&maxAge=604800&cacheSeconds=604800)

---

## TODO

* Read from stdin for records to process instead of file.
* https://docs.rs/env_logger/
* GitHub pages using VuePress? https://vuepress.vuejs.org/guide/deploy.html#github-pages

### https://endler.dev/2020/rust-compile-times/

Remove Unused Dependencies

So let's say you tried all of the above and find that compilation is still slow. What now?

Dependencies sometimes become obsolete thanks to refactoring. From time to time it helps to check if all of them are still needed to save compile time.

If this is your own project (or a project you like to contribute to), do a quick check if you can toss anything with cargo-udeps:

cargo install cargo-udeps && cargo +nightly udeps
