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
import { 
  convertPower, 
  SpheroCyl, 
  convertRx, 
  simulateLensmeterReading,
  inducedPrism,
  Eye,
  Decentration,
  crossedCylinders,
  transpose,
  obliqueMeridian,
  minimumBlankSize,
  recommendedBlankSize,
  CR_39_INDEX,
  POLYCARBONATE_INDEX
} from 'opticalc';

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

### Crossed Cylinders

```javascript
import { crossedCylinders } from 'opticalc';

// Combine two spherocylindrical lenses
const lens1 = { sphere: -2.00, cylinder: -1.00, axisDeg: 90.0 };
const lens2 = { sphere: -1.00, cylinder: -0.50, axisDeg: 180.0 };
const combined = crossedCylinders(lens1, lens2);
console.log(combined);
```

### Transposition

```javascript
import { transpose } from 'opticalc';

// Transpose from minus to plus cylinder form
const minusForm = { sphere: -3.50, cylinder: 2.00, axisDeg: 150.0 };
const plusForm = transpose(minusForm);
console.log(plusForm); // { sphere: -1.50, cylinder: -2.00, axisDeg: 60.0 }
```

### Oblique Meridian Power

```javascript
import { obliqueMeridian } from 'opticalc';

const lens = { sphere: -2.0, cylinder: -4.0, axisDeg: 30.0 };
const powerAt45 = obliqueMeridian(lens, 45.0);
console.log(powerAt45);
```

### Minimum Blank Size

```javascript
import { minimumBlankSize, recommendedBlankSize } from 'opticalc';

// Calculate minimum blank size for lens cutting
const minSize = minimumBlankSize(55.0, 50.0, 15.0, 53.0);
console.log(minSize); // 67.0

// Get recommended size with working edge border
const recommended = recommendedBlankSize(55.0, 50.0, 15.0, 53.0);
console.log(recommended); // 69.0 (minimum + 2mm)
```

### Material Constants

```javascript
import { convertPower, CR_39_INDEX, POLYCARBONATE_INDEX } from 'opticalc';

// Use predefined material indices
const measured = -4.50;
const truePower = convertPower(measured, CR_39_INDEX, POLYCARBONATE_INDEX);
console.log(truePower);

// Available constants:
// CR_39_INDEX = 1.498
// TRIVEX_INDEX = 1.532
// CROWN_GLASS_INDEX = 1.523
// POLYCARBONATE_INDEX = 1.586
// HIGH_INDEX_160_INDEX = 1.600
// HIGH_INDEX_167_INDEX = 1.670
// HIGH_INDEX_174_INDEX = 1.740
```

### Common.js

```javascript
const { convertPower, convertRx, inducedPrism, Eye } = require('opticalc');

const power = convertPower(-4.463, 1.523, 1.586);
console.log(power);

const lens = { sphere: 2.0, cylinder: -1.0, axisDeg: 25.0 };
const decentration = { horizontalMm: 2.0, verticalMm: -1.0 };
const prism = inducedPrism(Eye.OD, lens, decentration);
console.log(prism.magnitude());
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
