use super::map::RicMap;
use crate::chart::{Action, Chart, GenError, Outcome, Rejection, Report, ShipView, SpectatorView};
use geometry::{Cell, Map, Terrain};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use shared::{Params, Round, Seed, ShipId};
use std::collections::BTreeMap;

/// A reference [`Chart`]
#[derive(PartialEq, Debug)]
pub struct RefChart {
    map: RicMap,
    round: Round,
    params: Params,
    w_seed: Seed,
}

/// A reference [`Chart::ValidatedAction`]
#[derive(Debug)]
pub enum RefValidatedAction {}

impl Chart for RefChart {
    type ValidatedAction = RefValidatedAction;

    /// Generate the archipelago, returns a [`GenError`] if some step fails
    fn generate(seed: Seed, params: &Params) -> Result<Self, GenError> {
        // todo!();

        let mut rng: ChaCha8Rng = ChaCha8Rng::seed_from_u64(seed);

        let map = Self::map_generate(params.archipelago_radius);

        Ok(Self {
            map,
            round: 0,
            params: *params,
            w_seed: rng.next_u64(),
        })
    }

    /// Decide whether an action is legal. The only way to obtain a `ValidatedAction`.
    fn validate(&self, ship: ShipId, action: &Action) -> Result<Self::ValidatedAction, Rejection> {
        todo!()
    }

    /// Apply an already validated action. Cannot fail.
    fn apply(&mut self, action: Self::ValidatedAction) -> Vec<Report> {
        todo!()
    }

    /// End of round: the round number advances and the Islands take their turn.
    fn advance_round(&mut self) -> Vec<Report> {
        self.round += 1;
        todo!()
    }

    /// What a Ship is permitted to see.
    fn view_for(&self, ship: ShipId) -> Option<ShipView> {
        todo!()
    }

    /// What a spectator sees: everything.
    fn spectator_view(&self) -> SpectatorView {
        todo!()
    }

    /// Outcome of the game
    fn outcome(&self) -> Option<Outcome> {
        todo!()
    }

    /// Parameters passed at [`Chart::generate()`]
    fn params(&self) -> Params {
        self.params
    }

    /// The [`Chart`] [`Map`]
    fn map(&self) -> &impl Map {
        &self.map
    }

    /// The weather [`Seed`]
    fn weather_seed(&self) -> Seed {
        self.w_seed
    }

    /// The current [`Round`]
    fn round(&self) -> Round {
        self.round
    }
}

impl RefChart {
    /// Generate a `BTreeMap<Cell,Terrain>` cells
    fn cells_generate(radius: u32) -> BTreeMap<Cell, Terrain> {
        if radius > (i32::MAX / 2) as u32 {
            // prevents overflow in start(), where operations like R - q can reach 2 * R
            // return Err(Error::ErrRadius);
        }

        let rad: i32 = radius as i32; // i32::try_from(radius)?;
        let neg_rad: i32 = -rad;

        let mut cells: BTreeMap<Cell, Terrain> = BTreeMap::new();

        for q in neg_rad..=rad {
            let r_min = neg_rad.max(neg_rad.saturating_sub(q));
            let r_max = rad.min(rad.saturating_sub(q));

            for r in r_min..=r_max {
                cells.insert(Cell { q, r }, Terrain::Clear);
            }
        }

        cells
    }

    fn map_generate(radius: u32) -> RicMap {
        let cells: BTreeMap<Cell, Terrain> = Self::cells_generate(radius);
        let map = RicMap::init(radius, cells);

        map
    }
}
