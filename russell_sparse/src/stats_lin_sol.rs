use super::{StatsLinSolMUMPS, VerifyLinSys};
use russell_lab::format_nanoseconds;
use russell_openblas::get_num_threads;
use serde::{Deserialize, Serialize};
use serde_json;
use std::path::Path;

/// Holds the main information such as platform
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StatsLinSolMain {
    pub platform: String,
    pub blas_lib: String,
}

/// Holds information about the sparse matrix
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StatsLinSolMatrix {
    pub name: String,
    pub nrow: usize,
    pub ncol: usize,
    pub nnz: usize,
    pub symmetry: String,
}

/// Holds information about the linear solver (engine/genie)
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StatsLinSolSolver {
    pub name: String,
    pub version: String,
}

/// Holds some requested parameters
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StatsLinSolRequests {
    pub ordering: String,
    pub scaling: String,
    pub mumps_openmp_num_threads: usize,
}

/// Holds some output parameters
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StatsLinSolOutput {
    pub effective_ordering: String,
    pub effective_scaling: String,
    pub openmp_num_threads: i32,
    pub umfpack_strategy: String,
    pub umfpack_rcond_estimate: f64, // reciprocal condition number estimate
}

/// Holds the determinant of the coefficient matrix (if requested)
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StatsLinSolDeterminant {
    // det = mantissa * pow(base, exponent)
    pub mantissa: f64,
    pub base: f64,
    pub exponent: f64,
}

/// Holds the computer times in human readable format (post-processed)
///
/// **Note:** These are automatically converted from TimeNanoseconds when calling [StatsLinSol::get_json]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StatsLinSolTimeHuman {
    pub read_matrix: String,
    pub factorize: String,
    pub solve: String,
    pub total_f_and_s: String, // solve + factorize
    pub verify: String,
}

/// Holds the computer times in nanoseconds
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StatsLinSolTimeNanoseconds {
    pub read_matrix: u128,
    pub factorize: u128,
    pub solve: u128,
    pub total_f_and_s: u128, // solve + factorize
    pub verify: u128,
}

/// Holds information about the solution of a linear system
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StatsLinSol {
    pub main: StatsLinSolMain,
    pub matrix: StatsLinSolMatrix,
    pub solver: StatsLinSolSolver,
    pub requests: StatsLinSolRequests,
    pub output: StatsLinSolOutput,
    pub determinant: StatsLinSolDeterminant,
    pub verify: VerifyLinSys,
    pub time_human: StatsLinSolTimeHuman,
    pub time_nanoseconds: StatsLinSolTimeNanoseconds,
    pub mumps_stats: StatsLinSolMUMPS,
}

impl StatsLinSol {
    /// Allocates a blank structure
    pub fn new() -> Self {
        let unknown = "Unknown".to_string();
        StatsLinSol {
            main: StatsLinSolMain {
                platform: "Russell".to_string(),
                blas_lib: "OpenBLAS".to_string(),
            },
            matrix: StatsLinSolMatrix {
                name: unknown.clone(),
                nrow: 0,
                ncol: 0,
                nnz: 0,
                symmetry: unknown.clone(),
            },
            solver: StatsLinSolSolver {
                name: unknown.clone(),
                version: unknown.clone(),
            },
            requests: StatsLinSolRequests {
                ordering: unknown.clone(),
                scaling: unknown.clone(),
                mumps_openmp_num_threads: 0,
            },
            output: StatsLinSolOutput {
                effective_ordering: unknown.clone(),
                effective_scaling: unknown.clone(),
                openmp_num_threads: 0,
                umfpack_strategy: unknown.clone(),
                umfpack_rcond_estimate: 0.0,
            },
            determinant: StatsLinSolDeterminant {
                mantissa: 0.0,
                base: 0.0,
                exponent: 0.0,
            },
            verify: VerifyLinSys {
                max_abs_a: 0.0,
                max_abs_ax: 0.0,
                max_abs_diff: 0.0,
                relative_error: 0.0,
            },
            time_human: StatsLinSolTimeHuman {
                read_matrix: String::new(),
                factorize: String::new(),
                solve: String::new(),
                total_f_and_s: String::new(),
                verify: String::new(),
            },
            time_nanoseconds: StatsLinSolTimeNanoseconds {
                read_matrix: 0,
                factorize: 0,
                solve: 0,
                total_f_and_s: 0,
                verify: 0,
            },
            mumps_stats: StatsLinSolMUMPS {
                inf_norm_a: 0.0,
                inf_norm_x: 0.0,
                scaled_residual: 0.0,
                backward_error_omega1: 0.0,
                backward_error_omega2: 0.0,
                normalized_delta_x: 0.0,
                condition_number1: 0.0,
                condition_number2: 0.0,
            },
        }
    }

    /// Sets the matrix name as the stem of a file path
    pub fn set_matrix_name_from_path(&mut self, filepath: &str) {
        self.matrix.name = match Path::new(filepath).file_stem() {
            Some(v) => match v.to_str() {
                Some(w) => w.to_string(),
                None => "Unknown".to_string(),
            },
            None => "Unknown".to_string(),
        };
    }

    /// Gets a JSON representation of the stats structure
    pub fn get_json(&mut self) -> String {
        self.output.openmp_num_threads = get_num_threads();
        self.time_nanoseconds.total_f_and_s = self.time_nanoseconds.factorize + self.time_nanoseconds.solve;
        self.time_human.read_matrix = format_nanoseconds(self.time_nanoseconds.read_matrix);
        self.time_human.factorize = format_nanoseconds(self.time_nanoseconds.factorize);
        self.time_human.solve = format_nanoseconds(self.time_nanoseconds.solve);
        self.time_human.total_f_and_s = format_nanoseconds(self.time_nanoseconds.total_f_and_s);
        self.time_human.verify = format_nanoseconds(self.time_nanoseconds.verify);
        serde_json::to_string_pretty(&self).unwrap()
    }
}
