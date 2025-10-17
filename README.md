opticalc
============

[![Crates.io](https://img.shields.io/crates/v/opticalc.svg)](https://crates.io/crates/opticalc)
[![npm](https://img.shields.io/npm/v/opticalc.svg)](https://www.npmjs.com/package/opticalc)
[![docs.rs](https://docs.rs/opticalc/badge.svg)](https://docs.rs/opticalc)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![WebAssembly](https://img.shields.io/badge/WebAssembly-enabled-purple.svg)](https://webassembly.org)

> This package is also available for JavaScript/TypeScript via npm. See the [npm README](https://github.com/ocudigital/opticalc/blob/master/README.npm.md) for usage instructions.

Rust utilities for common clinical optics calculations used in optometry and ophthalmic settings.

Features
- **Index conversion**: Convert measured lens powers between assumed and actual refractive indices.
- **Rx conversion**: Scale full sphero‑cyl prescriptions between indices while preserving axis.
- **Lensmeter simulation**: Predict what a lensmeter would read for a true Rx at a different index.
- **Induced prism**: Compute horizontal/vertical prism from decentration using the full power matrix.
- **Crossed cylinders** and **oblique meridian** helpers.

## Examples

```rust
use opticalc::*;

// Convert a measured power from 1.523 to 1.586
let true_power = convert_power(-4.463, 1.523, 1.586);

// Convert a full sphero‑cyl Rx between indices
let measured = SpheroCyl { sphere: -2.00, cylinder: -1.00, axis_deg: 180.0 };
let true_rx = convert_rx(measured, 1.523, 1.586);

// Simulate what a lensmeter @1.523 would read for a true Rx @1.586
let reading = simulate_lensmeter_reading(true_rx, 1.523, 1.586);
```

Induced prism from decentration (Prentice’s rule, full toric matrix):

```rust
use opticalc::*;

let lens = SpheroCyl { sphere: 2.0, cylinder: -1.0, axis_deg: 25.0 };
let dec = Decentration { horizontal_mm: 2.0, vertical_mm: -1.0 };
let p = induced_prism(Eye::OD, lens, dec);

// Access signed or magnitude/base components
let h_signed = p.horizontal.signed();
let v_signed = p.vertical.signed();
let mag = p.magnitude();
```
