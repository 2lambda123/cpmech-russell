#![allow(unused, non_snake_case)]

use crate::Method;
use crate::StrError;
use russell_sparse::{Genie, LinSolParams};

/// Defines the configuration parameters for the ODE solver
#[derive(Clone, Debug)]
pub struct OdeParams {
    /// Method
    pub(crate) method: Method,

    /// linear solver kind
    pub(crate) genie: Genie,

    /// configurations for sparse linear solver
    pub lin_sol_params: LinSolParams,

    /// minimum H allowed
    pub Hmin: f64,

    /// initial H
    pub initial_stepsize: f64,

    /// max num iterations (allowed)
    pub NmaxIt: usize,

    /// max num substeps
    pub NmaxSS: usize,

    /// min step multiplier
    pub Mmin: f64,

    /// max step multiplier
    pub Mmax: f64,

    /// step multiplier factor
    pub Mfac: f64,

    /// coefficient to multiply stepsize if first step is rejected [0 ⇒ use dx_new]
    pub MfirstRej: f64,

    /// use Gustafsson's predictive controller
    pub PredCtrl: bool,

    /// smallest number satisfying 1.0 + ϵ > 1.0
    pub Eps: f64,

    /// max theta to decide whether the Jacobian should be recomputed or not
    pub ThetaMax: f64,

    /// c1 of HW-VII p124 => min ratio to retain previous h
    pub C1h: f64,

    /// c2 of HW-VII p124 => max ratio to retain previous h
    pub C2h: f64,

    /// strategy to select local error computation method
    pub LerrStrat: usize,

    /// allow use of go channels (threaded); e.g. to solve R and C systems concurrently
    pub GoChan: bool,

    /// use constant tangent (Jacobian) in BwEuler
    pub CteTg: bool,

    /// use RMS norm instead of Euclidean in BwEuler
    pub UseRmsNorm: bool,

    /// show messages, e.g. during iterations
    pub Verbose: bool,

    /// always start iterations with zero trial values (instead of collocation interpolation)
    pub ZeroTrial: bool,

    /// Lund stabilization coefficient β
    pub StabBeta: f64,

    /// number of steps to check stiff situation. 0 ⇒ no check. [default = 1]
    pub StiffNstp: usize,

    /// maximum value of ρs [default = 0.5]
    pub StiffRsMax: f64,

    /// number of "yes" stiff steps allowed [default = 15]
    pub StiffNyes: usize,

    /// number of "not" stiff steps to disregard stiffness [default = 6]
    pub StiffNnot: usize,

    /// step size for dense output
    pub denseDx: f64,

    /// Activates dense output
    pub denseOut: bool,

    /// number of dense steps
    pub denseNstp: usize,

    /// factor to multiply stabilization coefficient β
    pub stabBetaM: f64,

    /// absolute tolerance
    pub abs_tol: f64,

    /// relative tolerance
    pub rel_tol: f64,

    /// Newton's iterations tolerance
    pub fnewt: f64,

    /// min value of rerrPrev
    pub rerrPrevMin: f64,
}

impl OdeParams {
    /// Allocates a new instance with default values
    pub fn new(method: Method, lin_sol: Option<Genie>, lin_sol_params: Option<LinSolParams>) -> Self {
        let genie = match lin_sol {
            Some(g) => g,
            None => Genie::Umfpack,
        };
        let ls_params = match lin_sol_params {
            Some(p) => p,
            None => LinSolParams::new(),
        };
        let mut params = OdeParams {
            method,
            genie,
            lin_sol_params: ls_params,
            Hmin: 1.0e-10,
            initial_stepsize: 1.0e-4,
            NmaxIt: 7,
            NmaxSS: 1000,
            Mmin: 0.125,
            Mmax: 5.0,
            Mfac: 0.9,
            MfirstRej: 0.1,
            PredCtrl: true,
            Eps: 1e-16,
            ThetaMax: 1.0e-3,
            C1h: 1.0,
            C2h: 1.2,
            LerrStrat: 3,
            GoChan: true,
            CteTg: false,
            UseRmsNorm: true,
            Verbose: false,
            ZeroTrial: false,
            StabBeta: 0.04,
            StiffNstp: 0,
            StiffRsMax: 0.5,
            StiffNyes: 15,
            StiffNnot: 6,
            denseDx: 0.0,
            denseOut: false,
            denseNstp: 0,
            stabBetaM: 0.0,
            abs_tol: 0.0,
            rel_tol: 0.0,
            fnewt: 0.0,
            rerrPrevMin: 1.0e-4,
        };
        params.set_tolerances(1e-4, 1e-4).unwrap();
        if method == Method::Radau5 {
            params.rerrPrevMin = 1.0e-2;
        }
        if method == Method::DoPri5 {
            params.StabBeta = 0.04;
            params.stabBetaM = 0.75;
        }
        if method == Method::DoPri8 {
            params.stabBetaM = 0.2;
        }
        params
    }

    /// Sets the tolerances
    ///
    /// # Input
    ///
    /// * `abs_tol` -- absolute tolerance
    /// * `rel_tol` -- relative tolerance
    pub fn set_tolerances(&mut self, abs_tol: f64, rel_tol: f64) -> Result<(), StrError> {
        // check
        if abs_tol <= 0.0 {
            return Err("absolute tolerance must be greater than zero");
        }
        if abs_tol <= 10.0 * self.Eps {
            return Err("absolute tolerance must be grater than 10 * EPS");
        }
        if rel_tol <= 0.0 {
            return Err("relative tolerance must be greater than zero");
        }

        // set
        self.abs_tol = abs_tol;
        self.rel_tol = rel_tol;

        // change the tolerances (radau5 only)
        if self.method == Method::Radau5 {
            const BETA: f64 = 2.0 / 3.0;
            let quot = self.abs_tol / self.rel_tol;
            self.rel_tol = 0.1 * f64::powf(self.rel_tol, BETA);
            self.abs_tol = self.rel_tol * quot;
        }

        // tolerance for iterations
        self.fnewt = f64::max(10.0 * self.Eps / self.rel_tol, f64::min(0.03, f64::sqrt(self.rel_tol)));
        Ok(())
    }

    /// TODO
    pub(crate) fn validate(&self) -> Result<(), StrError> {
        Ok(())
    }
}