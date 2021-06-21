#[allow(unused_imports)]
#[macro_use]
extern crate derive_new;

use dylints_updater::*;
use std::env;

fn config_from_env_and_args() -> anyhow::Result<DylintsUpdaterConfig> {
    let dpath = env::var("DYLINT_LIBRARY_PATH").expect("
Please set DYLINT_LIBRARY_PATH environment variable.
E.g. if you are using bash, you may like to add `export DYLINT_LIBRARY_PATH=...` to your `~/.bashrc`.
Good location could be e.g. ~/.cache/dylint/lints. 
For proposed configuration you may like to run:
```
mkdir -p ~/.cache/dylint/lints
echo 'export DYLINT_LIBRARY_PATH=~/.cache/dylint/lints' >> ~/.bashrc
source ~/.bashrc # source bashrc to load into current shell
```
");
    let c = DylintsUpdaterConfig::new(dpath);
    Ok(c)
}

fn main() -> anyhow::Result<()> {
    let config = config_from_env_and_args()?;
    dylints_updater::run(&config)
}
