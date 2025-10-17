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

### Power Conversion

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

### Induced Prism from Decentration

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

### Crossed Cylinders

```rust
use opticalc::*;

// Combine two spherocylindrical lenses
let lens1 = SpheroCyl { sphere: -2.00, cylinder: -1.00, axis_deg: 90.0 };
let lens2 = SpheroCyl { sphere: -1.00, cylinder: -0.50, axis_deg: 180.0 };
let combined = crossed_cylinders(lens1, lens2);
```

### Transposition

```rust
use opticalc::*;

// Transpose from minus to plus cylinder form
let minus_form = SpheroCyl { sphere: -3.50, cylinder: 2.00, axis_deg: 150.0 };
let plus_form = minus_form.transpose();
// Result: -1.50 DS / -2.00 DC × 60

// Or use the function
let transposed = transpose(minus_form);
```

### Oblique Meridian Power

```rust
use opticalc::*;

let lens = SpheroCyl { sphere: -2.0, cylinder: -4.0, axis_deg: 30.0 };
let power_at_45 = oblique_meridian(lens, 45.0);
let power_at_90 = lens.power_at(90.0); // Same as oblique_meridian
```

### Minimum Blank Size

```rust
use opticalc::*;

// Calculate minimum blank size for lens cutting
let min_size = minimum_blank_size(55.0, 50.0, 15.0, 53.0);
// Result: 67.0 mm

// Get recommended size with working edge border
let recommended = recommended_blank_size(55.0, 50.0, 15.0, 53.0);
// Result: 69.0 mm (minimum + 2mm)
```

### Material Constants

```rust
use opticalc::*;

// Use predefined material indices
let measured = -4.50;
let true_power = convert_power(measured, CR_39_INDEX, POLYCARBONATE_INDEX);

// Available constants:
// CR_39_INDEX = 1.498
// TRIVEX_INDEX = 1.532
// CROWN_GLASS_INDEX = 1.523
// POLYCARBONATE_INDEX = 1.586
// HIGH_INDEX_160_INDEX = 1.600
// HIGH_INDEX_167_INDEX = 1.670
// HIGH_INDEX_174_INDEX = 1.740
```
