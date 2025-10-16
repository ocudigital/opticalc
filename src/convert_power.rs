use crate::*;

/// Convert a lens power measured under an assumed refractive index
/// to the true power for the actual material index.
///
/// Physics (thin-lens approximation in air):
///     F = (n - 1) * S,  where S depends only on surface curvatures.
/// If a lensmeter computes F_meas using n_assumed, then:
///     F_meas = (n_assumed - 1) * S
///     F_true = (n_actual  - 1) * S
/// =>  F_true = F_meas * (n_actual - 1) / (n_assumed - 1)
///
/// Notes:
/// - Works for sphere power, cylinder power, and prism-free measurements.
/// - Axis is unchanged; scale sphere & cylinder by the same factor.
/// - This is the standard, practical conversion used in labs. Thickness/vertex
///   effects are ignored (typically well within ANSI tolerances for routine work).
pub fn convert_power(measured_power_diopters: f64, n_assumed: f64, n_actual: f64) -> f64 {
    assert!(n_assumed > 1.0, "n_assumed must be > 1.0");
    assert!(n_actual  > 1.0, "n_actual must be > 1.0");

    let k_assumed = n_assumed - 1.0;
    let k_actual  = n_actual  - 1.0;
    measured_power_diopters * (k_actual / k_assumed)
}

/// Convert a full sph-cyl-axis Rx measured under an assumed index to the true Rx
/// at the actual index. Axis is returned unchanged.
///
/// Example:
/// A polycarbonate lens (n_actual = 1.586) designed to be -5.00 D true power
/// will often read about -4.463 D on a lensmeter calibrated to 1.523.
/// Converting -4.463 D measured @ 1.523 -> true @ 1.586 yields ~ -5.00 D.
pub fn convert_rx(
    measured: SpheroCyl,
    n_assumed: f64,
    n_actual: f64,
) -> SpheroCyl {
    let s = convert_power(measured.sphere, n_assumed, n_actual);
    let c = convert_power(measured.cylinder, n_assumed, n_actual);

    SpheroCyl {
        sphere: s,
        cylinder: c,
        axis_deg: measured.axis_deg,
    }
}

