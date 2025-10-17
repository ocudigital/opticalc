//! Lens material constants and refractive indices.
//!
//! This module provides constants for common lens materials used in optometry
//! and ophthalmology, including their refractive indices at standard wavelengths.
//!
//! ## Refractive Indices
//!
//! All refractive indices are measured at the sodium D-line (589.3 nm) at 20°C,
//! which is the standard reference wavelength in optical calculations.
//!
//! ## Usage
//!
//! ```rust
//! use opticalc::*;
//! 
//! // Convert power from CR-39 to Polycarbonate
//! let measured_power = -4.50; // measured on CR-39 lensmeter
//! let true_power = convert_power(measured_power, CR_39_INDEX, POLYCARBONATE_INDEX);
//! 
//! // Use in Rx conversion
//! let measured_rx = SpheroCyl { sphere: -2.00, cylinder: -1.00, axis_deg: 90.0 };
//! let true_rx = convert_rx(measured_rx, CR_39_INDEX, TRIVEX_INDEX);
//! ```

/// Refractive index of CR-39 (Columbia Resin #39).
/// 
/// CR-39 is a common plastic lens material with good optical properties.
/// It's lightweight, impact-resistant, and has excellent optical clarity.
/// 
/// **Refractive Index:** 1.498 at 589.3 nm (sodium D-line)
pub const CR_39_INDEX: f64 = 1.498;

/// Refractive index of Trivex.
/// 
/// Trivex is a premium lens material known for its exceptional impact resistance
/// and optical quality. It's lighter than polycarbonate and has excellent
/// stress resistance and optical clarity.
/// 
/// **Refractive Index:** 1.532 at 589.3 nm (sodium D-line)
pub const TRIVEX_INDEX: f64 = 1.532;

/// Refractive index of Polycarbonate.
/// 
/// Polycarbonate is a highly impact-resistant lens material commonly used
/// for safety glasses and children's eyewear. It has good optical properties
/// and is lightweight.
/// 
/// **Refractive Index:** 1.586 at 589.3 nm (sodium D-line)
pub const POLYCARBONATE_INDEX: f64 = 1.586;

/// Refractive index of Crown Glass (standard optical glass).
/// 
/// Crown glass is the traditional lens material and is still used in
/// some high-quality optical applications. It has excellent optical properties
/// but is heavier and more fragile than plastic materials.
/// 
/// **Refractive Index:** 1.523 at 589.3 nm (sodium D-line)
pub const CROWN_GLASS_INDEX: f64 = 1.523;

/// Refractive index of High-Index 1.60 plastic.
/// 
/// High-index materials allow for thinner lenses at higher prescriptions.
/// The 1.60 index is commonly used for moderate to high prescriptions
/// where lens thickness is a concern.
/// 
/// **Refractive Index:** 1.600 at 589.3 nm (sodium D-line)
pub const HIGH_INDEX_160_INDEX: f64 = 1.600;

/// Refractive index of High-Index 1.67 plastic.
/// 
/// High-index 1.67 material provides even thinner lenses for high
/// prescriptions while maintaining good optical quality.
/// 
/// **Refractive Index:** 1.670 at 589.3 nm (sodium D-line)
pub const HIGH_INDEX_167_INDEX: f64 = 1.670;

/// Refractive index of High-Index 1.74 plastic.
/// 
/// High-index 1.74 is one of the highest index plastic materials available,
/// providing the thinnest possible lenses for very high prescriptions.
/// 
/// **Refractive Index:** 1.740 at 589.3 nm (sodium D-line)
pub const HIGH_INDEX_174_INDEX: f64 = 1.740;
