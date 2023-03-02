# tokio-sunspec

A pure [Rust](https://www.rust-lang.org)
[SunSpec](https://en.wikipedia.org/wiki/SunSpec) library
based on [tokio](https://tokio.rs).

[![Crates.io](https://img.shields.io/crates/v/tokio-sunspec.svg)](https://crates.io/crates/tokio-sunspec)
[![Apache 2.0 licensed](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](./LICENSE)

## Features

- [x] Pure Rust library
- [x] Non-blocking
- [x] TCP Client
- [x] Model discovery
- [x] Type safe SunSpec models
- [ ] Repeating models
- [ ] RTU Client

## Installation

```toml
[dependencies]
tokio-sunspec = "*"
```

## Examples

The `src/models/` folder contains all pre generated models which may be available by the device. If you need to add your own model you can generate one with included Python script in the `scripts` folder.

### TCP Connect example for SMA Inverter

```rust
let socket_addr = "<ip-address>:502".parse().unwrap();
let device_id: u8 = 126;
let start_addr: u16 = 40000;

let mut client = tokio_sunspec::connect_tcp(socket_addr, device_id, start_addr).await?;

let res = client.read_point(model1::Mn).await?;
assert_eq!(res, "SMA");
```

## Protocol-Specification

- [SunSpec Information Model Specification (PDF)](https://sunspec.org/wp-content/uploads/2015/06/SunSpec-Information-Models-12041.pdf)

## Fruther notice

This lib only implements the SunSpec part. The connection via modbus is done by the [tokio-modbus](https://github.com/slowtec/tokio-modbus) lib.

## License

Copyright (c) 2023 Lukas Kirner

[Apache-2.0](LICENSE.md)
