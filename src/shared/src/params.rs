use crate::Round;

/// Configuration parameters for an Archipelago game.
///
/// Contains the numeric rules used to configure the game.
/// This struct is non-exhaustive, allowing future parameters
/// to be added without breaking external code.
///
/// The number of Ships is fixed at six.
///
/// Field values and combinations are not validated by this type.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Params {
    /// Radius of the archipelago in hex-grid steps.
    pub archipelago_radius: u32,

    /// Number of islands.
    pub island_count: u8,

    /// Number of cliff cells, limited to the `u16` range.
    pub cliff_count: u16,

    /// Number of rock cells, limited to the `u16` range.
    pub rock_count: u16,

    /// Maximum distance at which a Ship can see, in hex-grid steps.
    pub vision_range: u32,

    /// Maximum distance at which a Ship can bombard, in hex-grid steps.
    pub bombard_range: u32,

    /// Damage dealt by a bombardment.
    pub bombard_damage: u32,

    /// Hull strength of a Ship.
    pub hull_strength: u32,

    /// Maximum number of game rounds.
    pub round_cap: Round,

    /// Time limit for a round, in milliseconds.
    pub round_deadline_ms: u32,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            archipelago_radius: 12,
            island_count: 8,
            cliff_count: 6,
            rock_count: 12,
            vision_range: 6,
            bombard_range: 2,
            bombard_damage: 1,
            hull_strength: 3,
            round_cap: 200,
            round_deadline_ms: 500,
        }
    }
}

impl Params {
    /// Returns a configuration with the specified archipelago radius.
    #[must_use]
    pub fn with_archipelago_radius(mut self, radius: u32) -> Self {
        self.archipelago_radius = radius;
        self
    }

    /// Returns a configuration with the specified island count.
    #[must_use]
    pub fn with_island_count(mut self, count: u8) -> Self {
        self.island_count = count;
        self
    }

    /// Returns a configuration with the specified cliff count.
    #[must_use]
    pub fn with_cliff_count(mut self, count: u16) -> Self {
        self.cliff_count = count;
        self
    }

    /// Returns a configuration with the specified rock count.
    #[must_use]
    pub fn with_rock_count(mut self, count: u16) -> Self {
        self.rock_count = count;
        self
    }

    /// Returns a configuration with the specified vision range.
    #[must_use]
    pub fn with_vision_range(mut self, range: u32) -> Self {
        self.vision_range = range;
        self
    }

    /// Returns a configuration with the specified bombard range.
    #[must_use]
    pub fn with_bombard_range(mut self, range: u32) -> Self {
        self.bombard_range = range;
        self
    }

    /// Returns a configuration with the specified bombard damage.
    #[must_use]
    pub fn with_bombard_damage(mut self, damage: u32) -> Self {
        self.bombard_damage = damage;
        self
    }

    /// Returns a configuration with the specified hull strength.
    #[must_use]
    pub fn with_hull_strength(mut self, strength: u32) -> Self {
        self.hull_strength = strength;
        self
    }

    /// Returns a configuration with the specified round cap.
    #[must_use]
    pub fn with_round_cap(mut self, cap: Round) -> Self {
        self.round_cap = cap;
        self
    }

    /// Returns a configuration with the specified round deadline in milliseconds.
    #[must_use]
    pub fn with_round_deadline_ms(mut self, deadline_ms: u32) -> Self {
        self.round_deadline_ms = deadline_ms;
        self
    }

    /// Returns the damage dealt when a Ship enters a rock cell.
    ///
    /// Rock damage is half the hull strength, rounded up,
    /// according to Main 2.2.
    #[must_use]
    pub fn rock_damage(&self) -> u32 {
        self.hull_strength.div_ceil(2)
    }
}
