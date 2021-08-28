//! Russell - Rust Scientific Library
//!
//! **sparse**: Sparse matrix tools and solvers

/// Returns package description
pub fn desc() -> String {
    "Sparse matrix tools and solvers".to_string()
}

mod sparse_triplet;
pub use crate::sparse_triplet::*;
