//! This module contains functions for calculations with vectors

mod aliases;
mod complex_vec_add;
mod complex_vec_approx_eq;
mod complex_vec_copy;
mod complex_vec_norm;
mod complex_vec_scale;
mod complex_vec_unzip;
mod complex_vec_update;
mod complex_vec_zip;
mod num_vector;
mod vec_add;
mod vec_all_finite;
mod vec_approx_eq;
mod vec_copy;
mod vec_inner;
mod vec_max_abs_diff;
mod vec_max_scaled;
mod vec_norm;
mod vec_rms_scaled;
mod vec_scale;
mod vec_update;
pub use crate::vector::aliases::*;
pub use crate::vector::complex_vec_add::*;
pub use crate::vector::complex_vec_approx_eq::*;
pub use crate::vector::complex_vec_copy::*;
pub use crate::vector::complex_vec_norm::*;
pub use crate::vector::complex_vec_scale::*;
pub use crate::vector::complex_vec_unzip::*;
pub use crate::vector::complex_vec_update::*;
pub use crate::vector::complex_vec_zip::*;
pub use crate::vector::num_vector::*;
pub use crate::vector::vec_add::*;
pub use crate::vector::vec_all_finite::*;
pub use crate::vector::vec_approx_eq::*;
pub use crate::vector::vec_copy::*;
pub use crate::vector::vec_inner::*;
pub use crate::vector::vec_max_abs_diff::*;
pub use crate::vector::vec_max_scaled::*;
pub use crate::vector::vec_norm::*;
pub use crate::vector::vec_rms_scaled::*;
pub use crate::vector::vec_scale::*;
pub use crate::vector::vec_update::*;
