[![crates.io version badge](https://img.shields.io/crates/v/gm_hsm_skf?label=gm_hsm_skf)](https://crates.io/crates/gm-hsm-skf)
[![Documentation](https://docs.rs/gm-hsm-skf/badge.svg)](https://docs.rs/gm-hsm-skf)

Rust wrapper for GM/T 0016-2012(Smart token cryptography application interface specification).

![gm-key](doc/img/gm-key.jpg)

# Usage

Listing available device:

```rust
use gm_hsm_skf::{Engine, LibLoader};

fn main() {
    let engine = Engine::new(LibLoader::env_lookup().unwrap());
    let manager = engine.device_manager().unwrap();
    let list = manager.enum_device(true).unwrap();
    list.iter().for_each(|name| println!("{}", name));
}

```

# Examples

There are several included examples, which help demonstrate the functionality of this library and
can help debug software or hardware errors.

# Native Dependencies

To run the examples (or your application build on this library),The vendor library must be installed.The `LibLoader`
load the library dynamically.

# Resources

- [libloading](https://docs.rs/libloading/latest/libloading/)

## Special Thanks

- [JetBrains Developer Toolbox](https://www.jetbrains.com/?from=power4j)