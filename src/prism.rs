//! Induced prism calculation for a sphero-cylinder lens using Prentice's rule.
//!
//! ## Overview
//! - Computes the prism induced by decentering a lens (sphere + cylinder) from the optical center.
//! - Uses **Prentice’s Rule**:  Δ = c(cm) × F(D)
//! - Handles both horizontal (in/out) and vertical (up/down) decentrations.
//!
//! ## Conventions
//! - Lens powers are in **diopters (D)**.
//! - Cylinder axis is in **degrees [0,180)**.
//! - Decentrations are in **millimeters (mm)**.
//! - **Decentration signs**:
//!     - Horizontal (x): positive = **in** (nasal), negative = **out** (temporal).
//!     - Vertical (y): positive = **up**, negative = **down**.
//! - Output prism is in **prism diopters (∆)**.

use crate::*;

/// Compute induced prism using the full sphero-cylinder power matrix,
/// including toric cross-terms, with OD/OS handling, returning clinical base directions.
///
/// ## Method
/// The lens is represented as a 2×2 power matrix:
/// ```text
/// [ Px   Pt ]
/// [ Pt   Py ]
/// ```
/// where:
/// - `Px = S + C·sin²(axis)` (power at 180° horizontal meridian)
/// - `Py = S + C·cos²(axis)` (power at 90° vertical meridian)
/// - `Pt = −C·sin(axis)·cos(axis)` (toric cross-term)
///
/// The induced prism vector Δ = F · c, where:
/// - `c` is the decentration vector in cm (converted from mm),
/// - horizontal component = `-decentration_in/10`,
/// - vertical component   = `decentration_up/10`.
///
/// ## Eye handling
/// - For **OD**: positive `decentration_in` means nasal (in), negative = temporal (out).
/// - For **OS**: nasal is opposite, so the horizontal decentration is **negated** before use.
///
/// The returned [`CombinedPrism`] uses [`HorizontalPrism`] and [`VerticalPrism`] with
/// non-negative magnitudes and explicit base directions.
pub fn induced_prism(eye: Eye, lens: SpheroCyl, dec: Decentration) -> CombinedPrism {
    let s: f64 = lens.sphere;
    let c = lens.cylinder;
    let axis_rad = lens.axis_deg.to_radians();
    let sin_axis = axis_rad.sin();
    let cos_axis = axis_rad.cos();

    // Power components (Px, Pt, Py)
    let px = s + c * sin_axis * sin_axis; // power at 180° (horizontal meridian)
    let pt = -c * sin_axis * cos_axis; // toric cross-term
    let py = s + c * cos_axis * cos_axis; // power at 90° (vertical meridian)

    // Map decentrations and apply the OS nasal flip for the "in" component
    let dec_up_mm = dec.vertical_mm; // +up / −down
    let dec_in_mm = dec.horizontal_mm; // +in / −out
    let dec_in_adjusted_mm = match eye {
        Eye::OD => dec_in_mm,
        Eye::OS => -dec_in_mm, // nasal direction flips for OS
    };

    // Use cm and a leading negative on both components in the horizontal expression:
    // horizontal = (Px * -in_adj/10) + (Pt * -up/10)
    // vertical   = (-Pt * in_adj/10) + (-Py * up/10)
    let horiz_value = (px * (-dec_in_adjusted_mm) / 10.0) + (pt * (-dec_up_mm) / 10.0);
    let vert_value = (pt * dec_in_adjusted_mm / 10.0) + (py * dec_up_mm / 10.0);

    // Resolve clinical base directions and non-negative magnitudes.

    // Horizontal depends on eye:
    let horizontal_base = match eye {
        Eye::OD => {
            if horiz_value < 0.0 {
                HorizontalBase::In
            } else {
                HorizontalBase::Out
            }
        }
        Eye::OS => {
            if horiz_value < 0.0 {
                HorizontalBase::Out
            } else {
                HorizontalBase::In
            }
        }
    };
    let horizontal = HorizontalPrism::new(horiz_value.abs(), horizontal_base);

    // Vertical is eye-independent with BU positive, BD negative:
    let vertical_base = if vert_value >= 0.0 {
        VerticalBase::Up
    } else {
        VerticalBase::Down
    };
    let vertical = VerticalPrism::new(vert_value.abs(), vertical_base);

    CombinedPrism {
        horizontal,
        vertical,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::{assert_abs_diff_eq, assert_relative_eq};

    #[test]
    fn test_1() {
        let lens = SpheroCyl {
            sphere: 2.0,
            cylinder: -1.0,
            axis_deg: 26.0,
        };
        let dec = Decentration {
            horizontal_mm: 1.0,
            vertical_mm: 3.0,
        };

        let p = induced_prism(Eye::OS, lens, dec);
        assert_abs_diff_eq!(p.horizontal.signed(), -0.0625, epsilon = 1e-4);
        assert_relative_eq!(p.vertical.signed(), 0.31825, epsilon = 1e-4);
        assert_relative_eq!(p.horizontal.amount(), 0.0625, epsilon = 1e-4);
        assert!(matches!(p.horizontal.base(), Some(HorizontalBase::In)));
        assert_relative_eq!(p.vertical.amount(), 0.31825, epsilon = 1e-4);
        assert!(matches!(p.vertical.base(), Some(VerticalBase::Up)));

        let p = induced_prism(Eye::OD, lens, dec);
        assert_relative_eq!(p.vertical.signed(), 0.397, epsilon = 1e-4);
        assert_relative_eq!(p.vertical.amount(), 0.397, epsilon = 1e-4);
        assert!(matches!(p.vertical.base(), Some(VerticalBase::Up)));

        assert_abs_diff_eq!(p.horizontal.signed(), -0.299, epsilon = 1e-4);
        assert_relative_eq!(p.horizontal.amount(), 0.299, epsilon = 1e-4);
        assert!(matches!(p.horizontal.base(), Some(HorizontalBase::In)));
        
    }

    #[test]
    fn test_3() {
        let lens = SpheroCyl {
            sphere: -2.00,
            cylinder: -3.40,
            axis_deg: -5.0,
        };
        let dec = Decentration {
            horizontal_mm: -1.5,
            vertical_mm: -3.0,
        };

        let p = induced_prism(Eye::OD, lens, dec);
        assert_relative_eq!(p.horizontal.amount(), 0.392434, epsilon = 1e-4);
        assert!(matches!(p.horizontal.base(), Some(HorizontalBase::In)));
        assert_relative_eq!(p.vertical.amount(), 1.6565, epsilon = 1e-4);
        assert!(matches!(p.vertical.base(), Some(VerticalBase::Up)));

        let p = induced_prism(Eye::OS, lens, dec);
        assert_relative_eq!(p.horizontal.signed(), -0.21531, epsilon = 1e-4);
        assert_relative_eq!(p.horizontal.amount(), 0.21531, epsilon = 1e-4);
        assert!(matches!(p.horizontal.base(), Some(HorizontalBase::In)));
        assert_relative_eq!(p.vertical.signed(), 1.568, epsilon = 1e-4);
        assert_relative_eq!(p.vertical.amount(), 1.568, epsilon = 1e-4);
        assert!(matches!(p.vertical.base(), Some(VerticalBase::Up)));
    }

    #[test]
    fn test_4() {
        let lens = SpheroCyl {
            sphere: 0.0,
            cylinder: 0.0,
            axis_deg: 0.0,
        };
        let dec = Decentration {
            horizontal_mm: 0.0,
            vertical_mm: 0.0,
        };

        let p = induced_prism(Eye::OD, lens, dec);

        assert_abs_diff_eq!(p.horizontal.amount(), 0.0, epsilon = 1e-4);
        assert_relative_eq!(p.vertical.amount(), 0.0, epsilon = 1e-4);
        assert!(p.horizontal.base().is_none());
        assert!(p.vertical.base().is_none());
    }

    #[test]
    fn pure_sphere_horizontal_decentration_od_vs_os() {
        // +3.00 DS; 5 mm in
        let lens = SpheroCyl {
            sphere: 3.0,
            cylinder: 0.0,
            axis_deg: 0.0,
        };
        let dec = Decentration {
            horizontal_mm: 5.0,
            vertical_mm: 0.0,
        };

        // OD
        let p_od = induced_prism(Eye::OD, lens, dec);
        assert_abs_diff_eq!(p_od.horizontal.signed(), -1.5, epsilon = 1e-6);
        assert_abs_diff_eq!(p_od.vertical.signed(), 0.0, epsilon = 1e-6);
        assert_abs_diff_eq!(p_od.horizontal.amount(), 1.5, epsilon = 1e-6);
        assert!(matches!(p_od.horizontal.base(), Some(HorizontalBase::In)));

        // OS (nasal flip)
        let p_os = induced_prism(Eye::OS, lens, dec);
        assert_abs_diff_eq!(p_os.horizontal.signed(), -1.5, epsilon = 1e-6);
        assert_abs_diff_eq!(p_os.vertical.signed(), 0.0, epsilon = 1e-6);
        assert_abs_diff_eq!(p_os.horizontal.amount(), 1.5, epsilon = 1e-6);
        assert!(matches!(p_os.horizontal.base(), Some(HorizontalBase::In)));
    }

    #[test]
    fn pure_cylinder_vertical_decentration_axis_0_or_180_equivalence() {
        //  −2.00 DC × 180; 4 mm up
        let lens = SpheroCyl {
            sphere: 0.0,
            cylinder: -2.0,
            axis_deg: 180.0,
        };
        let dec = Decentration {
            horizontal_mm: 0.0,
            vertical_mm: 4.0,
        };

        let p = induced_prism(Eye::OD, lens, dec);
        // Prentice: Py = -2.00; c = 0.4 cm → vertical = -(-2)*0.4 = +0.8 ∆ (base-down)
        assert_abs_diff_eq!(p.horizontal.signed(), 0.0, epsilon = 1e-6);
        assert_abs_diff_eq!(p.vertical.signed(), -0.8, epsilon = 1e-6);
        assert!(matches!(p.vertical.base(), Some(VerticalBase::Down)));
        assert_eq!(p.horizontal.base(), None);

        // Axis 0° should behave the same for pure cyl wrt meridional powers
        let lens0 = SpheroCyl {
            sphere: 0.0,
            cylinder: -2.0,
            axis_deg: 0.0,
        };
        let p0 = induced_prism(Eye::OD, lens0, dec);
        assert_abs_diff_eq!(p0.vertical.signed(), p.vertical.signed(), epsilon = 1e-6);
    }

    #[test]
    fn pure_cylinder_horizontal_decentration_axis_90() {
        // −2.00 DC × 90; 3 mm in
        let lens = SpheroCyl {
            sphere: 0.0,
            cylinder: -2.0,
            axis_deg: 90.0,
        };
        let dec = Decentration {
            horizontal_mm: 3.0,
            vertical_mm: 0.0,
        };

        let p = induced_prism(Eye::OD, lens, dec);
        // Px = -2.00 at 180°; c = 0.3 cm → horizontal = -Px * c = 0.6 ∆ (positive)
        assert_abs_diff_eq!(p.horizontal.signed(), 0.6, epsilon = 1e-6);
        assert_abs_diff_eq!(p.vertical.signed(), 0.0, epsilon = 1e-6);
        assert!(matches!(p.horizontal.base(), Some(HorizontalBase::Out)));
    }

    #[test]
    fn mixed_sphero_cyl_os_out_and_up() {
        // −4.00 DS / +2.00 DC × 45; 2 mm out (−), 1 mm up
        let lens = SpheroCyl {
            sphere: -4.0,
            cylinder: 2.0,
            axis_deg: 45.0,
        };
        let dec = Decentration {
            horizontal_mm: -2.0,
            vertical_mm: 1.0,
        };

        let p = induced_prism(Eye::OS, lens, dec);
        assert_abs_diff_eq!(p.horizontal.signed(), -0.7, epsilon = 1e-6);
        assert_abs_diff_eq!(p.vertical.signed(), -0.5, epsilon = 1e-6);
        assert!(matches!(p.horizontal.base(), Some(HorizontalBase::In)));
        assert!(matches!(p.vertical.base(), Some(VerticalBase::Down)));
    }

    #[test]
    fn down_only_with_toric_cross_term() {
        // −1.50 DS / −1.50 DC × 30; 3 mm down
        let lens = SpheroCyl {
            sphere: -1.5,
            cylinder: -1.5,
            axis_deg: 30.0,
        };
        let dec = Decentration {
            horizontal_mm: 0.0,
            vertical_mm: -3.0,
        };

        let p = induced_prism(Eye::OD, lens, dec);
        assert_abs_diff_eq!(p.horizontal.signed(), 0.19485571585149866, epsilon = 1e-9);
        assert_abs_diff_eq!(p.vertical.signed(), 0.7875, epsilon = 1e-9);
        assert_eq!(p.horizontal.base(), Some(HorizontalBase::Out));
        assert_eq!(p.vertical.base(), Some(VerticalBase::Up));
    }

    #[test]
    fn equal_in_and_down_at_45_has_equal_components() {
        // 0 / −2.00 DC × 45; 2.5 mm in & 2.5 mm down
        let lens = SpheroCyl {
            sphere: 0.0,
            cylinder: -2.0,
            axis_deg: 45.0,
        };
        let dec = Decentration {
            horizontal_mm: 2.5,
            vertical_mm: -2.5,
        };

        let p = induced_prism(Eye::OD, lens, dec);
        assert_abs_diff_eq!(p.horizontal.signed(), 0.5, epsilon = 1e-9);
        assert_abs_diff_eq!(p.vertical.signed(), 0.5, epsilon = 1e-9);
        // Magnitude should be sqrt(0.5^2 + 0.5^2) = ~0.70710678
        assert_abs_diff_eq!(p.magnitude(), 0.7071067811865476, epsilon = 1e-12);
    }

    #[test]
    fn axis_135_cancels_components_for_symmetric_decentration() {
        // 0 / −2.00 DC × 135; 2.5 mm in & 2.5 mm down → ~zero vector (numerical noise)
        let lens = SpheroCyl {
            sphere: 0.0,
            cylinder: -2.0,
            axis_deg: 135.0,
        };
        let dec = Decentration {
            horizontal_mm: 2.5,
            vertical_mm: -2.5,
        };

        let p = induced_prism(Eye::OD, lens, dec);
        assert_abs_diff_eq!(p.horizontal.signed(), 0.0, epsilon = 1e-12);
        assert_abs_diff_eq!(p.vertical.signed(), 0.0, epsilon = 1e-12);
        assert_abs_diff_eq!(p.magnitude(), 0.0, epsilon = 1e-12);
    }

    #[test]
    fn zero_decentration_yields_zero_prism_even_with_power() {
        let lens = SpheroCyl {
            sphere: 1.25,
            cylinder: -3.50,
            axis_deg: 72.0,
        };
        let dec = Decentration {
            horizontal_mm: 0.0,
            vertical_mm: 0.0,
        };
        let p = induced_prism(Eye::OD, lens, dec);
        assert_abs_diff_eq!(p.horizontal.signed(), 0.0, epsilon = 1e-9);
        assert_abs_diff_eq!(p.vertical.signed(), 0.0, epsilon = 1e-9);
        assert_abs_diff_eq!(p.magnitude(), 0.0, epsilon = 1e-9);
    }

    #[test]
    fn one_centimeter_in_scales_linearly() {
        // +1.00 DS; 10 mm in (1 cm)
        let lens = SpheroCyl {
            sphere: 1.0,
            cylinder: -1.0,
            axis_deg: 0.0,
        };
        let dec = Decentration {
            horizontal_mm: 10.0,
            vertical_mm: 0.0,
        };
        let p = induced_prism(Eye::OD, lens, dec);
        // Px = +1.00; horizontal = -Px * 1.0 cm = -1.00 ∆
        assert_abs_diff_eq!(p.horizontal.signed(), -1.0, epsilon = 1e-9);
        assert_abs_diff_eq!(p.vertical.signed(), 0.0, epsilon = 1e-9);
        assert_abs_diff_eq!(p.magnitude(), 1.0, epsilon = 1e-9);
    }

    #[test]
    fn power_at_matches_definition() {
        let lens = SpheroCyl {
            sphere: -2.0,
            cylinder: 3.0,
            axis_deg: 25.0,
        };
        // φ = axis → power = S (since sin^2(0)=0)
        assert_abs_diff_eq!(lens.power_at(25.0), -2.0, epsilon = 1e-12);
        // φ = axis + 90 → power = S + C (since sin^2(90)=1)
        assert_abs_diff_eq!(lens.power_at(115.0), 1.0, epsilon = 1e-12);
        // φ = 0 and φ = 90 should align with Px and Py formulas
        let axis = lens.axis_deg.to_radians();
        let px = lens.sphere + lens.cylinder * axis.sin().powi(2);
        let py = lens.sphere + lens.cylinder * axis.cos().powi(2);
        assert_abs_diff_eq!(lens.power_at(0.0), px, epsilon = 1e-12);
        assert_abs_diff_eq!(lens.power_at(90.0), py, epsilon = 1e-12);
    }
}
