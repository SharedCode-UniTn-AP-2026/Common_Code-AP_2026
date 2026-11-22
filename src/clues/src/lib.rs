#![deny(unsafe_code)]
#![deny(clippy::pedantic)]
// TODO: docs
// #![deny(missing_docs)]

use std::fmt::Display;

use crate::Bound::{AtLeast, AtMost};

use geometry::{Cell, Map};

// TODO: logs

#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Clue {
    // No further than ‘radius‘ from ‘centre‘.
    Within { centre: Cell, radius: u32 },
    // Further than ‘radius‘ from ‘centre‘.
    Beyond { centre: Cell, radius: u32 },
    // The coordinate on ‘axis‘ satisfies ‘bound‘.
    // The clue is atleast or atmost n on that axis
    HalfPlane { axis: Axis, bound: Bound },
    // Strictly closer to ‘a‘ than to ‘b‘.
    Nearer { a: Cell, b: Cell },
}
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Axis {
    Q,
    R,
    S,
}

impl Display for Axis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Axis::Q => "Q",
            Axis::R => "R",
            Axis::S => "S",
        };

        write!(f, "{str}")
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Bound {
    AtLeast(i32),
    AtMost(i32),
}

impl Display for Bound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Bound::AtLeast(n) => format!("at least {n}"),
            Bound::AtMost(n) => format!("at most {n}"),
        };

        write!(f, "{str}")
    }
}

impl Bound {
    // if the overflow was to happen -> saturate i32
    fn relax(self, amount: u32) -> Bound {
        let a = i32::try_from(amount).unwrap_or(i32::MAX);
        match self {
            AtLeast(n) => AtLeast(n.saturating_sub(a)),
            AtMost(n) => AtMost(n.saturating_add(a)),
        }
    }
}

impl Clue {
    // True when this clue is consistent with the treasure lying in ‘cell‘.
    #[must_use]
    pub fn holds(self, cell: Cell) -> bool {
        match self {
            Clue::Within { centre, radius } => cell.distance(centre) <= radius,
            Clue::Beyond { centre, radius } => cell.distance(centre) > radius,
            Clue::HalfPlane { axis, bound } => {
                let v = match axis {
                    Axis::Q => cell.q,
                    Axis::R => cell.r,
                    Axis::S => cell.s(),
                };
                match bound {
                    Bound::AtLeast(k) => v >= k,
                    Bound::AtMost(k) => v <= k,
                }
            }
            Clue::Nearer { a, b } => cell.distance(a) < cell.distance(b),
        }
    }

    /// # Errors
    pub fn weaken(self, map: &impl Map, times: u32) -> Result<Clue, String> {
        match self {
            Clue::Within { centre, radius } => Ok(Clue::Within {
                centre,
                radius: radius.saturating_add(times), // sets the radius to u32::MAX if overflowed
            }),
            Clue::Beyond { centre, radius } => Ok(Clue::Beyond {
                centre,
                radius: radius.saturating_sub(times), // sets the radius to u32::MIN if underflowed
            }),
            Clue::HalfPlane { axis, bound } => Ok(Clue::HalfPlane {
                axis,
                bound: bound.relax(times),
            }),
            Clue::Nearer { a, b } => {
                // if times is 0 or 1 -> return self, nothing to weaken
                if times < 2 {
                    return Ok(self);
                }

                // if a == b the clue cannot be weakened -> return self
                if a == b {
                    return Ok(self);
                }

                // upcast to i64 for safer arithmetics here
                // since u32 reaches values a lot bigger than i32 does
                // no panics here is deliberate, since an island may try to weaken a clue more than
                // once and accidently go outside the map
                let k = i64::from(times);
                let dir_q = i64::from(b.q) - i64::from(a.q);
                let dir_r = i64::from(b.r) - i64::from(a.r);

                let new_q = i64::from(a.q) + k * dir_q;
                let new_r = i64::from(a.r) + k * dir_r;

                let (Ok(q), Ok(r)) = (i32::try_from(new_q), i32::try_from(new_r)) else {
                    return Err("Nearer: new cell is outside the map".into());
                };

                let new_b = Cell { q, r };
                if !map.contains(new_b) {
                    return Err("Nearer: new cell is outside the map".into());
                }

                Ok(Clue::Nearer { a, b: new_b })
            }
        }
    }
    // returns true if self is weaker than other
    // if self is atleast as strong as other -> returns false
    // returns Err if the two clues are of different types
    // useful for ships that will be trying to figure out if the clue they received is a weaker than
    // a previous one
    /// todo
    /// # Errors
    ///
    pub fn is_weaker_than(self, other: Clue) -> Result<bool, String> {
        match (self, other) {
            //Two within clues
            (Clue::Within { centre: c1, radius: r1 }, Clue::Within { centre: c2, radius: r2 }) if c1 == c2 => {
                Ok(r1 > r2)
            }

            //Two beyond clues
            (Clue::Beyond { centre: c1, radius: r1 }, Clue::Beyond { centre: c2, radius: r2 }) if c1 == c2 => {
                Ok(r1 < r2)
            }

            //Two halfplane clues
            (Clue::HalfPlane { axis: a1, bound: b1 }, Clue::HalfPlane { axis: a2, bound: b2 }) if a1 == a2 => {
                match (b1, b2) {
                    (AtLeast(first), AtLeast(second)) => Ok(first < second),
                    (AtMost(first), AtMost(second)) => Ok(first > second),
                    _ => Err("The two Clues are not compatible".into()),
                }
            }

            //Two nearer clues
            (Clue::Nearer { a: a1, b: b1 }, Clue::Nearer { a: a2, b: b2 }) if a1 == a2 => {
                Ok(a1.distance(b1) > a1.distance(b2))
            }

            _ => Err("can't compare the two clues".into()),
        }
    }
}

impl Display for Clue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Clue::Within { centre, radius } => format!(
                "The treasure is within {} units of ({},{},{}) cell",
                radius,
                centre.q,
                centre.r,
                centre.s(),
            ),
            Clue::Beyond { centre, radius } => format!(
                "The treasure is beyond {} units from the ({},{},{}) cell",
                radius,
                centre.q,
                centre.r,
                centre.s(),
            ),
            Clue::HalfPlane { axis, bound } => {
                format!("The treasure is {bound} on the {axis} axis")
            }
            Clue::Nearer { a, b } => format!(
                "The treasure is nearer to ({}, {}, {}) than to ({}, {}, {})",
                a.q,
                a.r,
                a.s(),
                b.q,
                b.r,
                b.s()
            ),
        };

        write!(f, "{str}")
    }
}

// TODO: tests
#[cfg(test)]
pub mod tests;
