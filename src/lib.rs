#![doc = include_str!("../README.md")]
mod convert_power;
mod crossed_cylinders;
mod oblique_meridian;
mod prism;

pub use convert_power::*;
pub use crossed_cylinders::*;
pub use oblique_meridian::*;
pub use prism::*;

/// Defines the eye.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Eye {
    /// Right eye.
    OD,

    /// Left eye.
    OS,
}

/// Defines a sphero-cylinder lens.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

/// Base direction for horizontal prism relative to the patient.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HorizontalBase {
    /// Base In (BI): prism base oriented toward the patient's nose.
    #[cfg_attr(feature = "serde", serde(rename = "BI"))]
    In,
    /// Base Out (BO): prism base oriented toward the patient's periphery.
    #[cfg_attr(feature = "serde", serde(rename = "BO"))]
    Out,
}

/// Base direction for vertical prism.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VerticalBase {
    /// Base Up (BU): prism base oriented upward.
    #[cfg_attr(feature = "serde", serde(rename = "BU"))]
    Up,
    /// Base Down (BD): prism base oriented downward.
    #[cfg_attr(feature = "serde", serde(rename = "BD"))]
    Down,
}

/// Represents horizontal prism in prism diopters, with an explicit base direction.
///
/// In clinical optometry and ophthalmology, horizontal prism is defined by:
/// - **Base In (BI):** base oriented toward the patient's nose (conventionally negative in signed form).
/// - **Base Out (BO):** base oriented toward the patient's temple (conventionally positive in signed form).
///
/// This type keeps the `amount` always **non-negative**. The sign is derived
/// only when calling [`signed`](HorizontalPrism::signed).
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HorizontalPrism {
    /// Magnitude of horizontal prism, measured in prism diopters (Δ).
    ///
    /// This value is always non-negative. The prism’s *direction* (in or out)
    /// is stored separately in [`base`](HorizontalPrism::base).
    amount: f64,

    /// Direction of the prism base relative to the patient:
    /// - [`HorizontalBase::In`] for base-in.
    /// - [`HorizontalBase::Out`] for base-out.
    base: HorizontalBase,
}

impl HorizontalPrism {
    /// Creates a new [`HorizontalPrism`] with the given amount and base direction.
    ///
    /// # Panics
    ///
    /// Panics in debug builds if `amount` is negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use opticalc::*;
    /// let prism = HorizontalPrism::new(2.0, HorizontalBase::Out);
    /// assert_eq!(prism.amount(), 2.0);
    /// assert_eq!(prism.signed(), 2.0); // base-out is positive
    /// ```
    pub fn new(amount: f64, base: HorizontalBase) -> Self {
        debug_assert!(amount >= 0.0, "amount must be non-negative");
        Self { amount, base }
    }

    /// Creates a new [`HorizontalPrism`] from a signed prism value.
    ///
    /// Conventions:
    /// - Negative values = **Base In (BI)**
    /// - Positive values = **Base Out (BO)**
    /// - Zero = 0Δ Base Out by default (arbitrary, but consistent)
    ///
    /// # Examples
    ///
    /// ```
    /// use opticalc::*;
    /// let prism = HorizontalPrism::from_signed(-3.0);
    /// assert_eq!(prism.amount(), 3.0);
    /// assert_eq!(prism.base(), Some(HorizontalBase::In));
    /// ```
    pub fn from_signed(value: f64) -> Self {
        if value < 0.0 {
            Self {
                amount: -value,
                base: HorizontalBase::In,
            }
        } else {
            Self {
                amount: value,
                base: HorizontalBase::Out,
            }
        }
    }

    /// Returns `true` if this prism has zero magnitude.
    ///
    /// A prism with `amount == 0.0` is considered equivalent to having
    /// no horizontal prism correction.
    pub fn is_none(&self) -> bool {
        // Check if the prism is zero, or within an f64 rounding error of zero
        self.amount == 0.0 || (self.amount - 0.0).abs() < f64::EPSILON
    }

    /// Returns the base direction, or `None` if the prism has zero magnitude.
    ///
    /// # Examples
    ///
    /// ```
    /// use opticalc::*;
    /// let prism = HorizontalPrism::new(0.0, HorizontalBase::In);
    /// assert_eq!(prism.base(), None);
    /// ```
    pub fn base(&self) -> Option<HorizontalBase> {
        if self.is_none() {
            return None;
        }

        Some(self.base)
    }

    /// Returns the absolute (non-negative) magnitude of the prism, in prism diopters.
    ///
    /// # Examples
    ///
    /// ```
    /// use opticalc::*;
    /// let prism = HorizontalPrism::new(3.0, HorizontalBase::In);
    /// assert_eq!(prism.amount(), 3.0);
    /// ```
    pub fn amount(&self) -> f64 {
        self.amount
    }

    /// Returns the signed magnitude of the prism in prism diopters.
    ///
    /// Conventions:
    /// - **Base In (BI):** negative value.
    /// - **Base Out (BO):** positive value.
    ///
    /// # Examples
    ///
    /// ```
    /// use opticalc::*;
    /// let prism_in = HorizontalPrism::new(2.0, HorizontalBase::In);
    /// assert_eq!(prism_in.signed(), -2.0);
    ///
    /// let prism_out = HorizontalPrism::new(2.0, HorizontalBase::Out);
    /// assert_eq!(prism_out.signed(), 2.0);
    /// ```
    pub fn signed(&self) -> f64 {
        match self.base {
            HorizontalBase::In => -self.amount,
            HorizontalBase::Out => self.amount,
        }
    }
}

