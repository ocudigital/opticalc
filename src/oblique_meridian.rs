use crate::*;

/// Oblique Meridian Power
///
/// Compute meridional power at meridian φ (in degrees).
/// - φ=0° → horizontal meridian.
/// - φ=90° → vertical meridian.
pub fn oblique_meridian(lens: SpheroCyl, axis_deg: f64) -> f64 {
    lens.power_at(axis_deg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn matches_power_at_definition() {
        let lens = SpheroCyl {
            sphere: -2.0,
            cylinder: -4.0,
            axis_deg: 30.0,
        };
        for a in [0.0, 15.0, 30.0, 45.0, 60.0, 90.0, 120.0, 150.0, 180.0] {
            assert_abs_diff_eq!(oblique_meridian(lens, a), lens.power_at(a), epsilon = 1e-12);
        }
    }

    #[test]
    fn special_cases_90_and_180() {
        let lens = SpheroCyl {
            sphere: 1.25,
            cylinder: -2.75,
            axis_deg: 30.0,
        };
        let p90 = oblique_meridian(lens, 90.0);
        let expected_90 =
            lens.sphere + lens.cylinder * ((90.0 - lens.axis_deg).to_radians().sin().powi(2));
        assert_abs_diff_eq!(p90, expected_90, epsilon = 1e-12);

        let p180 = oblique_meridian(lens, 180.0);
        let expected_180 =
            lens.sphere + lens.cylinder * ((180.0 - lens.axis_deg).to_radians().sin().powi(2));
        assert_abs_diff_eq!(p180, expected_180, epsilon = 1e-12);
    }
}
