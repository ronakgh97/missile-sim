use crate::entity::{Missile, Target};
use nalgebra::Vector3;

/// A guidance law for missile-target engagement.
///
/// Implement this trait to create custom guidance algorithms.
/// The library provides six built-in implementations:
///
/// - [`PureProportionalNavigation`] (PPN)
/// - [`TrueProportionalNavigation`] (TPN)
/// - [`AugmentedProportionalNavigation`] (APN)
/// - [`PurePursuit`] (PP)
/// - [`DeviatedPursuit`] (DP)
/// - [`LeadPursuit`] (LP)
///
/// # Example
///
/// ```
/// use missile_sim::prelude::*;
/// use nalgebra::Vector3;
///
/// struct DirectAim;
///
/// impl GuidanceLaw for DirectAim {
///     fn calculate_acceleration(&self, missile: &Missile, target: &Target) -> Vector3<f64> {
///         (target.state.position - missile.state.position).normalize() * missile.max_acceleration
///     }
///
///     fn name(&self) -> &str {
///         "DirectAim"
///     }
/// }
/// ```
pub trait GuidanceLaw: Send + Sync {
    /// Computes the acceleration command for the missile.
    ///
    /// # Arguments
    ///
    /// * `missile` — Current missile state and parameters.
    /// * `target` — Current target state and parameters.
    ///
    /// # Returns
    ///
    /// A 3D acceleration vector (m/s²). The simulation engine will clamp
    /// this to the missile's `max_acceleration`.
    fn calculate_acceleration(&self, missile: &Missile, target: &Target) -> Vector3<f64>;

    /// Returns a human-readable name for this guidance law.
    fn name(&self) -> &str;
}