/// Represents vertical prism in prism diopters, with an explicit base direction.
///
/// In clinical optometry and ophthalmology, vertical prism is defined by:
/// - **Base Up (BU):** base oriented upward (conventionally positive in signed form).
/// - **Base Down (BD):** base oriented downward (conventionally negative in signed form).
///
/// As with [`HorizontalPrism`], the `amount` is always stored as **non-negative**.  
/// The sign is derived only when calling [`signed`](VerticalPrism::signed).
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VerticalPrism {
    /// Magnitude of vertical prism, measured in prism diopters (Δ).
    ///
    /// This value is always non-negative. The prism’s *direction* (up or down)
    /// is stored separately in [`base`](VerticalPrism::base).
    amount: f64,

    /// Direction of the prism base relative to the patient:
    /// - [`VerticalBase::Up`] for base-up.
    /// - [`VerticalBase::Down`] for base-down.
    base: VerticalBase,
}

impl VerticalPrism {
    /// Creates a new [`VerticalPrism`] with the given amount and base direction.
    ///
    /// # Panics
    ///
    /// Panics in debug builds if `amount` is negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use opticalc::*;
    /// let prism = VerticalPrism::new(1.5, VerticalBase::Up);
    /// assert_eq!(prism.amount(), 1.5);
    /// assert_eq!(prism.signed(), 1.5); // base-up is positive
    /// ```
    pub fn new(amount: f64, base: VerticalBase) -> Self {
        debug_assert!(amount >= 0.0, "amount must be non-negative");
        Self { amount, base }
    }

    /// Creates a new [`VerticalPrism`] from a signed prism value.
    ///
    /// Conventions:
    /// - Positive values = **Base Up (BU)**
    /// - Negative values = **Base Down (BD)**
    /// - Zero = 0Δ Base Up by default (arbitrary, but consistent)
    ///
    /// # Examples
    ///
    /// ```
    /// use opticalc::*;
    /// let prism = VerticalPrism::from_signed(-2.0);
    /// assert_eq!(prism.amount(), 2.0);
    /// assert_eq!(prism.base(), Some(VerticalBase::Down));
    /// ```
    pub fn from_signed(value: f64) -> Self {
        if value < 0.0 {
            Self {
                amount: -value,
                base: VerticalBase::Down,
            }
        } else {
            Self {
                amount: value,
                base: VerticalBase::Up,
            }
        }
    }

    /// Returns `true` if this prism has zero magnitude.
    ///
    /// A prism with `amount == 0.0` is considered equivalent to having
    /// no vertical prism correction.
    pub fn is_none(&self) -> bool {
        self.amount == 0.0
    }

    /// Returns the base direction, or `None` if the prism has zero magnitude.
    ///
    /// # Examples
    ///
    /// ```
    /// use opticalc::*;
    /// let prism = VerticalPrism::new(0.0, VerticalBase::Down);
    /// assert_eq!(prism.base(), None);
    /// ```
    pub fn base(&self) -> Option<VerticalBase> {
        if self.is_none() {
            return None;
        }

        Some(self.base)
    }

    /// Returns the absolute (non-negative) magnitude of the prism, in prism diopters.
    ///
    /// # Examples
    ///
    /// ```
    /// use opticalc::*;
    /// let prism = VerticalPrism::new(2.5, VerticalBase::Down);
    /// assert_eq!(prism.amount(), 2.5);
    /// ```
    pub fn amount(&self) -> f64 {
        self.amount
    }

    /// Returns the signed magnitude of the prism in prism diopters.
    ///
    /// Conventions:
    /// - **Base Up (BU):** positive value.
    /// - **Base Down (BD):** negative value.
    ///
    /// # Examples
    ///
    /// ```
    /// use opticalc::*;
    /// let prism_up = VerticalPrism::new(2.0, VerticalBase::Up);
    /// assert_eq!(prism_up.signed(), 2.0);
    ///
    /// let prism_down = VerticalPrism::new(2.0, VerticalBase::Down);
    /// assert_eq!(prism_down.signed(), -2.0);
    /// ```
    pub fn signed(&self) -> f64 {
        match self.base {
            VerticalBase::Up => self.amount,
            VerticalBase::Down => -self.amount,
        }
    }
}

/// Represents combined horizontal and vertical prism components.
///
/// This type encapsulates both horizontal and vertical prism effects,
/// allowing calculation of the resultant prism magnitude.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CombinedPrism {
    /// Horizontal prism component.
    pub horizontal: HorizontalPrism,
    /// Vertical prism component.
    pub vertical: VerticalPrism,
}

impl CombinedPrism {
    /// Returns the resultant prism magnitude in prism diopters.
    ///
    /// This computes the vector magnitude of the combined horizontal
    /// and vertical prism components using the Pythagorean theorem.
    ///
    /// # Examples
    ///
    /// ```
    /// use opticalc::*;
    /// let prism = CombinedPrism {
    ///     horizontal: HorizontalPrism::new(3.0, HorizontalBase::Out),
    ///     vertical: VerticalPrism::new(4.0, VerticalBase::Up),
    /// };
    /// assert_eq!(prism.magnitude(), 5.0); // 3-4-5 triangle
    /// ```
    pub fn magnitude(&self) -> f64 {
        self.horizontal.amount().hypot(self.vertical.amount())
    }
}

/// Represents lens decentration relative to the optical center.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

