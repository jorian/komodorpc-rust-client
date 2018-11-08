Get this working:

Download and install Rust: https://www.rust-lang.org/en-US/install.html

Create a new project:

```
cargo new --bin hello_komodo
```

Add the following in Cargo.toml:

```toml
[dependencies]
komodo_rpc_client = "0.0.1"
```

See the examples in this folder for guidance on your first RPC to Komodo using Rust.

For now, it's `getinfo` and `getaddresstxids` (needs addressindex enabled) to show a call with and without parameters.
`komodo_rpc_api` shows all the currently implemented API calls you can use.

