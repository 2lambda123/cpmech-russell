//! This module implements algorithms built from base, math, and vector-matrix routines

mod bracket_min;
mod common;
mod interp_lagrange;
mod min_solver_brent;
mod num_jacobian;
mod quadrature;
mod root_solver_brent;
mod testing;
pub use crate::algo::bracket_min::*;
pub use crate::algo::common::*;
pub use crate::algo::interp_lagrange::*;
pub use crate::algo::min_solver_brent::*;
pub use crate::algo::num_jacobian::*;
pub use crate::algo::quadrature::*;
pub use crate::algo::root_solver_brent::*;
pub use crate::algo::testing::*;
