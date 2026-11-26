use super::map::{ConfMap, ConfMapErr, MapUpdatablecells};
use crate::chart::{Action, Chart, GenErrItem, GenError, Outcome, Rejection, Report, ShipView, SpectatorView};
use geometry::{Cell, Map, Terrain};
use rand::seq::IndexedRandom;
use rand::{Rng, RngExt, SeedableRng};
use rand_chacha::ChaCha8Rng;
use shared::{IslandId, Params, Round, Seed, ShipId};
use std::collections::{BTreeMap, BTreeSet};

/// A reference [`Chart`]
#[derive(PartialEq, Debug)]
pub struct RefChart {
    map: ConfMap,
    round: Round,
    params: Params,
    treasure: Cell,
    w_seed: Seed,
}

/// A reference [`Chart::ValidatedAction`]
#[derive(Debug)]
pub enum RefValidatedAction {}

impl Chart for RefChart {
    type ValidatedAction = RefValidatedAction;

    /// Generate the archipelago, returns a [`GenError`] if some step fails
    ///
    /// # Panics
    ///
    /// # Errors
    ///
    /// See [`GenError`] definitions
    fn generate(seed: Seed, params: &Params) -> Result<Self, GenError> {
        // 1. Seed reproducible random generator from the generation stream (not wall clock).
        let mut rng: ChaCha8Rng = ChaCha8Rng::seed_from_u64(seed);

        let radius = params.archipelago_radius;

        // 2. Create sea hex grid of `archipelago_radius` . Reject if radius ≤ `vision_range`.
        if radius <= params.vision_range {
            return Err(GenError::VisionRangeTooHigh);
        }

        dbg!("Passed 2");

        let mut map = match Self::map_generate(radius) {
            Ok(val) => val,
            Err(error) => {
                println!("{error:?}");
                Self::empty_map()
            }
        };
        dbg!("Generated a map");

        // 3. Place `island_count` connected island footprints.
        Self::island_place(&mut rng, &mut map, params.island_count)?;
        dbg!("Placed islands");

        let retries = 3;

        // 4. Place `cliff_count` cliffs (must not touch islands or sea corners/adjacent cells)
        Self::cliff_place(&mut rng, &mut map, params.cliff_count, retries)?;
        dbg!("Placed cliffs");

        // 5. Place `rock_count` rocks on clear water (never on corners or adjacent cells).
        Self::rock_place(&mut rng, &mut map, params.rock_count, retries)?;
        dbg!("Placed rocks");

        // 6. Connectivity Check: Verify clear water is fully connected without crossing rocks
        if !Self::water_connected(
            &map,
            Cell {
                q: -radius.cast_signed(),
                r: 0,
            },
            &mut BTreeSet::new(),
        ) {
            return Err(GenError::WaterNotConnected);
        }
        dbg!("Water is connected");

        // 7. Treasure Placement: Select clear water cell not adjacent to land/corners. Retry if impossible.
        let treasure = Self::treasure_place(&mut rng, &map)?;
        dbg!("Treasure placed");
        // 8. Assign one CLUE to each Island (Section 4).
        // todo!();

        // 9. Solvability Check: Verify full clue intersection equals exactly 1 cell. Advance seed and retry if failed
        // todo!();

        Ok(Self {
            map,
            round: 0,
            params: *params,
            treasure,
            w_seed: rng.next_u64(),
        })
    }

    /// Decide whether an action is legal. The only way to obtain a `ValidatedAction`.
    ///
    /// # Errors
    ///
    /// See [`Rejection`] definitions
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
    fn cells_generate(radius: u32) -> Result<BTreeMap<Cell, Terrain>, ConfMapErr> {
        if !ConfMap::radius_bound(radius) {
            return Err(ConfMapErr::InvalidRadius);
        }

        let rad: i32 = radius.cast_signed();
        let neg_rad: i32 = -rad;

        let mut cells: BTreeMap<Cell, Terrain> = BTreeMap::new();

        for q in neg_rad..=rad {
            let r_min = neg_rad.max(neg_rad.saturating_sub(q));
            let r_max = rad.min(rad.saturating_sub(q));

            for r in r_min..=r_max {
                cells.insert(Cell { q, r }, Terrain::Clear);
            }
        }

        Ok(cells)
    }

    /// Empty [`ConfMap`], used to recover invalid `map_generate`
    fn empty_map() -> ConfMap {
        ConfMap::empty()
    }

    /// Generates a new [`ConfMap`]
    fn map_generate(radius: u32) -> Result<ConfMap, ConfMapErr> {
        let cells: BTreeMap<Cell, Terrain> = Self::cells_generate(radius)?;
        let map = ConfMap::init(radius, cells)?;

        Ok(map)
    }

