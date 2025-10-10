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

/// Defines the eye.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Eye {
    /// Right eye.
    OD,

    /// Left eye.
    OS,
}

/// Base direction for horizontal prism relative to the patient.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HorizontalBase {
    In,
    Out,
}

/// Base direction for vertical prism.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerticalBase {
    Up,
    Down,
}

/// Defines a sphero-cylinder lens.
#[derive(Debug, Clone, Copy)]
pub struct SpheroCyl {
    /// Spherical component of the lens power (D).
    /// Example: +2.00 DS → `sphere = 2.0`.
    pub sphere: f64,

    /// Cylindrical component of the lens power (D).
    /// Can be written in minus or plus form.
    /// Example: −1.25 DC × 180 → `cylinder = -1.25, axis_deg = 180.0`.
    pub cylinder: f64,

    /// Cylinder axis in degrees, range [0, 180).
    /// Example: −1.25 DC × 90 → `axis_deg = 90.0`.
    pub axis_deg: f64,
}

/// Represents lens decentration relative to the optical center.
#[derive(Debug, Clone, Copy)]
pub struct Decentration {
    /// Vertical decentration in millimeters.
    /// - Positive = **up**.
    /// - Negative = **down**.
    /// Example: 3 mm down → `vertical_mm = -3.0`.
    pub vertical_mm: f64,

    /// Horizontal decentration in millimeters.
    /// - Positive = **in** (nasal).
    /// - Negative = **out** (temporal).
    /// Example: 2 mm in → `horizontal_mm = 2.0`.
    pub horizontal_mm: f64,
}

/// Raw induced prism vector in prism diopters (∆).
#[derive(Debug, Clone, Copy)]
pub struct InducedPrism {
    /// Signed horizontal prism (∆).
    /// Sign is intermediate; interpretation depends on eye.
    pub horizontal_prism: f64,
    /// Signed vertical prism (∆).
    /// Positive/negative correspond to base-down/up before interpretation.
    pub vertical_prism: f64,
    /// Vector magnitude (∆).
    pub magnitude: f64,
}

/// Prism with clinical base direction resolved for a given eye.
#[derive(Debug, Clone, Copy)]
pub struct ClinicalInducedPrism {
    pub eye: Eye,
    pub prism: InducedPrism,
}

impl ClinicalInducedPrism {
    /// Get horizontal prism magnitude and base direction. Will return None if the prism is 0.
    pub fn horizontal(&self) -> Option<(f64, HorizontalBase)> {
        let value = self.prism.horizontal_prism;
        if value == 0.0 {
            return None;
        }

        let base = match self.eye {
            Eye::OD => {
                if value < 0.0 {
                    HorizontalBase::In
                } else {
                    HorizontalBase::Out
                }
            }
            Eye::OS => {
                if value < 0.0 {
                    HorizontalBase::Out
                } else {
                    HorizontalBase::In
                }
            }
        };

        Some((value.abs(), base))
    }

    /// Get vertical prism magnitude and base direction. Will return None if the prism is 0.
    pub fn vertical(&self) -> Option<(f64, VerticalBase)> {
        let value = self.prism.vertical_prism;
        if value == 0.0 {
            return None;
        }

        let base = if value < 0.0 {
            VerticalBase::Up
        } else {
            VerticalBase::Down
        };

        Some((value.abs(), base))
    }
}

impl SpheroCyl {
    /// Compute meridional power at meridian φ (in degrees).
    /// - φ=0° → horizontal meridian.
    /// - φ=90° → vertical meridian.
    pub fn power_at(&self, phi_deg: f64) -> f64 {
        let delta = (phi_deg - self.axis_deg).to_radians();
        self.sphere + self.cylinder * (delta.sin().powi(2))
    }
}

