//! Minimum blank size calculation for single vision lenses.
//!
//! ## Overview
//! Calculates the minimum blank size required for cutting a lens based on frame parameters
//! and optical decentration. This is essential for determining the smallest lens blank
//! that can be used while maintaining proper optical centration.
//!
//! ## Formula
//! The minimum blank size is calculated as:
//! ```text
//! Minimum Blank Size = Effective Diameter + (Eyesize + Bridge - IPD)
//! ```
//!
//! Where:
//! - **Effective Diameter**: The effective diameter of the frame (mm)
//! - **Eyesize**: The horizontal width of the lens opening (mm)
//! - **Bridge**: The distance between the two lens openings (mm)
//! - **IPD**: Interpupillary Distance - the distance between the patient's pupils (mm)
//!
//! ## Clinical Notes
//! - It's recommended to add 2mm to the minimum blank size (providing 1mm working edge border)
//! - This accounts for edge imperfections and manufacturing tolerances
//! - All measurements are in millimeters
//! - This calculation assumes standard single vision lens requirements


/// Calculate the minimum blank size for a single vision lens.
///
/// This function computes the smallest lens blank size required based on frame
/// parameters and the patient's interpupillary distance.
///
///
/// N.B. For manufacturing purposes, consider using [`recommended_blank_size`] which adds
/// 2mm to the minimum blank size (providing 1mm working edge border) to account for 
/// edge imperfections and tolerances.
///
/// # Parameters
/// - `effective_diameter_mm`: The effective diameter of the frame in millimeters
/// - `eyesize_mm`: The horizontal width of the lens opening in millimeters
/// - `bridge_mm`: The distance between the two lens openings in millimeters
/// - `ipd_mm`: The patient's interpupillary distance in millimeters
///
/// # Returns
/// The minimum blank size in millimeters.
///
/// # Examples
///
/// ```
/// use opticalc::minimum_blank_size;
/// 
/// // Standard frame with 55mm effective diameter, 50mm eyesize, 15mm bridge, 53mm IPD
/// let min_size = minimum_blank_size(55.0, 50.0, 15.0, 53.0);
/// assert_eq!(min_size, 67.0); // 55 + (50 + 15 - 53) = 55 + 12 = 67
/// ```
///
/// # Panics
/// This function will panic in debug builds if any parameter is negative.
pub fn minimum_blank_size(
    effective_diameter_mm: f64,
    eyesize_mm: f64,
    bridge_mm: f64,
    ipd_mm: f64,
) -> f64 {
    debug_assert!(effective_diameter_mm >= 0.0, "effective_diameter_mm must be non-negative");
    debug_assert!(eyesize_mm >= 0.0, "eyesize_mm must be non-negative");
    debug_assert!(bridge_mm >= 0.0, "bridge_mm must be non-negative");
    debug_assert!(ipd_mm >= 0.0, "ipd_mm must be non-negative");

    effective_diameter_mm + (eyesize_mm + bridge_mm - ipd_mm)
}

