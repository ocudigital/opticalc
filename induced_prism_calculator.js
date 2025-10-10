/**
 * Induced Prism Calculator
 * Calculates induced prism from lens powers and decentration
 */

function calculateInducedPrism(eye, spherePower, cylinderPower, cylinderAxis, decentrationUp, decentrationIn) {
  // Convert degrees to radians
  const toRadians = (degrees) => degrees * Math.PI / 180;
  
  // Calculate power components at the principal meridians
  const sinAxis = Math.sin(toRadians(cylinderAxis));
  const cosAxis = Math.cos(toRadians(cylinderAxis));
  
  // Power at 180° (horizontal meridian)
  const Px = spherePower + (cylinderPower * sinAxis * sinAxis);
  
  // Toric power component (45° oblique)
  const Pt = -cylinderPower * sinAxis * cosAxis;
  
  // Power at 90° (vertical meridian)
  const Py = spherePower + (cylinderPower * cosAxis * cosAxis);
  
  // Calculate induced prism
  // For OD: decentration in is positive nasally
  // For OS: decentration in is negative nasally (so we negate it)
  const decentrationInAdjusted = (eye === 'OS') ? -decentrationIn : decentrationIn;
  
  // Horizontal prism (in prism diopters)
  const horizontalPrismValue = (Px * -decentrationInAdjusted / 10) + (Pt * -decentrationUp / 10);
  const horizontalPrism = Math.abs(parseFloat(horizontalPrismValue.toFixed(3)));
  
  // Vertical prism (in prism diopters)
  const verticalPrismValue = (-Pt * decentrationInAdjusted / 10) + (-Py * decentrationUp / 10);
  const verticalPrism = Math.abs(parseFloat(verticalPrismValue.toFixed(3)));
  
  // Determine base directions
  let horizontalBase;
  if (eye === 'OD') {
    horizontalBase = horizontalPrismValue < 0 ? 'Base In' : 'Base Out';
  } else {
    horizontalBase = horizontalPrismValue < 0 ? 'Base Out' : 'Base In';
  }
  
  const verticalBase = verticalPrismValue < 0 ? 'Base Up' : 'Base Down';
  
  return {
    eye: eye,
    powerComponents: {
      Px: parseFloat(Px.toFixed(3)),
      Pt: parseFloat(Pt.toFixed(3)),
      Py: parseFloat(Py.toFixed(3))
    },
    horizontalPrism: horizontalPrism,
    horizontalBase: horizontalBase,
    verticalPrism: verticalPrism,
    verticalBase: verticalBase
  };
}

/**
 * Example usage:
 */
function example() {
  // Right eye (OD) example
  const odResult = calculateInducedPrism(
    'OD',
    2.00,   // Sphere power (diopters)
    -1.00,  // Cylinder power (diopters)
    26,     // Cylinder axis (degrees)
    3,      // Decentration up (mm)
    1       // Decentration in (mm)
  );
  
  console.log('OD Results:', odResult);
  // Output format:
  // {
  //   eye: 'OD',
  //   powerComponents: { Px: ..., Pt: ..., Py: ... },
  //   horizontalPrism: 0.123,
  //   horizontalBase: 'Base In',
  //   verticalPrism: 0.456,
  //   verticalBase: 'Base Down'
  // }
  
  // Left eye (OS) example
  const osResult = calculateInducedPrism(
    'OS',
    2.00,   // Sphere power (diopters)
    -1.00,  // Cylinder power (diopters)
    26,     // Cylinder axis (degrees)
    3,      // Decentration up (mm)
    1       // Decentration in (mm)
  );
  
  console.log('OS Results:', osResult);
}

// For Node.js environments
if (typeof module !== 'undefined' && module.exports) {
  module.exports = { calculateInducedPrism };
}