    /// Place `param.cliff_count` [`Terrain::Cliff`]
    ///
    /// # Errors
    ///
    /// Returns [`GenError::CantPlace { item: GenErrorItem::Cliff }`] if cannot place any cliff after n-`retries`
    fn island_place<'a, R: Rng + ?Sized, M: Map + MapUpdatablecells>(
        rng: &mut R,
        map: &'a mut M,
        islands: u8,
    ) -> Result<&'a mut M, GenError> {
        let corners = map.corners();
        let mut excluded = BTreeSet::new();

        for c in corners {
            excluded.insert(c);
            excluded.extend(c.ring(1));
        }

        for _ in 0..islands {
            let candidates: Vec<Cell> = map.clear_cells().filter(|c| !excluded.contains(c)).collect();

            let island_cell = (0..1).find_map(|_| {
                let island_candidate = candidates.choose(rng)?;
                let footprint: u8 = rng.random_range(1..=3);

                let mut island_fp: Vec<Cell> = vec![];

                if footprint == 1 {
                    if Self::island_filter(*island_candidate, map, &island_fp) {
                        island_fp.push(*island_candidate);
                    }
                } else {
                    for _ in 2..=footprint {
                        if let Some(cell) = island_candidate
                            .ring(1)
                            .filter(|c| map.contains(*c))
                            .find(|c| Self::island_filter(*c, map, &island_fp))
                        {
                            island_fp.push(cell);
                        } else {
                            break;
                        }
                    }
                }

                Some(island_fp)
            });

            let Some(island_selected) = island_cell else {
                return Err(GenError::CantPlace {
                    item: GenErrItem::Island,
                });
            };

            for island in island_selected {
                let island_id = IslandId(rng.random());

                if let Err(error) = map.update_cell(island, Terrain::Island(island_id)) {
                    println!("{error:?}");
                    return Err(GenError::CantPlace {
                        item: GenErrItem::Island,
                    });
                }
            }
        }

        Ok(map)
    }

    /// Check for possible [`Island`] footprint candidate
    fn island_filter(candidate: Cell, map: &impl Map, excluded: &[Cell]) -> bool {
        candidate
            .ring(1)
            .filter(|c| map.contains(*c) && !excluded.contains(c))
            .all(|c| !map.is_land(c))
    }

    /// Place `param.cliff_count` [`Terrain::Cliff`]
    ///
    /// # Errors
    ///
    /// Returns [`GenError::CantPlace { item: GenErrorItem::Cliff }`] if cannot place any cliff after n-`retries`
    fn cliff_place<'a, R: Rng, M: Map + MapUpdatablecells>(
        rng: &mut R,
        map: &'a mut M,
        cliffs: u16,
        retries: u8,
    ) -> Result<&'a mut M, GenError> {
        let corners = map.corners();
        let mut excluded = BTreeSet::new();

        for c in corners {
            excluded.insert(c);
            excluded.extend(c.ring(1));
        }

        for _ in 0..cliffs {
            let candidates: Vec<Cell> = map.clear_cells().filter(|c| !excluded.contains(c)).collect();

            let cliff_cell = (0..retries).find_map(|_| {
                let cliff_candidate = candidates.choose(rng)?;

                cliff_candidate
                    .ring(1)
                    .filter(|c| map.contains(*c))
                    .all(|c| !map.is_land(c))
                    .then_some(cliff_candidate)
            });

            let Some(cliff_selected) = cliff_cell else {
                return Err(GenError::CantPlace {
                    item: GenErrItem::Cliff,
                });
            };

            if let Err(error) = map.update_cell(*cliff_selected, Terrain::Cliff) {
                println!("{error:?}");
                return Err(GenError::CantPlace {
                    item: GenErrItem::Cliff,
                });
            }
        }

        Ok(map)
    }

    /// Place `param.rock_count` [`Terrain::Rock`]
    ///
    /// # Errors
    ///
    /// Returns [`GenError::CantPlace { item: GenErrorItem::Rock }`] if cannot place any rock after n-`retries`
    fn rock_place<'a, R: Rng, M: Map + MapUpdatablecells>(
        rng: &mut R,
        map: &'a mut M,
        rocks: u16,
        retries: u8,
    ) -> Result<&'a mut M, GenError> {
        let corners = map.corners();
        let mut excluded = BTreeSet::new();

        for cell in corners {
            excluded.insert(cell);
            excluded.extend(cell.ring(1));
        }

        for _ in 0..rocks {
            let candidates: Vec<Cell> = map.clear_cells().filter(|c| !excluded.contains(c)).collect();

            let rock_cell = (0..retries).find_map(|_| {
                let rock_candidate = candidates.choose(rng)?;

                Some(rock_candidate)
            });

            let Some(rock_selected) = rock_cell else {
                return Err(GenError::CantPlace { item: GenErrItem::Rock });
            };

            if let Err(error) = map.update_cell(*rock_selected, Terrain::Rock) {
                println!("{error:?}");
                return Err(GenError::CantPlace { item: GenErrItem::Rock });
            }
        }

        Ok(map)
    }

    /// Checks wether water is connected
    fn water_connected<M: Map>(map: &M, start_cell: Cell, visited: &mut BTreeSet<Cell>) -> bool {
        if map.is_land(start_cell) {
            return false;
        }

        if !visited.insert(start_cell) {
            return true;
        }

        start_cell
            .ring(1)
            .filter(|c| map.contains(*c) && !map.is_land(*c) && map.terrain(*c).is_some_and(|t| t == Terrain::Rock))
            .all(|c| Self::water_connected(map, c, visited))
    }

    /// Finds a cell for the treasure
    ///
    /// # Errors
    ///
    /// Returns [`GenError::CantPlace { item: GenErrorItem::Treasure }`] if a cell cannot be found
    fn treasure_place<R: Rng, M: Map>(rng: &mut R, map: &M) -> Result<Cell, GenError> {
        let corners = map.corners();
        let candidates: Vec<Cell> = map.clear_cells().filter(|c| !corners.contains(c)).collect();

        let treasure_cell = (0..1).find_map(|_| {
            let treasure_candidate = candidates.choose(rng)?;

            treasure_candidate
                .ring(1)
                .filter(|c| map.contains(*c))
                .all(|c| !map.is_land(c))
                .then_some(treasure_candidate)
        });

        let Some(treasure_selected) = treasure_cell else {
            return Err(GenError::CantPlace {
                item: GenErrItem::Treasure,
            });
        };

        Ok(*treasure_selected)
    }
}
