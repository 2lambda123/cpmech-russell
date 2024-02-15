use crate::constants::*;
use crate::StrError;
use crate::{Information, Method, NumSolver, ParamsERK, System, Workspace};
use russell_lab::{vec_copy, vec_update, Matrix, Vector};
use russell_sparse::CooMatrix;

pub(crate) struct ExplicitRungeKutta<'a, F, J, A>
where
    F: FnMut(&mut Vector, f64, &Vector, &mut A) -> Result<(), StrError>,
    J: FnMut(&mut CooMatrix, f64, &Vector, f64, &mut A) -> Result<(), StrError>,
{
    /// Holds the ERK method
    method: Method,

    /// Holds the parameters
    params: ParamsERK,

    /// ODE system
    system: System<'a, F, J, A>,

    /// Information such as implicit, embedded, etc.
    info: Information,

    /// Runge-Kutta A coefficients
    aa: Matrix,

    /// Runge-Kutta B coefficients
    bb: Vector,

    /// Runge-Kutta C coefficients
    cc: Vector,

    /// (embedded) error coefficients
    ///
    /// difference between B and Be: e = b - be
    ee: Option<Vector>,

    /// A coefficients for dense output
    aad: Option<Matrix>,

    /// C coefficients for dense output
    ccd: Option<Vector>,

    /// D coefficients for dense output
    dd: Option<Matrix>,

    /// Number of stages
    nstage: usize,

    /// Lund stabilization factor (n)
    ///
    /// exponent `n = 1/(q+1)` or `1/(q+1)-0.75⋅β` of `rerr ⁿ`
    lund_factor: f64,

    /// Auxiliary variable: 1 / m_min
    d_min: f64,

    /// Auxiliary variable: 1 / m_max
    d_max: f64,

    /// Stiffness ratio
    stiffness_ratio: f64,

    /// Array of vectors holding the updates
    ///
    /// v[stg][dim] = ya[dim] + h*sum(a[stg][j]*f[j][dim], j, nstage)
    v: Vec<Vector>,

    /// Array of vectors holding the function evaluations
    ///
    /// k[stg][dim] = f(u[stg], v[stg][dim])
    k: Vec<Vector>,

    /// Auxiliary workspace (will contain y0 to be used in accept_update)
    w: Vector,

    /// Dense output coefficients (nstage_dense * ndim)
    dense_out: Option<Vec<Vector>>,

    /// k values for dense output
    kd: Option<Vec<Vector>>,

    /// y values for dense output (len(kd)>0)
    yd: Option<Vector>,
}

