use std::time::Duration;

use bevy::{
    reflect::Reflect,
    time::{Timer, TimerMode},
};

/// An enumeration representing various reasons for despawning an entity.
///
/// The [`DespawnReason`] enum is used to indicate different conditions or events that lead to the despawning of an entity.
#[derive(Debug, Clone, PartialEq, Reflect)]
pub enum DespawnReason {
    /// Indicates that the entity was forcefully despawned. After been removed if object must respawn ([`Respawn`](crate::component::Respawn))
    Forced,
    /// Specifies that the entity was despawned because it exceeded a certain value along a specific axis.
    More(f32, AxisName),
    /// Specifies that the entity was despawned because it fell below a certain value along a specific axis.
    Less(f32, AxisName),
    /// Specifies that the entity was despawned after timeout.
    After(DespawnTimer),
}

/// A timer used to despawn an entity after a certain amount of time.
#[derive(Debug, Clone, PartialEq, Reflect)]
pub struct DespawnTimer(Timer);

impl DespawnTimer {
    /// Creates a new [`DespawnTimer`] with the specified duration.
    pub fn new(duration: f32) -> Self {
        Self(Timer::from_seconds(duration, TimerMode::Repeating))
    }

    /// Updates the timer.
    pub fn update(&mut self, delta: Duration) -> &mut Self {
        self.0.tick(delta);
        self
    }

    /// Returns `true` if the timer has finished.
    pub fn just_finished(&self) -> bool {
        self.0.just_finished()
    }
}

/// An enumeration representing axis names.
///
/// The [`AxisName`] enum is used to specify the names of different axes in 3D space.
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub enum AxisName {
    X,
    Y,
    Z,
}

pub trait IntoDespawnTypeVec {
    fn into_despawn_type_vec(self) -> Vec<DespawnReason>;
}

impl IntoDespawnTypeVec for DespawnReason {
    fn into_despawn_type_vec(self) -> Vec<DespawnReason> {
        vec![self]
    }
}

impl<A: Into<DespawnReason>, B: Into<DespawnReason>> IntoDespawnTypeVec for (A, B) {
    fn into_despawn_type_vec(self) -> Vec<DespawnReason> {
        vec![self.0.into(), self.1.into()]
    }
}

impl<A: Into<DespawnReason>, B: Into<DespawnReason>, C: Into<DespawnReason>> IntoDespawnTypeVec
    for (A, B, C)
{
    fn into_despawn_type_vec(self) -> Vec<DespawnReason> {
        vec![self.0.into(), self.1.into(), self.2.into()]
    }
}

impl<
        A: Into<DespawnReason>,
        B: Into<DespawnReason>,
        C: Into<DespawnReason>,
        D: Into<DespawnReason>,
    > IntoDespawnTypeVec for (A, B, C, D)
{
    fn into_despawn_type_vec(self) -> Vec<DespawnReason> {
        vec![self.0.into(), self.1.into(), self.2.into(), self.3.into()]
    }
}

impl<
        A: Into<DespawnReason>,
        B: Into<DespawnReason>,
        C: Into<DespawnReason>,
        D: Into<DespawnReason>,
        E: Into<DespawnReason>,
    > IntoDespawnTypeVec for (A, B, C, D, E)
{
    fn into_despawn_type_vec(self) -> Vec<DespawnReason> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
        ]
    }
}

impl<
        A: Into<DespawnReason>,
        B: Into<DespawnReason>,
        C: Into<DespawnReason>,
        D: Into<DespawnReason>,
        E: Into<DespawnReason>,
        F: Into<DespawnReason>,
    > IntoDespawnTypeVec for (A, B, C, D, E, F)
{
    fn into_despawn_type_vec(self) -> Vec<DespawnReason> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
        ]
    }
}

impl<
        A: Into<DespawnReason>,
        B: Into<DespawnReason>,
        C: Into<DespawnReason>,
        D: Into<DespawnReason>,
        E: Into<DespawnReason>,
        F: Into<DespawnReason>,
        G: Into<DespawnReason>,
    > IntoDespawnTypeVec for (A, B, C, D, E, F, G)
{
    fn into_despawn_type_vec(self) -> Vec<DespawnReason> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
        ]
    }
}

impl<
        A: Into<DespawnReason>,
        B: Into<DespawnReason>,
        C: Into<DespawnReason>,
        D: Into<DespawnReason>,
        E: Into<DespawnReason>,
        F: Into<DespawnReason>,
        G: Into<DespawnReason>,
        H: Into<DespawnReason>,
    > IntoDespawnTypeVec for (A, B, C, D, E, F, G, H)
{
    fn into_despawn_type_vec(self) -> Vec<DespawnReason> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
        ]
    }
}

impl<
        A: Into<DespawnReason>,
        B: Into<DespawnReason>,
        C: Into<DespawnReason>,
        D: Into<DespawnReason>,
        E: Into<DespawnReason>,
        F: Into<DespawnReason>,
        G: Into<DespawnReason>,
        H: Into<DespawnReason>,
        I: Into<DespawnReason>,
    > IntoDespawnTypeVec for (A, B, C, D, E, F, G, H, I)
{
    fn into_despawn_type_vec(self) -> Vec<DespawnReason> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
        ]
    }
}

impl<
        A: Into<DespawnReason>,
        B: Into<DespawnReason>,
        C: Into<DespawnReason>,
        D: Into<DespawnReason>,
        E: Into<DespawnReason>,
        F: Into<DespawnReason>,
        G: Into<DespawnReason>,
        H: Into<DespawnReason>,
        I: Into<DespawnReason>,
        J: Into<DespawnReason>,
    > IntoDespawnTypeVec for (A, B, C, D, E, F, G, H, I, J)
{
    fn into_despawn_type_vec(self) -> Vec<DespawnReason> {
        vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
            self.8.into(),
            self.9.into(),
        ]
    }
}
