# Dylint updater

[Dylint] require lints :

* to be libraries compiled for given (version of) toolchain.
* and stored under [`DYLINT_LIBRARY_PATH`](https://github.com/trailofbits/dylint#how-libraries-are-found)

This tool is to help with this process.

Just keep repositories with your lints in one directory and run there:

```
dylints_updater rebuild_and_update  # rebuilds lints and updates under `DYLINT_LIBRARY_PATH`
```

this command will:

* rebuild -> with current toolchain
* update -> put compiled libraries under [`DYLINT_LIBRARY_PATH`]

(Tool may ask you to setup [`DYLINT_LIBRARY_PATH`](https://github.com/trailofbits/dylint#how-libraries-are-found))

# Prerequisites

In order to use [Dylint] lints you should have installed:

```
cargo install cargo-dylint dylint-link
```

# Installation

```
cargo install --git https://github.com/gww-parity/dylints_updater.git --branch dylints_updater
```

# Example dylint lint lints

This project is used e.g. with Dylint lints for [Substrate]: [`https://github.com/gww-parity/substrate_lints`](https://github.com/gww-parity/substrate_lints)

# License

[License](LICENSE)


[Dylint]: https://github.com/trailofbits/dylint
[Substrate]: https://www.substrate.io/
[`DYLINT_LIBRARY_PATH`]: https://github.com/trailofbits/dylint#how-libraries-are-found
