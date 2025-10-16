use crate::*;

/// Calculate Obliquely Crossed Cylinders
/// 
/// Combine two spherocylindrical lens prescriptions into a single, resultant prescription.
/// If you have two lenses—each with its own sphere, cylinder power, and axis, compute the single lens power and orientation that would replicate the net effect of stacking or combining those two lenses.
pub fn crossed_cylinders(lens1: SpheroCyl, lens2: SpheroCyl) -> SpheroCyl {
    // Power matrix components for lens 1
    let s1 = lens1.sphere;
    let c1 = lens1.cylinder;
    let a1 = lens1.axis_deg.to_radians();
    let sin1 = a1.sin();
    let cos1 = a1.cos();
    let px1 = s1 + c1 * sin1 * sin1;
    let pt1 = -c1 * sin1 * cos1;
    let py1 = s1 + c1 * cos1 * cos1;

    // Power matrix components for lens 2
    let s2 = lens2.sphere;
    let c2 = lens2.cylinder;
    let a2 = lens2.axis_deg.to_radians();
    let sin2 = a2.sin();
    let cos2 = a2.cos();
    let px2 = s2 + c2 * sin2 * sin2;
    let pt2 = -c2 * sin2 * cos2;
    let py2 = s2 + c2 * cos2 * cos2;

    // Resultant matrix: simple sum of power matrices
    let px = px1 + px2;
    let pt = pt1 + pt2;
    let py = py1 + py2;

    // Invariants
    let trace = px + py;
    let determinant = (px * py) - (pt * pt);

    // Cylinder in minus-cylinder convention
    let delta = (trace * trace - 4.0 * determinant).max(0.0).sqrt();
    let cylinder = -delta;
    let sphere = (trace - cylinder) / 2.0;

    // Axis from matrix orientation; use atan2 for numerical robustness
    let axis_rad = (sphere - px).atan2(pt);
    let mut axis_deg = axis_rad.to_degrees();
    // Normalize to [0, 180)
    axis_deg = axis_deg % 180.0;
    if axis_deg < 0.0 {
        axis_deg += 180.0;
    }

    SpheroCyl {
        sphere,
        cylinder,
        axis_deg,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    fn matrix_from_lens(l: SpheroCyl) -> (f64, f64, f64) {
        let a = l.axis_deg.to_radians();
        let s = l.sphere;
        let c = l.cylinder;
        let sin = a.sin();
        let cos = a.cos();
        let px = s + c * sin * sin;
        let pt = -c * sin * cos;
        let py = s + c * cos * cos;
        (px, pt, py)
    }

    #[test]
    fn pure_spheres_add() {
        let l1 = SpheroCyl {
            sphere: 2.0,
            cylinder: 0.0,
            axis_deg: 0.0,
        };
        let l2 = SpheroCyl {
            sphere: -1.25,
            cylinder: 0.0,
            axis_deg: 90.0,
        };
        let r = crossed_cylinders(l1, l2);
        assert_abs_diff_eq!(r.sphere, 0.75, epsilon = 1e-12);
        assert_abs_diff_eq!(r.cylinder, 0.0, epsilon = 1e-12);
        // Axis arbitrary when cylinder = 0; we only check it's within range
        assert!(r.axis_deg >= 0.0 && r.axis_deg < 180.0);
    }

    #[test]
    fn aligned_minus_cylinders_add() {
        // −2.00 DC × 90 combined with −1.00 DC × 90 → −3.00 DC × 90, S=0
        let l1 = SpheroCyl {
            sphere: 0.0,
            cylinder: -2.0,
            axis_deg: 90.0,
        };
        let l2 = SpheroCyl {
            sphere: 0.0,
            cylinder: -1.0,
            axis_deg: 90.0,
        };
        let r = crossed_cylinders(l1, l2);
        assert_abs_diff_eq!(r.sphere, 0.0, epsilon = 1e-12);
        assert_abs_diff_eq!(r.cylinder, -3.0, epsilon = 1e-12);
        assert_abs_diff_eq!(r.axis_deg, 90.0, epsilon = 1e-12);
    }

    #[test]
    fn matrix_consistency_oblique_case() {
        let l1 = SpheroCyl {
            sphere: -4.0,
            cylinder: 2.0,
            axis_deg: 45.0,
        };
        let l2 = SpheroCyl {
            sphere: -5.0,
            cylinder: -3.0,
            axis_deg: 120.0,
        };

        let (px1, pt1, py1) = matrix_from_lens(l1);
        let (px2, pt2, py2) = matrix_from_lens(l2);
        let px_sum = px1 + px2;
        let pt_sum = pt1 + pt2;
        let py_sum = py1 + py2;

        let r = crossed_cylinders(l1, l2);
        let (pxr, ptr, pyr) = matrix_from_lens(r);

        assert_abs_diff_eq!(pxr, px_sum, epsilon = 1e-9);
        assert_abs_diff_eq!(ptr, pt_sum, epsilon = 1e-9);
        assert_abs_diff_eq!(pyr, py_sum, epsilon = 1e-9);
    }
}