//! Transposition of spherocylindrical lens prescriptions.
//!
//! ## Overview
//! Converts a spherocylindrical lens prescription between plus and minus cylinder forms.
//! This is essential in optometry for standardizing prescriptions and ensuring
//! compatibility with different lens manufacturing systems.
//!
//! ## Transposition Rules
//! To convert from minus cylinder to plus cylinder form (or vice versa):
//! 1. **New Sphere**: Original sphere + Original cylinder
//! 2. **New Cylinder**: Negate the original cylinder (change sign)
//! 3. **New Axis**: Add 90° if original axis < 90°, subtract 90° if original axis ≥ 90°
//!
//! ## Clinical Notes
//! - Both forms represent the same optical power
//! - Plus cylinder form is often preferred for manufacturing
//! - Minus cylinder form is commonly used in clinical practice
//! - The axis adjustment ensures the cylinder power is applied to the correct meridian

use crate::*;

impl SpheroCyl {
    /// Transpose this lens prescription to the opposite cylinder form.
    ///
    /// Converts between plus and minus cylinder notation while maintaining
    /// the same optical power. The axis is adjusted by ±90° to ensure
    /// the cylinder power is applied to the correct meridian.
    ///
    /// # Returns
    /// A new `SpheroCyl` with the transposed prescription.
    ///
    /// # Examples
    ///
    /// ```
    /// use opticalc::*;
    /// 
    /// // Minus cylinder form: -3.50 DS / +2.00 DC × 150
    /// let minus_form = SpheroCyl {
    ///     sphere: -3.50,
    ///     cylinder: 2.00,
    ///     axis_deg: 150.0,
    /// };
    /// 
    /// // Transpose to plus cylinder form: -1.50 DS / -2.00 DC × 60
    /// let plus_form = minus_form.transpose();
    /// assert_eq!(plus_form.sphere, -1.50);
    /// assert_eq!(plus_form.cylinder, -2.00);
    /// assert_eq!(plus_form.axis_deg, 60.0);
    /// ```
    pub fn transpose(self) -> Self {
        let new_sphere = self.sphere + self.cylinder;
        let new_cylinder = -self.cylinder;
        let new_axis = if self.axis_deg >= 90.0 {
            self.axis_deg - 90.0
        } else {
            self.axis_deg + 90.0
        };

        Self {
            sphere: new_sphere,
            cylinder: new_cylinder,
            axis_deg: new_axis,
        }
    }
}