/// Compute induced prism using the full sphero-cylinder power matrix,
/// including toric cross-terms, with OD/OS handling.
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
/// - vertical component   = `-decentration_up/10`.
///
/// ## Eye handling
/// - For **OD**: positive `decentration_in` means nasal (in), negative = temporal (out).
/// - For **OS**: nasal is opposite, so the horizontal decentration is negated.
///
/// ## Output
/// - Returns `Prism` with signed horizontal and vertical components (∆).
/// - Positive/negative signs indicate direction; base direction labels can be
///   derived from eye + sign convention.
/// - `magnitude` is the Euclidean norm of the prism vector (always ≥ 0).
pub fn induced_prism(eye: Eye, lens: SpheroCyl, dec: Decentration) -> ClinicalInducedPrism {
    let s: f64 = lens.sphere;
    let c = lens.cylinder;
    let axis_rad = lens.axis_deg.to_radians();
    let sin_axis = axis_rad.sin();
    let cos_axis = axis_rad.cos();

    // Power components (Px, Pt, Py)
    let px = s + c * sin_axis * sin_axis; // power at 180° (horizontal meridian)
    let pt = -c * sin_axis * cos_axis; // toric cross-term
    let py = s + c * cos_axis * cos_axis; // power at 90° (vertical meridian)

    // Map decentrations to the variables and apply the OS nasal flip
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
    let vert_value = (-pt * dec_in_adjusted_mm / 10.0) + (-py * dec_up_mm / 10.0);

    let prism = InducedPrism {
        horizontal_prism: horiz_value,
        vertical_prism: vert_value,
        magnitude: horiz_value.hypot(vert_value),
    };

    ClinicalInducedPrism { eye, prism }
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

        assert_abs_diff_eq!(p.prism.horizontal_prism, 0.0625, epsilon = 1e-4);
        assert_relative_eq!(p.prism.vertical_prism, -0.31825, epsilon = 1e-4);

        // Optional: check clinical bases
        let (hmag, hbase) = p
            .horizontal()
            .expect("expected non-zero horizontal prism");
        let (vmag, vbase) = p
            .vertical()
            .expect("expected non-zero vertical prism");
        assert_relative_eq!(hmag, 0.0625, epsilon = 1e-4);
        assert!(matches!(hbase, HorizontalBase::In));
        assert_relative_eq!(vmag, 0.31825, epsilon = 1e-4);
        assert!(matches!(vbase, VerticalBase::Up));
    }

    #[test]
    fn test_2() {
        let lens = SpheroCyl {
            sphere: 2.0,
            cylinder: -1.0,
            axis_deg: 26.0,
        };
        let dec = Decentration {
            horizontal_mm: 1.0,
            vertical_mm: 3.0,
        };

        let p = induced_prism(Eye::OD, lens, dec);

        assert_abs_diff_eq!(p.prism.horizontal_prism, -0.2989, epsilon = 1e-4);
        assert_relative_eq!(p.prism.vertical_prism, -0.397, epsilon = 1e-4);

        // Optional: check clinical bases
        let (hmag, hbase) = p
            .horizontal()
            .expect("expected non-zero horizontal prism");
        let (vmag, vbase) = p
            .vertical()
            .expect("expected non-zero vertical prism");
        assert_relative_eq!(hmag, 0.2989, epsilon = 1e-4);
        assert!(matches!(hbase, HorizontalBase::In));
        assert_relative_eq!(vmag, 0.397, epsilon = 1e-4);
        assert!(matches!(vbase, VerticalBase::Up));
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

        // Optional: check clinical bases
        let (hmag, hbase) = p
            .horizontal()
            .expect("expected non-zero horizontal prism");
        let (vmag, vbase) = p
            .vertical()
            .expect("expected non-zero vertical prism");
        assert_relative_eq!(hmag, 0.392434, epsilon = 1e-4);
        assert!(matches!(hbase, HorizontalBase::In));
        assert_relative_eq!(vmag, 1.6565, epsilon = 1e-4);
        assert!(matches!(vbase, VerticalBase::Up));
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

        assert_abs_diff_eq!(p.prism.horizontal_prism, 0.0, epsilon = 1e-4);
        assert_relative_eq!(p.prism.vertical_prism, 0.0, epsilon = 1e-4);

        // Optional: check clinical bases (should be None when zero)
        assert!(p.horizontal().is_none());
        assert!(p.vertical().is_none());
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
        assert_abs_diff_eq!(p_od.prism.horizontal_prism, -1.5, epsilon = 1e-6);
        assert_abs_diff_eq!(p_od.prism.vertical_prism, 0.0, epsilon = 1e-6);
        let (hmag_od, hbase_od) = p_od
            .horizontal()
            .expect("expected non-zero horizontal prism for OD");
        assert_abs_diff_eq!(hmag_od, 1.5, epsilon = 1e-6);
        // OD: negative -> base in
        assert!(matches!(hbase_od, HorizontalBase::In));

        // OS (nasal flip)
        let p_os = induced_prism(Eye::OS, lens, dec);
        assert_abs_diff_eq!(p_os.prism.horizontal_prism, 1.5, epsilon = 1e-6);
        assert_abs_diff_eq!(p_os.prism.vertical_prism, 0.0, epsilon = 1e-6);
        let (hmag_os, hbase_os) = p_os
            .horizontal()
            .expect("expected non-zero horizontal prism for OS");
        assert_abs_diff_eq!(hmag_os, 1.5, epsilon = 1e-6);
        // OS: positive -> base in
        assert!(matches!(hbase_os, HorizontalBase::In));
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
        assert_abs_diff_eq!(p.prism.horizontal_prism, 0.0, epsilon = 1e-6);
        assert_abs_diff_eq!(p.prism.vertical_prism, 0.8, epsilon = 1e-6);
        let (vmag, vbase) = p
            .vertical()
            .expect("expected non-zero vertical prism");
        assert_abs_diff_eq!(vmag, 0.8, epsilon = 1e-6);
        assert!(matches!(vbase, VerticalBase::Down));

        // Axis 0° should behave the same for pure cyl wrt meridional powers
        let lens0 = SpheroCyl {
            sphere: 0.0,
            cylinder: -2.0,
            axis_deg: 0.0,
        };
        let p0 = induced_prism(Eye::OD, lens0, dec);
        assert_abs_diff_eq!(
            p0.prism.vertical_prism,
            p.prism.vertical_prism,
            epsilon = 1e-6
        );
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
        assert_abs_diff_eq!(p.prism.horizontal_prism, 0.6, epsilon = 1e-6);
        assert_abs_diff_eq!(p.prism.vertical_prism, 0.0, epsilon = 1e-6);
        let (hmag, hbase) = p
            .horizontal()
            .expect("expected non-zero horizontal prism");
        assert_abs_diff_eq!(hmag, 0.6, epsilon = 1e-6);
        // OD: positive -> base out
        assert!(matches!(hbase, HorizontalBase::Out));
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
        assert_abs_diff_eq!(p.prism.horizontal_prism, 0.7, epsilon = 1e-6);
        assert_abs_diff_eq!(p.prism.vertical_prism, 0.5, epsilon = 1e-6);
        // Clinical bases
        let (hmag, hbase) = p
            .horizontal()
            .expect("expected non-zero horizontal prism");
        let (vmag, vbase) = p
            .vertical()
            .expect("expected non-zero vertical prism");
        assert_abs_diff_eq!(hmag, 0.7, epsilon = 1e-6);
        assert!(matches!(hbase, HorizontalBase::In)); // OS + → base in
        assert_abs_diff_eq!(vmag, 0.5, epsilon = 1e-6);
        assert!(matches!(vbase, VerticalBase::Down)); // + → base down
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
        assert_abs_diff_eq!(
            p.prism.horizontal_prism,
            0.19485571585149866,
            epsilon = 1e-9
        );
        assert_abs_diff_eq!(p.prism.vertical_prism, -0.7875, epsilon = 1e-9);
        let (_hmag, _hbase) = p
            .horizontal()
            .expect("expected non-zero horizontal prism");
        let (vmag, vbase) = p
            .vertical()
            .expect("expected non-zero vertical prism");
        assert_abs_diff_eq!(vmag, 0.7875, epsilon = 1e-9);
        assert!(matches!(vbase, VerticalBase::Up)); // negative → base up
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
        assert_abs_diff_eq!(p.prism.horizontal_prism, 0.5, epsilon = 1e-9);
        assert_abs_diff_eq!(p.prism.vertical_prism, -0.5, epsilon = 1e-9);
        // Magnitude should be sqrt(0.5^2 + 0.5^2) = ~0.70710678
        assert_abs_diff_eq!(p.prism.magnitude, 0.7071067811865476, epsilon = 1e-12);
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
        assert_abs_diff_eq!(p.prism.horizontal_prism, 0.0, epsilon = 1e-12);
        assert_abs_diff_eq!(p.prism.vertical_prism, 0.0, epsilon = 1e-12);
        assert_abs_diff_eq!(p.prism.magnitude, 0.0, epsilon = 1e-12);
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
        assert_abs_diff_eq!(p.prism.horizontal_prism, 0.0, epsilon = 1e-9);
        assert_abs_diff_eq!(p.prism.vertical_prism, 0.0, epsilon = 1e-9);
        assert_abs_diff_eq!(p.prism.magnitude, 0.0, epsilon = 1e-9);
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
        assert_abs_diff_eq!(p.prism.horizontal_prism, -1.0, epsilon = 1e-9);
        assert_abs_diff_eq!(p.prism.vertical_prism, 0.0, epsilon = 1e-9);
        assert_abs_diff_eq!(p.prism.magnitude, 1.0, epsilon = 1e-9);
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
