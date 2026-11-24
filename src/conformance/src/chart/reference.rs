use super::{Action, Chart, GenError, Outcome, Rejection, Report, ShipView, SpectatorView};
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

#[derive(Debug, PartialEq)]
/// a
pub struct RicMap {
    radius: u32,
    cells: BTreeMap<Cell, Terrain>,
}

impl Map for RicMap {
    fn radius(&self) -> u32 {
        self.radius
    }

    fn contains(&self, cell: Cell) -> bool {
        if let Some(_) = self.cells.get(&cell) {
            true
        } else {
            false
        }
    }

    fn terrain(&self, cell: Cell) -> Option<Terrain> {
        if let Some(terr) = self.cells.get(&cell) {
            Some(terr.clone())
        } else {
            None
        }
    }

    fn is_land(&self, cell: Cell) -> bool {
        self.terrain(cell).is_some_and(|t| t.is_land())
    }
    fn is_water(&self, cell: Cell) -> bool {
        self.terrain(cell).is_some_and(|t| t.is_water())
    }
    /// The six corners, in direction order (counter-clockwise, East is the first). Ship i starts on corner i.
    fn corners(&self) -> [Cell; 6] {
        let rad = self.radius().cast_signed();
        [
            Cell { q: rad, r: 0 },
            Cell { q: rad, r: -rad },
            Cell { q: 0, r: -rad },
            Cell { q: -rad, r: 0 },
            Cell { q: -rad, r: rad },
            Cell { q: 0, r: rad },
        ]
    }
    /// Every cell in the sea, in a fixed order. Clear water and rocks.
    fn water_cells(&self) -> impl Iterator<Item = Cell> + '_ {
        Cell { q: 0, r: 0 }
            .disc(self.radius())
            .filter(|&cell| self.is_water(cell))
    }
    /// Water that is not a rock.
    fn clear_cells(&self) -> impl Iterator<Item = Cell> + '_ {
        Cell { q: 0, r: 0 }.disc(self.radius()).filter(|&cell| {
            if let Some(terr) = self.cells.get(&cell) {
                *terr == Terrain::Clear
            } else {
                false
            }
        })
    }
}

impl RicMap {
    /// a
    pub fn init(radius: u32, cells: BTreeMap<Cell, Terrain>) -> Self {
        Self { radius, cells }
    }
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
            params: params.clone(),
            w_seed: rng.next_u64(),
        })
    }

    /// Decide whether an action is legal. The only way to obtain a ValidatedAction.
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
        self.params.clone()
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
    /// Generate a BTreeMap of <Cell,Terrain> cells
    fn cells_generate(radius: u32) -> BTreeMap<Cell, Terrain> {
        let rad: i64 = radius as i64;
        let neg_rad: i64 = -rad;

        let mut cells: BTreeMap<Cell, Terrain> = BTreeMap::new();

        for q in neg_rad..=rad {
            let r_min = neg_rad.max(neg_rad.saturating_sub(q));
            let r_max = rad.min(rad.saturating_sub(q));

            for r in r_min..=r_max {
                cells.insert(
                    Cell {
                        q: q as i32,
                        r: r as i32,
                    },
                    Terrain::Clear,
                );
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
