![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/power4j/gm-hsm/CI.yml)
[![crates.io version badge](https://img.shields.io/crates/v/gm-hsm-sys?label=gm-hsm-sys)](https://crates.io/crates/gm-hsm-sys)
[![crates.io version badge](https://img.shields.io/crates/v/gm-hsm-skf?label=gm-hsm-skf)](https://crates.io/crates/gm-hsm-skf)

> This project is a rewrite of the [skf-rs](https://github.com/power4j/skf-rs) project to support more types of
> cryptographic devices.

- `gm-hsm-sys`: FFI Types for `gm-hsm-skf`,`gm-hsm-sdf`.
- `gm-hsm-skf`: Rust wrapper for `GM/T 0016-2012` (Smart token cryptography application interface specification).
- `gm-hsm-sdf`: Rust wrapper for `GM/T 0018-2012` (Interface specifications of cryptography device application).

## License

This project is licensed under the [MIT license].

[MIT license]: https://github.com/power4j/gm-hsm/blob/master/LICENSE