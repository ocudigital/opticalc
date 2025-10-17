# opticalc

[![Crates.io](https://img.shields.io/crates/v/opticalc.svg)](https://crates.io/crates/opticalc)
[![npm](https://img.shields.io/npm/v/opticalc.svg)](https://www.npmjs.com/package/opticalc)
[![docs.rs](https://docs.rs/opticalc/badge.svg)](https://docs.rs/opticalc)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![WebAssembly](https://img.shields.io/badge/WebAssembly-enabled-purple.svg)](https://webassembly.org)

JavaScript/TypeScript utilities for common clinical optics calculations used in optometry and ophthalmic settings.

## Installation

```bash
npm install opticalc
```

## Features

- **Index conversion**: Convert measured lens powers between assumed and actual refractive indices
- **Rx conversion**: Scale full sphero‑cyl prescriptions between indices while preserving axis
- **Lensmeter simulation**: Predict what a lensmeter would read for a true Rx at a different index
- **Induced prism**: Compute horizontal/vertical prism from decentration using the full power matrix
- **Crossed cylinders** and **oblique meridian** helpers

## Usage

### Basic Power Conversion

```javascript
import { convertPower } from 'opticalc';

// Convert a measured power from 1.523 to 1.586
const truePower = convertPower(-4.463, 1.523, 1.586);
console.log(truePower); // -4.25
```

### TypeScript

```typescript
import { convertPower, SpheroCyl, convertRx, simulateLensmeterReading } from 'opticalc';

// Convert a full sphero‑cyl Rx between indices
const measured: SpheroCyl = { 
  sphere: -2.00, 
  cylinder: -1.00, 
  axisDeg: 180.0 
};

const trueRx = convertRx(measured, 1.523, 1.586);
console.log(trueRx);

// Simulate what a lensmeter @1.523 would read for a true Rx @1.586
const reading = simulateLensmeterReading(trueRx, 1.523, 1.586);
console.log(reading);
```

### Induced Prism from Decentration

```javascript
import { inducedPrism, Eye } from 'opticalc';

const lens = { sphere: 2.0, cylinder: -1.0, axisDeg: 25.0 };
const decentration = { horizontalMm: 2.0, verticalMm: -1.0 };
const prism = inducedPrism(Eye.OD, lens, decentration);

// Access signed or magnitude/base components
const hSigned = prism.horizontal.signed();
const vSigned = prism.vertical.signed();
const magnitude = prism.magnitude();
```

### Common.js

```javascript
const { convertPower, convertRx } = require('opticalc');

const power = convertPower(-4.463, 1.523, 1.586);
console.log(power);
```

## API Reference

### Functions

- `convertPower(measuredPower, fromIndex, toIndex)` - Convert lens power between refractive indices
- `convertRx(spheroCyl, fromIndex, toIndex)` - Convert full prescription between indices
- `simulateLensmeterReading(trueRx, lensmeterIndex, trueIndex)` - Simulate lensmeter reading
- `inducedPrism(eye, lens, decentration)` - Calculate induced prism from decentration

### Types

- `SpheroCyl` - Sphero-cylindrical prescription
- `Decentration` - Horizontal and vertical decentration in mm
- `Eye` - Enum for OD (right) and OS (left) eyes
- `Prism` - Prism power with horizontal/vertical components

## Browser Support

This package uses WebAssembly and requires a modern browser with WebAssembly support:
- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 16+

## License

MIT
