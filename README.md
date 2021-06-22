# Dylint updater - updating lints to new toolchain made easy!

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

# Example usage

```
# when new lint added or changed toolchain
cd directory_with_lints_projects_directories
dylints_updater
```

And then

```
# to lint with mega_lint
cd project_to_lint
cargo dylint mega_lint
# or with all lints
cargo dylint --all
```

(more options line `cargo dylint --list`  availabe with dylint help and documentation)

This repository contains example [Dockerfile](Dockerfile) so you can also try with it:

```
docker build .
```

To repeat output/run by yourself last command in dockerfile:

```
# get image hash (or tag it if you prefer)
image_hash="$(docker build .|tail -n 1|awk '{print $NF}')"
# and run lints:
docker run $image_hash bash -c 'cd /substrate_lints/write_and_err/inputs/pseudo_write_and_err_00; cargo dylint write_and_error'
```

to get lintint output for example:

```
warning: check_fn match! (fn_kind: "Fn", node_id: NodeId(268) visitor.found_return=true write=`Some(src/lib.rs:117:3: 117:22 (#0))` err=`Some(src/lib.rs:118:3: 118:28 (#0))`
   --> src/lib.rs:116:1
    |
116 | / pub fn xyz_should_match<T: Config>(origin: OriginFor<T>) -> DispatchResult {
117 | |   XYZ::<T>::put(true);
118 | |   ensure_root::<T>(origin)?;
119 | |   Ok(())
120 | |   // this pattern is wrong because we could both change storage and return an error
121 | | }
    | |_^
    |
    = note: `#[warn(write_and_error)]` on by default
<<... cut ...>>
```


# License

[License](LICENSE)


[Dylint]: https://github.com/trailofbits/dylint
[Substrate]: https://www.substrate.io/
[`DYLINT_LIBRARY_PATH`]: https://github.com/trailofbits/dylint#how-libraries-are-found
