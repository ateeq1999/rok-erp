//! The pieces every pharmacy board is built from, recovered from the boards in
//! `design/pharmacy`: a titled card, a stat row, a table with a header row, a
//! filter bar, and a two-column split with a detail rail.
//!
//! These are the shapes the boards repeat. A screen that draws its own copy of
//! one of them is the copy that drifts from the board, so the shape lives here
//! once and every screen fills it in with its own figures.
//!
//! A feature's presentation imports `crate::features::shared::board`, never
//! `crate::screens::board`. The fs-based boundary test in the feature registry
//! enforces
//! that until the screens module is gone.

pub mod board;