/// Transpose a spherocylindrical lens prescription to the opposite cylinder form.
///
/// This function converts between plus and minus cylinder notation while maintaining
/// the same optical power. It's a convenience function that calls the `transpose()`
/// method on the `SpheroCyl` struct.
///
/// # Parameters
/// - `lens`: The spherocylindrical lens prescription to transpose
///
/// # Returns
/// A new `SpheroCyl` with the transposed prescription.
///
/// # Examples
///
/// ```
/// use opticalc::*;
/// 
/// // Plus cylinder form: +1.00 DS / -1.50 DC × 90
/// let plus_form = SpheroCyl {
///     sphere: 1.00,
///     cylinder: -1.50,
///     axis_deg: 90.0,
/// };
/// 
/// // Transpose to minus cylinder form: -0.50 DS / +1.50 DC × 180
/// let minus_form = transpose(plus_form);
/// assert_eq!(minus_form.sphere, -0.50);
/// assert_eq!(minus_form.cylinder, 1.50);
/// assert_eq!(minus_form.axis_deg, 180.0);
/// ```
pub fn transpose(lens: SpheroCyl) -> SpheroCyl {
    lens.transpose()
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    const EPS: f64 = 1e-12;

    #[test]
    fn transpose_minus_to_plus_cylinder() {
        // -3.50 DS / +2.00 DC × 150 → -1.50 DS / -2.00 DC × 60
        let minus_form = SpheroCyl {
            sphere: -3.50,
            cylinder: 2.00,
            axis_deg: 150.0,
        };
        
        let plus_form = minus_form.transpose();
        
        assert_abs_diff_eq!(plus_form.sphere, -1.50, epsilon = EPS);
        assert_abs_diff_eq!(plus_form.cylinder, -2.00, epsilon = EPS);
        assert_abs_diff_eq!(plus_form.axis_deg, 60.0, epsilon = EPS);
    }

    #[test]
    fn transpose_plus_to_minus_cylinder() {
        // +1.00 DS / -1.50 DC × 90 → -0.50 DS / +1.50 DC × 0
        let plus_form = SpheroCyl {
            sphere: 1.00,
            cylinder: -1.50,
            axis_deg: 90.0,
        };
        
        let minus_form = plus_form.transpose();
        
        assert_abs_diff_eq!(minus_form.sphere, -0.50, epsilon = EPS);
        assert_abs_diff_eq!(minus_form.cylinder, 1.50, epsilon = EPS);
        assert_abs_diff_eq!(minus_form.axis_deg, 0.0, epsilon = EPS);
    }

    #[test]
    fn transpose_double_transposition_returns_original() {
        // Transposing twice should return the original prescription
        let original = SpheroCyl {
            sphere: -2.25,
            cylinder: -1.75,
            axis_deg: 45.0,
        };
        
        let transposed_once = original.transpose();
        let transposed_twice = transposed_once.transpose();
        
        assert_abs_diff_eq!(transposed_twice.sphere, original.sphere, epsilon = EPS);
        assert_abs_diff_eq!(transposed_twice.cylinder, original.cylinder, epsilon = EPS);
        assert_abs_diff_eq!(transposed_twice.axis_deg, original.axis_deg, epsilon = EPS);
    }

    #[test]
    fn transpose_axis_adjustment_less_than_90() {
        // Axis < 90°: add 90°
        let lens = SpheroCyl {
            sphere: 0.0,
            cylinder: -2.0,
            axis_deg: 30.0,
        };
        
        let transposed = lens.transpose();
        assert_abs_diff_eq!(transposed.axis_deg, 120.0, epsilon = EPS);
    }

    #[test]
    fn transpose_axis_adjustment_greater_than_or_equal_90() {
        // Axis ≥ 90°: subtract 90°
        let lens = SpheroCyl {
            sphere: 0.0,
            cylinder: -2.0,
            axis_deg: 120.0,
        };
        
        let transposed = lens.transpose();
        assert_abs_diff_eq!(transposed.axis_deg, 30.0, epsilon = EPS);
    }

    #[test]
    fn transpose_axis_exactly_90() {
        // Axis = 90°: subtract 90° → 0°
        let lens = SpheroCyl {
            sphere: 0.0,
            cylinder: -2.0,
            axis_deg: 90.0,
        };
        
        let transposed = lens.transpose();
        assert_abs_diff_eq!(transposed.axis_deg, 0.0, epsilon = EPS);
    }

    #[test]
    fn transpose_axis_180_becomes_90() {
        // Axis = 180°: subtract 90° → 90°
        let lens = SpheroCyl {
            sphere: 0.0,
            cylinder: -2.0,
            axis_deg: 180.0,
        };
        
        let transposed = lens.transpose();
        assert_abs_diff_eq!(transposed.axis_deg, 90.0, epsilon = EPS);
    }

    #[test]
    fn transpose_zero_cylinder_unchanged() {
        // Pure sphere: no change in transposition
        let sphere = SpheroCyl {
            sphere: 2.50,
            cylinder: 0.0,
            axis_deg: 0.0,
        };
        
        let transposed = sphere.transpose();
        
        assert_abs_diff_eq!(transposed.sphere, 2.50, epsilon = EPS);
        assert_abs_diff_eq!(transposed.cylinder, 0.0, epsilon = EPS);
        assert_abs_diff_eq!(transposed.axis_deg, 90.0, epsilon = EPS); // axis still changes
    }

    #[test]
    fn transpose_function_equivalent_to_method() {
        let lens = SpheroCyl {
            sphere: -1.75,
            cylinder: 1.25,
            axis_deg: 135.0,
        };
        
        let method_result = lens.transpose();
        let function_result = transpose(lens);
        
        assert_abs_diff_eq!(method_result.sphere, function_result.sphere, epsilon = EPS);
        assert_abs_diff_eq!(method_result.cylinder, function_result.cylinder, epsilon = EPS);
        assert_abs_diff_eq!(method_result.axis_deg, function_result.axis_deg, epsilon = EPS);
    }

    #[test]
    fn transpose_preserves_optical_power() {
        // The optical power at any meridian should be preserved after transposition
        let original = SpheroCyl {
            sphere: -2.0,
            cylinder: -3.0,
            axis_deg: 60.0,
        };
        
        let transposed = original.transpose();
        
        // Test power at several meridians
        for meridian in [0.0, 30.0, 60.0, 90.0, 120.0, 150.0, 180.0] {
            let original_power = original.power_at(meridian);
            let transposed_power = transposed.power_at(meridian);
            assert_abs_diff_eq!(original_power, transposed_power, epsilon = EPS);
        }
    }

    #[test]
    fn transpose_complex_prescription() {
        // Complex prescription with fractional values
        let lens = SpheroCyl {
            sphere: -4.25,
            cylinder: 2.75,
            axis_deg: 37.5,
        };
        
        let transposed = lens.transpose();
        
        // Expected: -1.50 DS / -2.75 DC × 127.5
        assert_abs_diff_eq!(transposed.sphere, -1.50, epsilon = EPS);
        assert_abs_diff_eq!(transposed.cylinder, -2.75, epsilon = EPS);
        assert_abs_diff_eq!(transposed.axis_deg, 127.5, epsilon = EPS);
    }

    #[test]
    fn transpose_high_power_prescription() {
        // High power prescription
        let lens = SpheroCyl {
            sphere: -8.00,
            cylinder: 4.50,
            axis_deg: 15.0,
        };
        
        let transposed = lens.transpose();
        
        // Expected: -3.50 DS / -4.50 DC × 105.0
        assert_abs_diff_eq!(transposed.sphere, -3.50, epsilon = EPS);
        assert_abs_diff_eq!(transposed.cylinder, -4.50, epsilon = EPS);
        assert_abs_diff_eq!(transposed.axis_deg, 105.0, epsilon = EPS);
    }
}