impl<'a, F, J, A> ExplicitRungeKutta<'a, F, J, A>
where
    F: FnMut(&mut Vector, f64, &Vector, &mut A) -> Result<(), StrError>,
    J: FnMut(&mut CooMatrix, f64, &Vector, f64, &mut A) -> Result<(), StrError>,
{
    /// Allocates a new instance
    pub fn new(method: Method, params: ParamsERK, system: System<'a, F, J, A>) -> Result<Self, StrError> {
        // information
        let info = method.information();
        if info.implicit {
            return Err("the method must not be implicit");
        }
        if method == Method::FwEuler {
            return Err("the method must not be FwEuler");
        }

        // Runge-Kutta coefficients
        #[rustfmt::skip]
        let (aa, bb, cc) = match method {
            Method::Radau5     => panic!("<not available>"),
            Method::BwEuler    => panic!("<not available>"),
            Method::FwEuler    => panic!("<not available>"),
            Method::Rk2        => (Matrix::from(&RUNGE_KUTTA_2_A)     , Vector::from(&RUNGE_KUTTA_2_B)     , Vector::from(&RUNGE_KUTTA_2_C)    ),
            Method::Rk3        => (Matrix::from(&RUNGE_KUTTA_3_A)     , Vector::from(&RUNGE_KUTTA_3_B)     , Vector::from(&RUNGE_KUTTA_3_C)    ),
            Method::Heun3      => (Matrix::from(&HEUN_3_A)            , Vector::from(&HEUN_3_B)            , Vector::from(&HEUN_3_C)           ),
            Method::Rk4        => (Matrix::from(&RUNGE_KUTTA_4_A)     , Vector::from(&RUNGE_KUTTA_4_B)     , Vector::from(&RUNGE_KUTTA_4_C)    ),
            Method::Rk4alt     => (Matrix::from(&RUNGE_KUTTA_ALT_4_A) , Vector::from(&RUNGE_KUTTA_ALT_4_B) , Vector::from(&RUNGE_KUTTA_ALT_4_C)),
            Method::MdEuler    => (Matrix::from(&MODIFIED_EULER_A)    , Vector::from(&MODIFIED_EULER_B)    , Vector::from(&MODIFIED_EULER_C)   ),
            Method::Merson4    => (Matrix::from(&MERSON_4_A)          , Vector::from(&MERSON_4_B)          , Vector::from(&MERSON_4_C)         ),
            Method::Zonneveld4 => (Matrix::from(&ZONNEVELD_4_A)       , Vector::from(&ZONNEVELD_4_B)       , Vector::from(&ZONNEVELD_4_C)      ),
            Method::Fehlberg4  => (Matrix::from(&FEHLBERG_4_A)        , Vector::from(&FEHLBERG_4_B)        , Vector::from(&FEHLBERG_4_C)       ),
            Method::DoPri5     => (Matrix::from(&DORMAND_PRINCE_5_A)  , Vector::from(&DORMAND_PRINCE_5_B)  , Vector::from(&DORMAND_PRINCE_5_C) ),
            Method::Verner6    => (Matrix::from(&VERNER_6_A)          , Vector::from(&VERNER_6_B)          , Vector::from(&VERNER_6_C)         ),
            Method::Fehlberg7  => (Matrix::from(&FEHLBERG_7_A)        , Vector::from(&FEHLBERG_7_B)        , Vector::from(&FEHLBERG_7_C)       ),
            Method::DoPri8     => (Matrix::from(&DORMAND_PRINCE_8_A)  , Vector::from(&DORMAND_PRINCE_8_B)  , Vector::from(&DORMAND_PRINCE_8_C) ),
        };

        // coefficients for error estimate
        let ee = if info.embedded {
            match method {
                Method::Radau5 => None,
                Method::BwEuler => None,
                Method::FwEuler => None,
                Method::Rk2 => None,
                Method::Rk3 => None,
                Method::Heun3 => None,
                Method::Rk4 => None,
                Method::Rk4alt => None,
                Method::MdEuler => Some(Vector::from(&MODIFIED_EULER_E)),
                Method::Merson4 => Some(Vector::from(&MERSON_4_E)),
                Method::Zonneveld4 => Some(Vector::from(&ZONNEVELD_4_E)),
                Method::Fehlberg4 => Some(Vector::from(&FEHLBERG_4_E)),
                Method::DoPri5 => Some(Vector::from(&DORMAND_PRINCE_5_E)),
                Method::Verner6 => Some(Vector::from(&VERNER_6_E)),
                Method::Fehlberg7 => Some(Vector::from(&FEHLBERG_7_E)),
                Method::DoPri8 => Some(Vector::from(&DORMAND_PRINCE_8_E)),
            }
        } else {
            None
        };

        // coefficients for dense output
        let ndim = system.ndim;
        let (mut aad, mut ccd, mut dd) = (None, None, None);
        let (mut dense_out, mut kd, mut yd) = (None, None, None);
        if params.use_dense_output && method == Method::DoPri5 {
            dd = Some(Matrix::from(&DORMAND_PRINCE_5_D));
            dense_out = Some(vec![Vector::new(ndim); 5]);
        }
        if params.use_dense_output && method == Method::DoPri8 {
            aad = Some(Matrix::from(&DORMAND_PRINCE_8_AD));
            ccd = Some(Vector::from(&DORMAND_PRINCE_8_CD));
            dd = Some(Matrix::from(&DORMAND_PRINCE_8_D));
            dense_out = Some(vec![Vector::new(ndim); 8]);
            kd = Some(vec![Vector::new(ndim); 3]);
            yd = Some(Vector::new(ndim));
        }

        // number of stages
        let nstage = bb.dim();

        // Lund stabilization factor (n)
        let lund_factor = if params.lund_beta > 0.0 {
            1.0 / ((info.order_of_estimator + 1) as f64) - params.lund_beta * params.lund_beta_m
        } else {
            1.0 / ((info.order_of_estimator + 1) as f64)
        };

        // return structure
        Ok(ExplicitRungeKutta {
            method,
            params,
            system,
            info,
            aa,
            bb,
            cc,
            ee,
            aad,
            ccd,
            dd,
            nstage,
            lund_factor,
            d_min: 1.0 / params.m_min,
            d_max: 1.0 / params.m_max,
            stiffness_ratio: 0.0,
            v: vec![Vector::new(ndim); nstage],
            k: vec![Vector::new(ndim); nstage],
            w: Vector::new(ndim),
            dense_out,
            kd,
            yd,
        })
    }
}

