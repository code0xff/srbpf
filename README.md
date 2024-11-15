# srbpf

`srbpf` is a fork of `solana_rbpf`, modified to support `no_std` environments. This project adapts `solana_rbpf` to run in embedded or constrained systems by removing dependencies on the standard library and enabling operation solely through an interpreter.

**Status:** Experimental. Additional verification is necessary before considering `srbpf` for production use.

## Features

- **No Standard Library**: All standard library dependencies have been removed to support `no_std` environments, making `srbpf` suitable for embedded systems or other constrained environments.
- **Interpreter-Only Execution**: `srbpf` has been modified to function entirely through an interpreter, avoiding the JIT compiler. This approach minimizes memory requirements and simplifies the runtime environment.
  
## Getting Started

To use `srbpf` in your `no_std` project, add the following to your `Cargo.toml`:

```toml
[dependencies]
srbpf = { git = "https://github.com/code0xff/srbpf", default-features = false }
```

Ensure that your project is also configured to work in a `no_std` environment, as `srbpf` has been designed specifically for this use case.

## Usage

`srbpf` can be used similarly to `solana_rbpf` but with restrictions in functionality due to the `no_std` environment. Note that it only supports interpreter execution; JIT-based execution is not available.

## Limitations

- **Experimental**: `srbpf` is currently experimental, and its use in production environments requires thorough validation.
- **No JIT Support**: Only the interpreter is available, so performance may vary compared to JIT-compiled versions.
- **Reduced Standard Library Compatibility**: As a `no_std` project, functionality dependent on the Rust standard library has been omitted.

## License

`srbpf` is licensed under either of **Apache License 2.0** or **MIT License** at your option.

- For more details, see the [LICENSE-APACHE](./LICENSE-APACHE) or [LICENSE-MIT](./LICENSE-MIT) files in the repository.
