# exprql - Expression Query Language

### Installing

`cargo install wasm-pack pest-language-server`

> dependencies will be installed when you build for a target, since they differ per-target

Currently only WASM build is tested and configured, the crate should technically work as a lib though.

### Cross-compiling for Linux

on macos, do this
`brew tap SergioBenitez/osxct`
`brew install x86_64-unknown-linux-gnu`
`rustup target add x86_64-unknown-linux-gnu`

add this to `~/.cargo/config.toml`

```toml
[target.x86_64-unknown-linux-gnu]
linker = "/opt/homebrew/bin/x86_64-unknown-linux-gnu-gcc"
```

then to compile
`cargo build --target x86_64-unknown-linux-gnu`

### Compiling for WASM

`wasm-pack build --release --target bundler --scope inline-studio`

### Publishing NPM package

Compile for WASM, then navigate to `pkg` subdirectory

`npm publish --access=public`
