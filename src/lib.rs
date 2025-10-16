
pub mod prism;
pub mod crossed_cylinders;
pub mod oblique_meridian;
pub mod convert_power;

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

impl SpheroCyl {
    /// Compute meridional power at meridian φ (in degrees).
    /// - φ=0° → horizontal meridian.
    /// - φ=90° → vertical meridian.
    pub fn power_at(&self, phi_deg: f64) -> f64 {
        let delta = (phi_deg - self.axis_deg).to_radians();
        self.sphere + self.cylinder * (delta.sin().powi(2))
    }
}