/// Inverse of `convert_rx`: given the *true* Rx at the actual index,
/// compute what a lensmeter calibrated to the assumed index would read.
/// Axis is unchanged.
///
/// Example:
/// True polycarbonate Rx: -5.00 D @ n = 1.586.
/// A lensmeter calibrated to 1.523 would read about -4.463 D.
pub fn simulate_lensmeter_reading(
    true_rx: SpheroCyl,
    n_assumed: f64,
    n_actual: f64,
) -> SpheroCyl {
    let s = convert_power(true_rx.sphere, n_actual, n_assumed);
    let c = convert_power(true_rx.cylinder, n_actual, n_assumed);

    SpheroCyl {
        sphere: s,
        cylinder: c,
        axis_deg: true_rx.axis_deg,
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use approx::{assert_abs_diff_eq, assert_relative_eq};

    // ~0.001 D absolute tolerance is fine for our numeric checks
    const EPS: f64 = 1e-3;

    #[test]
    fn convert_power_poly_example_true_from_measured() {
        // Lensmeter assumes crown glass 1.523; actual lens is poly 1.586.
        // Measured: -4.463 D @ 1.523  => True ≈ -5.00 D @ 1.586
        let n_assumed = 1.523;
        let n_actual = 1.586;
        let measured = -4.463;

        let true_power = convert_power(measured, n_assumed, n_actual);
        assert_abs_diff_eq!(true_power, -5.00, epsilon = EPS);
    }

    #[test]
    fn convert_power_measured_from_true_inverse() {
        // Inverse of the above: True -5.00 @ 1.586 => Measured ≈ -4.463 @ 1.523
        let n_assumed = 1.523;
        let n_actual = 1.586;
        let true_power = -5.00;

        let measured = convert_power(true_power, n_actual, n_assumed);
        assert_abs_diff_eq!(measured, -4.463, epsilon = EPS);
    }

    #[test]
    fn convert_power_zero_is_stable() {
        let z = convert_power(0.0, 1.523, 1.586);
        assert_abs_diff_eq!(z, 0.0, epsilon = EPS);
    }

    #[test]
    fn convert_rx_scales_sphere_and_cylinder_axis_unchanged() {
        let n_assumed = 1.523;
        let n_actual = 1.586;

        let measured = SpheroCyl { sphere: -2.00, cylinder: -1.00, axis_deg: 180.0 };
        let out = convert_rx(measured, n_assumed, n_actual);

        // Scale factor (n_actual-1)/(n_assumed-1)
        let factor = (n_actual - 1.0) / (n_assumed - 1.0); // ≈ 1.120458...
        let expected_s = measured.sphere * factor;   // ≈ -2.2409
        let expected_c = measured.cylinder * factor; // ≈ -1.1205

        assert_abs_diff_eq!(out.sphere, expected_s, epsilon = EPS);
        assert_abs_diff_eq!(out.cylinder, expected_c, epsilon = EPS);
        assert_abs_diff_eq!(out.axis_deg, measured.axis_deg, epsilon = 1e-9);
    }

    #[test]
    fn convert_rx_handles_plus_cylinder_notation() {
        let n_assumed = 1.523;
        let n_actual = 1.586;

        // Plus-cylinder form: +1.00 DC × 90
        let measured = SpheroCyl { sphere: -1.00, cylinder: 1.00, axis_deg: 90.0 };
        let out = convert_rx(measured, n_assumed, n_actual);

        let factor = (n_actual - 1.0) / (n_assumed - 1.0);

        assert_abs_diff_eq!(out.sphere, measured.sphere * factor, epsilon = EPS);
        assert_abs_diff_eq!(out.cylinder, measured.cylinder * factor, epsilon = EPS);
        assert_abs_diff_eq!(out.axis_deg, 90.0, epsilon = 1e-9);
    }

    #[test]
    fn simulate_lensmeter_reading_inverse_of_convert_rx() {
        // Pick an arbitrary true Rx at n_actual and simulate lensmeter @ n_assumed,
        // then convert back; expect to recover the original (within EPS).
        let n_assumed = 1.523;
        let n_actual = 1.586;

        let true_rx = SpheroCyl { sphere: -3.25, cylinder: -2.25, axis_deg: 37.0 };

        let reading = simulate_lensmeter_reading(true_rx, n_assumed, n_actual);
        let recovered = convert_rx(reading, n_assumed, n_actual);

        assert_abs_diff_eq!(recovered.sphere, true_rx.sphere, epsilon = EPS);
        assert_abs_diff_eq!(recovered.cylinder, true_rx.cylinder, epsilon = EPS);
        assert_abs_diff_eq!(recovered.axis_deg, true_rx.axis_deg, epsilon = 1e-9);
    }

    #[test]
    fn convert_rx_zero_cylinder_passes_through() {
        let n_assumed = 1.523;
        let n_actual = 1.586;
        let measured = SpheroCyl { sphere: 2.50, cylinder: 0.0, axis_deg: 5.0 };
        let out = convert_rx(measured, n_assumed, n_actual);

        let factor = (n_actual - 1.0) / (n_assumed - 1.0);
        assert_abs_diff_eq!(out.sphere, 2.50 * factor, epsilon = EPS);
        assert_abs_diff_eq!(out.cylinder, 0.0, epsilon = EPS);
        assert_abs_diff_eq!(out.axis_deg, 5.0, epsilon = 1e-9);
    }

    #[test]
    #[should_panic(expected = "n_assumed must be > 1.0")]
    fn convert_power_panics_on_bad_assumed_index() {
        let _ = convert_power(1.0, 1.0, 1.586);
    }

    #[test]
    #[should_panic(expected = "n_actual must be > 1.0")]
    fn convert_power_panics_on_bad_actual_index() {
        let _ = convert_power(1.0, 1.523, 1.0);
    }

    #[test]
    fn relative_tolerance_example_factor() {
        let n_assumed = 1.523;
        let n_actual = 1.586;
        let factor = (n_actual - 1.0) / (n_assumed - 1.0);
        // Compare against a hard-coded reference using relative tolerance
        assert_relative_eq!(factor, 1.120458, max_relative = 1e-6);
    }
}
