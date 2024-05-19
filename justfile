#!/usr/bin/env just --justfile





# 增加crates下面所有仓库的cargo pushlish功能Here's the rest of the code to add a `publish` target that will publish all crates in the `crates` directory:


release:
  cargo build --release

lint:
  cargo clippy

bin:
  cargo run --bin bin -- arg1

example:
  cargo run --example exname -- arg1

# Publish all crates in the 'crates' directory
publish:
  @mkdir -p crates
  @for crate in $(ls crates); do \
      echo "Publishing $(cargo pkgid crates/$$crate)"; \
      (cd crates/$$crate && cargo publish --crates-io ); \
  done


wpublish:
  @for /D %%crate in (crates\*) do (
      cd %%crate
      cargo publish --crates-io
      cd ..\
  )