impl<'a, F, J, A> NumSolver<A> for ExplicitRungeKutta<'a, F, J, A>
where
    F: FnMut(&mut Vector, f64, &Vector, &mut A) -> Result<(), StrError>,
    J: FnMut(&mut CooMatrix, f64, &Vector, f64, &mut A) -> Result<(), StrError>,
{
    /// Initializes the internal variables
    fn initialize(&mut self, _x: f64, _y: &Vector) {}

    /// Calculates the quantities required to update x and y
    fn step(&mut self, work: &mut Workspace, x: f64, y: &Vector, h: f64, args: &mut A) -> Result<(), StrError> {
        // auxiliary
        let k = &mut self.k;
        let v = &mut self.v;

        // compute k0 (otherwise, use k0 saved in accept_update)
        if (work.first_step || !self.info.first_step_same_as_last) && !work.follows_reject_step {
            let u0 = x + h * self.cc[0];
            work.bench.n_function_eval += 1;
            (self.system.function)(&mut k[0], u0, y, args)?; // k0 := f(ui,vi)
        }

        // compute ki
        for i in 1..self.nstage {
            let ui = x + h * self.cc[i];
            vec_copy(&mut v[i], &y).unwrap(); // vi := ya
            for j in 0..i {
                vec_update(&mut v[i], h * self.aa.get(i, j), &k[j]).unwrap(); // vi += h ⋅ aij ⋅ kj
            }
            work.bench.n_function_eval += 1;
            (self.system.function)(&mut k[i], ui, &v[i], args)?; // ki := f(ui,vi)
        }

        // update
        if !self.info.embedded {
            for m in 0..self.system.ndim {
                self.w[m] = y[m];
                for i in 0..self.nstage {
                    self.w[m] += self.bb[i] * k[i][m] * h;
                }
            }
            return Ok(());
        }

        // auxiliary
        let ee = self.ee.as_ref().unwrap();
        let mut s_num = 0.0;
        let mut s_den = 0.0;
        let dim = self.system.ndim as f64;

        // error estimation for Dormand-Prince 8 with 5 and 3 orders
        if self.method == Method::DoPri8 {
            let (bhh1, bhh2, bhh3) = (DORMAND_PRINCE_8_BHH1, DORMAND_PRINCE_8_BHH2, DORMAND_PRINCE_8_BHH3);
            let mut err_3 = 0.0;
            let mut err_5 = 0.0;
            for m in 0..self.system.ndim {
                self.w[m] = y[m];
                let mut err_a = 0.0;
                let mut err_b = 0.0;
                for i in 0..self.nstage {
                    self.w[m] += self.bb[i] * k[i][m] * h;
                    err_a += self.bb[i] * k[i][m];
                    err_b += ee[i] * k[i][m];
                }
                let sk = self.params.abs_tol + self.params.rel_tol * f64::max(f64::abs(y[m]), f64::abs(self.w[m]));
                err_a -= bhh1 * k[0][m] + bhh2 * k[8][m] + bhh3 * k[11][m];
                err_3 += (err_a / sk) * (err_a / sk);
                err_5 += (err_b / sk) * (err_b / sk);
                // stiffness estimation
                let a = self.nstage - 1;
                let b = self.nstage - 2;
                let dk = k[a][m] - k[b][m];
                let dv = v[a][m] - v[b][m];
                s_num += dk * dk;
                s_den += dv * dv;
            }
            let mut den = err_5 + 0.01 * err_3; // similar to Eq. (10.17) of [1, page 255]
            if den <= 0.0 {
                den = 1.0;
            }
            work.rel_error = f64::abs(h) * err_5 * f64::sqrt(1.0 / (dim * den));
            if s_den > 0.0 {
                self.stiffness_ratio = h * f64::sqrt(s_num / s_den);
            }
            return Ok(());
        }

        // update, error and stiffness estimation
        let mut sum = 0.0;
        for m in 0..self.system.ndim {
            self.w[m] = y[m];
            let mut l_err_m = 0.0;
            for i in 0..self.nstage {
                let kh = k[i][m] * h;
                self.w[m] += self.bb[i] * kh;
                l_err_m += ee[i] * kh;
            }
            let sk = self.params.abs_tol + self.params.rel_tol * f64::max(f64::abs(y[m]), f64::abs(self.w[m]));
            let ratio = l_err_m / sk;
            sum += ratio * ratio;
            // stiffness estimation
            let a = self.nstage - 1;
            let b = self.nstage - 2;
            let dk = k[a][m] - k[b][m];
            let dv = v[a][m] - v[b][m];
            s_num += dk * dk;
            s_den += dv * dv;
        }
        work.rel_error = f64::max(f64::sqrt(sum / dim), 1.0e-10);
        if s_den > 0.0 {
            self.stiffness_ratio = h * f64::sqrt(s_num / s_den);
        }
        return Ok(());
    }

    /// Updates x and y and computes the next stepsize
    fn accept(
        &mut self,
        work: &mut Workspace,
        x: &mut f64,
        y: &mut Vector,
        h: f64,
        args: &mut A,
    ) -> Result<(), StrError> {
        // store data for future dense output (Dormand-Prince 5)
        if self.params.use_dense_output && self.method == Method::DoPri5 {
            let dd = self.dd.as_ref().unwrap();
            let d = self.dense_out.as_mut().unwrap();
            let k = &self.k;
            for m in 0..self.system.ndim {
                let y_diff = self.w[m] - y[m];
                let b_spl = h * k[0][m] - y_diff;
                d[0][m] = y[m];
                d[1][m] = y_diff;
                d[2][m] = b_spl;
                d[3][m] = y_diff - h * k[6][m] - b_spl;
                d[4][m] = dd.get(0, 0) * k[0][m]
                    + dd.get(0, 2) * k[2][m]
                    + dd.get(0, 3) * k[3][m]
                    + dd.get(0, 4) * k[4][m]
                    + dd.get(0, 5) * k[5][m]
                    + dd.get(0, 6) * k[6][m];
                d[4][m] *= h;
            }
        }

        // store data for future dense output (Dormand-Prince 8)
        if self.params.use_dense_output && self.method == Method::DoPri8 {
            // auxiliary variables
            let aad = self.aad.as_ref().unwrap();
            let cd = self.ccd.as_ref().unwrap();
            let dd = self.dd.as_ref().unwrap();
            let d = self.dense_out.as_mut().unwrap();
            let kd = self.kd.as_mut().unwrap();
            let yd = self.yd.as_mut().unwrap();
            let k = &self.k;

            // first function evaluation
            for m in 0..self.system.ndim {
                yd[m] = y[m]
                    + h * (aad.get(0, 0) * k[0][m]
                        + aad.get(0, 6) * k[6][m]
                        + aad.get(0, 7) * k[7][m]
                        + aad.get(0, 8) * k[8][m]
                        + aad.get(0, 9) * k[9][m]
                        + aad.get(0, 10) * k[10][m]
                        + aad.get(0, 11) * k[11][m]
                        + aad.get(0, 12) * k[11][m]);
            }
            let u = *x + cd[0] * h;
            work.bench.n_function_eval += 1;
            (self.system.function)(&mut kd[0], u, yd, args)?;

            // second function evaluation
            for m in 0..self.system.ndim {
                yd[m] = y[m]
                    + h * (aad.get(1, 0) * k[0][m]
                        + aad.get(1, 5) * k[5][m]
                        + aad.get(1, 6) * k[6][m]
                        + aad.get(1, 7) * k[7][m]
                        + aad.get(1, 10) * k[10][m]
                        + aad.get(1, 11) * k[11][m]
                        + aad.get(1, 12) * k[11][m]
                        + aad.get(1, 13) * kd[0][m]);
            }
            let u = *x + cd[1] * h;
            work.bench.n_function_eval += 1;
            (self.system.function)(&mut kd[1], u, yd, args)?;

            // next third function evaluation
            for m in 0..self.system.ndim {
                yd[m] = y[m]
                    + h * (aad.get(2, 0) * k[0][m]
                        + aad.get(2, 5) * k[5][m]
                        + aad.get(2, 6) * k[6][m]
                        + aad.get(2, 7) * k[7][m]
                        + aad.get(2, 8) * k[8][m]
                        + aad.get(2, 12) * k[11][m]
                        + aad.get(2, 13) * kd[0][m]
                        + aad.get(2, 14) * kd[1][m]);
            }
            let u = *x + cd[2] * h;
            work.bench.n_function_eval += 1;
            (self.system.function)(&mut kd[2], u, yd, args)?;

            // final results
            for m in 0..self.system.ndim {
                let y_diff = self.w[m] - y[m];
                let b_spl = h * k[0][m] - y_diff;
                d[0][m] = y[m];
                d[1][m] = y_diff;
                d[2][m] = b_spl;
                d[3][m] = y_diff - h * k[11][m] - b_spl;
                d[4][m] = h
                    * (dd.get(0, 0) * k[0][m]
                        + dd.get(0, 5) * k[5][m]
                        + dd.get(0, 6) * k[6][m]
                        + dd.get(0, 7) * k[7][m]
                        + dd.get(0, 8) * k[8][m]
                        + dd.get(0, 9) * k[9][m]
                        + dd.get(0, 10) * k[10][m]
                        + dd.get(0, 11) * k[11][m]
                        + dd.get(0, 12) * k[11][m]
                        + dd.get(0, 13) * kd[0][m]
                        + dd.get(0, 14) * kd[1][m]
                        + dd.get(0, 15) * kd[2][m]);
                d[5][m] = h
                    * (dd.get(1, 0) * k[0][m]
                        + dd.get(1, 5) * k[5][m]
                        + dd.get(1, 6) * k[6][m]
                        + dd.get(1, 7) * k[7][m]
                        + dd.get(1, 8) * k[8][m]
                        + dd.get(1, 9) * k[9][m]
                        + dd.get(1, 10) * k[10][m]
                        + dd.get(1, 11) * k[11][m]
                        + dd.get(1, 12) * k[11][m]
                        + dd.get(1, 13) * kd[0][m]
                        + dd.get(1, 14) * kd[1][m]
                        + dd.get(1, 15) * kd[2][m]);
                d[6][m] = h
                    * (dd.get(2, 0) * k[0][m]
                        + dd.get(2, 5) * k[5][m]
                        + dd.get(2, 6) * k[6][m]
                        + dd.get(2, 7) * k[7][m]
                        + dd.get(2, 8) * k[8][m]
                        + dd.get(2, 9) * k[9][m]
                        + dd.get(2, 10) * k[10][m]
                        + dd.get(2, 11) * k[11][m]
                        + dd.get(2, 12) * k[11][m]
                        + dd.get(2, 13) * kd[0][m]
                        + dd.get(2, 14) * kd[1][m]
                        + dd.get(2, 15) * kd[2][m]);
                d[7][m] = h
                    * (dd.get(3, 0) * k[0][m]
                        + dd.get(3, 5) * k[5][m]
                        + dd.get(3, 6) * k[6][m]
                        + dd.get(3, 7) * k[7][m]
                        + dd.get(3, 8) * k[8][m]
                        + dd.get(3, 9) * k[9][m]
                        + dd.get(3, 10) * k[10][m]
                        + dd.get(3, 11) * k[11][m]
                        + dd.get(3, 12) * k[11][m]
                        + dd.get(3, 13) * kd[0][m]
                        + dd.get(3, 14) * kd[1][m]
                        + dd.get(3, 15) * kd[2][m]);
            }
        }

        // update x and y
        *x += h;
        vec_copy(y, &self.w).unwrap();

        // update k0
        if self.info.first_step_same_as_last {
            for m in 0..self.system.ndim {
                self.k[0][m] = self.k[self.nstage - 1][m]; // k0 := ks for next step
            }
        }

        // handle not embedded methods
        if !self.info.embedded {
            return Ok(());
        }

        // estimate the new stepsize
        let mut d = f64::powf(work.rel_error, self.lund_factor);
        if self.params.lund_beta > 0.0 && work.rel_error_prev > 0.0 {
            // lund-stabilization
            d = d / f64::powf(work.rel_error_prev, self.params.lund_beta);
        }
        d = f64::max(self.d_max, f64::min(self.d_min, d / self.params.m_factor)); // we require fac1 <= h_new/h <= fac2
        work.h_new = h / d;
        Ok(())
    }

    /// Rejects the update
    fn reject(&mut self, work: &mut Workspace, h: f64) {
        // estimate new stepsize
        let d = f64::powf(work.rel_error, self.lund_factor) / self.params.m_factor;
        work.h_new = h / f64::min(self.d_min, d);
    }

    /// Computes the dense output
    fn dense_output(&self, y_out: &mut Vector, h: f64, x: f64, x_out: f64) {
        if self.params.use_dense_output && self.method == Method::DoPri5 {
            let d = self.dense_out.as_ref().unwrap();
            let x_prev = x - h;
            let theta = (x_out - x_prev) / h;
            let u_theta = 1.0 - theta;
            for m in 0..self.system.ndim {
                y_out[m] = d[0][m] + theta * (d[1][m] + u_theta * (d[2][m] + theta * (d[3][m] + u_theta * d[4][m])));
            }
        }
        if self.params.use_dense_output && self.method == Method::DoPri8 {
            let d = self.dense_out.as_ref().unwrap();
            let x_prev = x - h;
            let theta = (x_out - x_prev) / h;
            let u_theta = 1.0 - theta;
            for m in 0..self.system.ndim {
                let par = d[4][m] + theta * (d[5][m] + u_theta * (d[6][m] + theta * d[7][m]));
                y_out[m] = d[0][m] + theta * (d[1][m] + u_theta * (d[2][m] + theta * (d[3][m] + u_theta * par)));
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::ExplicitRungeKutta;
    use crate::{no_jacobian, HasJacobian, Method, ParamsERK, System};
    use russell_lab::approx_eq;

    #[test]
    fn constants_are_consistent() {
        let methods = Method::explicit_methods();
        let staged = methods.iter().filter(|&&m| m != Method::FwEuler);
        struct Args {}
        for method in staged {
            println!("\n... {:?} ...", method);
            let params = ParamsERK::new(*method);
            let system = System::new(
                1,
                |_, _, _, _args: &mut Args| Ok(()),
                no_jacobian,
                HasJacobian::No,
                None,
                None,
            );
            let erk = ExplicitRungeKutta::new(*method, params, system).unwrap();
            let nstage = erk.nstage;
            assert_eq!(erk.aa.dims(), (nstage, nstage));
            assert_eq!(erk.bb.dim(), nstage);
            assert_eq!(erk.cc.dim(), nstage);
            let info = method.information();
            if info.embedded {
                let ee = erk.ee.as_ref().unwrap();
                assert_eq!(ee.dim(), nstage);
            }

            println!("Σi bi = 1                                 (Eq. 1.11a, page 135)");
            let mut sum = 0.0;
            for i in 0..nstage {
                sum += erk.bb[i];
            }
            approx_eq(sum, 1.0, 1e-15);

            println!("Σi bi ci = 1/2                            (Eq. 1.11b, page 135)");
            sum = 0.0;
            for i in 0..nstage {
                sum += erk.bb[i] * erk.cc[i];
            }
            approx_eq(sum, 1.0 / 2.0, 1e-15);

            if erk.info.order < 4 {
                continue;
            }

            println!("Σi bi ci² = 1/3                           (Eq. 1.11c, page 135)");
            sum = 0.0;
            for i in 0..nstage {
                sum += erk.bb[i] * erk.cc[i] * erk.cc[i];
            }
            approx_eq(sum, 1.0 / 3.0, 1e-15);

            println!("Σi,j bi aij cj = 1/6                      (Eq. 1.11d, page 135)");
            sum = 0.0;
            for i in 0..nstage {
                for j in 0..nstage {
                    sum += erk.bb[i] * erk.aa.get(i, j) * erk.cc[j];
                }
            }
            approx_eq(sum, 1.0 / 6.0, 1e-15);

            println!("Σi bi ci³ = 1/4                           (Eq. 1.11e, page 135)");
            sum = 0.0;
            for i in 0..nstage {
                sum += erk.bb[i] * erk.cc[i] * erk.cc[i] * erk.cc[i];
            }
            approx_eq(sum, 1.0 / 4.0, 1e-15);

            println!("Σi,j bi ci aij cj = 1/8                   (Eq. 1.11f, page 135)");
            sum = 0.0;
            for i in 0..nstage {
                for j in 0..nstage {
                    sum += erk.bb[i] * erk.cc[i] * erk.aa.get(i, j) * erk.cc[j];
                }
            }
            approx_eq(sum, 1.0 / 8.0, 1e-15);

            println!("Σi,j bi aij cj² = 1/12                    (Eq. 1.11g, page 136)");
            sum = 0.0;
            for i in 0..nstage {
                for j in 0..nstage {
                    sum += erk.bb[i] * erk.aa.get(i, j) * erk.cc[j] * erk.cc[j];
                }
            }
            approx_eq(sum, 1.0 / 12.0, 1e-15);

            println!("Σi,j,k bi aij ajk ck = 1/24               (Eq. 1.11h, page 136)");
            sum = 0.0;
            for i in 0..nstage {
                for j in 0..nstage {
                    for k in 0..nstage {
                        sum += erk.bb[i] * erk.aa.get(i, j) * erk.aa.get(j, k) * erk.cc[k];
                    }
                }
            }
            approx_eq(sum, 1.0 / 24.0, 1e-15);

            println!("Σ(i=1,s) bi ciⁿ⁻¹ = 1 / n   for n = 1..8  (Eq. 5.20a, page 181)");
            for n in 1..erk.info.order {
                let mut sum = 0.0;
                for i in 1..(nstage + 1) {
                    sum += erk.bb[i - 1] * f64::powf(erk.cc[i - 1], (n - 1) as f64);
                }
                approx_eq(sum, 1.0 / (n as f64), 1e-15);
            }

            println!("Σ(j=1,i-1) aij = ci         for i = 1..s  (Eq. 5.20b, page 181)");
            for i in 1..(nstage + 1) {
                let mut sum = 0.0;
                for j in 1..i {
                    sum += erk.aa.get(i - 1, j - 1);
                }
                approx_eq(sum, erk.cc[i - 1], 1e-14);
            }

            if erk.info.order < 5 {
                continue;
            }

            println!("Σ(j=1,i-1) aij cj = ci² / 2 for i = 3..s  (Eq. 5.20c, page 181)");
            for i in 3..(nstage + 1) {
                let mut sum = 0.0;
                for j in 1..i {
                    sum += erk.aa.get(i - 1, j - 1) * erk.cc[j - 1];
                }
                approx_eq(sum, erk.cc[i - 1] * erk.cc[i - 1] / 2.0, 1e-14);
            }
        }
    }
}