Get this working:

Download and install Rust: https://www.rust-lang.org/en-US/install.html

Create a new project:

```
cargo new --bin hello_komodo
```

This creates a folder called `hello_komodo`. Add the following in `hello_komodo/Cargo.toml`:

```toml
[dependencies]
komodo_rpc_client = "0.0.2"
```

This downloads the needed crate (package) `komodo_rpc_client`, the one used in the examples.

See the examples in this folder for guidance on your first RPC to Komodo using Rust. You can copy these examples straight to `hello_komodo/src/main.rs`.

For now, it's `getinfo` and `getaddresstxids` (needs addressindex enabled) to show a call with and without parameters.
`src/komodo_rpc_api` shows all the currently implemented API calls you can use.

Finally, execute `cargo run` in the `hello_komodo` directory. 