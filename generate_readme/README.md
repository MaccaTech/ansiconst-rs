# INTERNAL

For internal use only.

Used to generate `ansiconst`'s `README` file from its Rustdoc as follows:

```shell
# In ansiconst root
cd generate_readme
cargo build
cd -
./generate_readme/target/debug/generate > README.md
```

Depends on crate `cargo-readme`, which (unfortunately) creates broken links in some cases.
Therefore this crate manually repairs the broken links when generating the `README`
for `ansiconst`.