/// Calculate the recommended blank size including working edge border.
///
/// This function adds 2mm to the minimum blank size, which provides a 1mm working edge
/// border around the entire lens. This is the standard industry recommendation for
/// lens manufacturing to account for edge imperfections and manufacturing tolerances.
///
/// # Parameters
/// - `effective_diameter_mm`: The effective diameter of the frame in millimeters
/// - `eyesize_mm`: The horizontal width of the lens opening in millimeters
/// - `bridge_mm`: The distance between the two lens openings in millimeters
/// - `ipd_mm`: The patient's interpupillary distance in millimeters
///
/// # Returns
/// The recommended blank size in millimeters (minimum + 2mm total, providing 1mm working edge border).
///
/// # Examples
///
/// ```
/// use opticalc::recommended_blank_size;
/// 
/// // Standard frame with working edge border
/// let recommended_size = recommended_blank_size(55.0, 50.0, 15.0, 53.0);
/// assert_eq!(recommended_size, 69.0); // 67 + 2 = 69
/// ```
pub fn recommended_blank_size(
    effective_diameter_mm: f64,
    eyesize_mm: f64,
    bridge_mm: f64,
    ipd_mm: f64,
) -> f64 {
    minimum_blank_size(effective_diameter_mm, eyesize_mm, bridge_mm, ipd_mm) + 2.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    const EPS: f64 = 1e-9;

    #[test]
    fn minimum_blank_size_standard_example() {
        // Standard frame: 55mm effective diameter, 50mm eyesize, 15mm bridge, 53mm IPD
        let result = minimum_blank_size(55.0, 50.0, 15.0, 53.0);
        assert_abs_diff_eq!(result, 67.0, epsilon = EPS);
    }

    #[test]
    fn minimum_blank_size_formula_verification() {
        // Test the formula: Effective Diameter + (Eyesize + Bridge - IPD)
        let effective_diameter = 60.0;
        let eyesize = 48.0;
        let bridge = 18.0;
        let ipd = 62.0;
        
        let result = minimum_blank_size(effective_diameter, eyesize, bridge, ipd);
        let expected = effective_diameter + (eyesize + bridge - ipd);
        
        assert_abs_diff_eq!(result, expected, epsilon = EPS);
        assert_abs_diff_eq!(result, 64.0, epsilon = EPS); // 60 + (48 + 18 - 62) = 64
    }

    #[test]
    fn minimum_blank_size_large_frame() {
        // Large frame example
        let result = minimum_blank_size(70.0, 60.0, 20.0, 65.0);
        assert_abs_diff_eq!(result, 85.0, epsilon = EPS); // 70 + (60 + 20 - 65) = 85
    }

    #[test]
    fn minimum_blank_size_small_frame() {
        // Small frame example
        let result = minimum_blank_size(45.0, 40.0, 12.0, 50.0);
        assert_abs_diff_eq!(result, 47.0, epsilon = EPS); // 45 + (40 + 12 - 50) = 47
    }

    #[test]
    fn minimum_blank_size_zero_components() {
        // Edge case: zero effective diameter
        let result = minimum_blank_size(0.0, 50.0, 15.0, 53.0);
        assert_abs_diff_eq!(result, 12.0, epsilon = EPS); // 0 + (50 + 15 - 53) = 12
    }

    #[test]
    fn minimum_blank_size_negative_decentration() {
        // Case where IPD is larger than eyesize + bridge (negative decentration)
        let result = minimum_blank_size(55.0, 45.0, 10.0, 60.0);
        assert_abs_diff_eq!(result, 50.0, epsilon = EPS); // 55 + (45 + 10 - 60) = 50
    }

    #[test]
    fn recommended_blank_size_adds_working_edge() {
        let min_size = minimum_blank_size(55.0, 50.0, 15.0, 53.0);
        let recommended = recommended_blank_size(55.0, 50.0, 15.0, 53.0);
        
        assert_abs_diff_eq!(recommended, min_size + 2.0, epsilon = EPS);
        assert_abs_diff_eq!(recommended, 69.0, epsilon = EPS);
    }

    #[test]
    fn recommended_blank_size_consistency() {
        // Test that recommended_blank_size gives same result as manual calculation
        let effective_diameter = 60.0;
        let eyesize = 48.0;
        let bridge = 18.0;
        let ipd = 62.0;
        
        let recommended = recommended_blank_size(effective_diameter, eyesize, bridge, ipd);
        let manual = minimum_blank_size(effective_diameter, eyesize, bridge, ipd) + 2.0;
        
        assert_abs_diff_eq!(recommended, manual, epsilon = EPS);
    }

    #[test]
    fn minimum_blank_size_precision() {
        // Test with fractional values to ensure precision
        let result = minimum_blank_size(55.5, 50.25, 15.75, 53.5);
        let expected = 55.5 + (50.25 + 15.75 - 53.5);
        assert_abs_diff_eq!(result, expected, epsilon = EPS);
        assert_abs_diff_eq!(result, 68.0, epsilon = EPS);
    }

    #[test]
    #[should_panic(expected = "effective_diameter_mm must be non-negative")]
    fn minimum_blank_size_panics_negative_effective_diameter() {
        let _ = minimum_blank_size(-1.0, 50.0, 15.0, 53.0);
    }

    #[test]
    #[should_panic(expected = "eyesize_mm must be non-negative")]
    fn minimum_blank_size_panics_negative_eyesize() {
        let _ = minimum_blank_size(55.0, -1.0, 15.0, 53.0);
    }

    #[test]
    #[should_panic(expected = "bridge_mm must be non-negative")]
    fn minimum_blank_size_panics_negative_bridge() {
        let _ = minimum_blank_size(55.0, 50.0, -1.0, 53.0);
    }

    #[test]
    #[should_panic(expected = "ipd_mm must be non-negative")]
    fn minimum_blank_size_panics_negative_ipd() {
        let _ = minimum_blank_size(55.0, 50.0, 15.0, -1.0);
    }

    #[test]
    fn minimum_blank_size_edge_case_zero_ipd() {
        // Edge case: zero IPD (theoretical)
        let result = minimum_blank_size(55.0, 50.0, 15.0, 0.0);
        assert_abs_diff_eq!(result, 120.0, epsilon = EPS); // 55 + (50 + 15 - 0) = 120
    }

    #[test]
    fn minimum_blank_size_edge_case_zero_eyesize_and_bridge() {
        // Edge case: zero eyesize and bridge
        let result = minimum_blank_size(55.0, 0.0, 0.0, 53.0);
        assert_abs_diff_eq!(result, 2.0, epsilon = EPS); // 55 + (0 + 0 - 53) = 2
    }
}
