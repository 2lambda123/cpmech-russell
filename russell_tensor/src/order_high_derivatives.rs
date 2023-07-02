use crate::{
    t2_dyad_t2, t2_odyad_t2, t2_qsd_t2, t2_ssd, Mandel, StrError, Tensor2, Tensor4, ONE_BY_3, SQRT_3, TOL_J2, TWO_BY_3,
};
use russell_lab::{mat_add, mat_mat_mul, mat_update};

/// Calculates the derivative of the inverse tensor w.r.t. the defining Tensor2
///
/// ```text
/// dA⁻¹         _
/// ──── = - A⁻¹ ⊗ A⁻ᵀ
///  dA
/// ```
///
/// ```text
/// With orthonormal Cartesian components:
///
/// ∂A⁻¹ᵢⱼ
/// ────── = - A⁻¹ᵢₖ A⁻ᵀⱼₗ
///  ∂Aₖₗ
/// ```
///
/// ## Output
///
/// * `dai_da` -- the derivative of the inverse tensor
///
/// ## Input
///
/// * `ai` -- the pre-computed inverse tensor
/// * `a` -- the defining tensor
pub fn deriv_inverse_tensor(dai_da: &mut Tensor4, ai: &Tensor2) -> Result<(), StrError> {
    let mut ai_t = ai.clone();
    ai.transpose(&mut ai_t).unwrap();
    t2_odyad_t2(dai_da, -1.0, &ai, &ai_t)
}

/// Calculates the derivative of the inverse tensor w.r.t. the defining Tensor2 (symmetric)
///
/// ```text
/// dA⁻¹     1      _                 
/// ──── = - ─ (A⁻¹ ⊗ A⁻¹ + A⁻¹ ⊗ A⁻¹)
///  dA      2                  ‾     
///
///      = - 0.5 ssd(A⁻¹)
/// ```
///
/// ```text
/// With orthonormal Cartesian components:
///
/// ∂A⁻¹ᵢⱼ     1
/// ────── = - ─ (A⁻¹ᵢₖ A⁻¹ⱼₗ + A⁻¹ᵢₗ A⁻¹ⱼₖ)
///  ∂Aₖₗ      2
/// ```
///
/// ## Output
///
/// * `dai_da` -- the derivative of the inverse tensor (must be Symmetric)
///
/// ## Input
///
/// * `ai` -- the pre-computed inverse tensor (must be Symmetric or Symmetric2D)
pub fn deriv_inverse_tensor_sym(dai_da: &mut Tensor4, ai: &Tensor2) -> Result<(), StrError> {
    if ai.case() == Mandel::General {
        return Err("tensor 'ai' must be Symmetric or Symmetric2D");
    }
    if dai_da.case() != Mandel::Symmetric {
        return Err("tensor 'dai_da' must be Symmetric");
    }
    t2_ssd(dai_da, -0.5, ai).unwrap();
    Ok(())
}

/// Calculates the derivative of the squared tensor w.r.t. the defining Tensor2
///
/// ```text
/// dA²     _       _
/// ─── = A ⊗ I + I ⊗ Aᵀ
/// dA
/// ```
///
/// ```text
/// With orthonormal Cartesian components:
///
/// ∂A²ᵢⱼ
/// ───── = Aᵢₖ δⱼₗ + δᵢₖ Aₗⱼ
///  ∂Aₖₗ
/// ```
///
/// ## Output
///
/// * `da2_da` -- the derivative of the squared tensor (must be General)
///
/// ## Input
///
/// * `a` -- the given tensor
///
/// ## Note
///
/// Two temporary Tensor2 and a Tensor4 are allocated in this function.
pub fn deriv_squared_tensor(da2_da: &mut Tensor4, a: &Tensor2) -> Result<(), StrError> {
    if da2_da.case() != Mandel::General {
        return Err("tensor 'da2_da' must be General");
    }

    // compute A odyad I
    let ii = Tensor2::identity(a.case());
    t2_odyad_t2(da2_da, 1.0, &a, &ii).unwrap();

    // compute I odyad transpose(A)
    let mut at = a.clone();
    a.transpose(&mut at).unwrap();
    let mut ii_odyad_at = Tensor4::new(Mandel::General);
    t2_odyad_t2(&mut ii_odyad_at, 1.0, &ii, &at).unwrap();

    // compute A odyad I + I odyad transpose(A)
    for m in 0..9 {
        for n in 0..9 {
            da2_da.mat.set(m, n, da2_da.mat.get(m, n) + ii_odyad_at.mat.get(m, n));
        }
    }
    Ok(())
}

/// Calculates the derivative of the squared tensor w.r.t. the defining Tensor2 (symmetric)
///
/// ```text
/// dA²   1    _               _
/// ─── = ─ (A ⊗ I + A ⊗ I + I ⊗ A + I ⊗ A)
/// dA    2            ‾               ‾
///
///     = 0.5 qsd(A, I)
/// ```
///
/// ```text
/// With orthonormal Cartesian components:
///
/// ∂A²ᵢⱼ   1
/// ───── = ─ (Aᵢₖ δⱼₗ + Aᵢₗ δⱼₖ + δᵢₖ Aⱼₗ + δᵢₗ Aⱼₖ)
///  ∂Aₖₗ   2
/// ```
///
/// ## Output
///
/// * `da2_da` -- the derivative of the squared tensor (must be Symmetric)
///
/// ## Input
///
/// * `a` -- the given tensor (must be Symmetric or Symmetric2D)
pub fn deriv_squared_tensor_sym(da2_da: &mut Tensor4, a: &Tensor2) -> Result<(), StrError> {
    if a.case() == Mandel::General {
        return Err("tensor 'a' must be Symmetric or Symmetric2D");
    }
    if da2_da.case() != Mandel::Symmetric {
        return Err("tensor 'da2_da' must be Symmetric");
    }
    let ii = Tensor2::identity(a.case());
    t2_qsd_t2(da2_da, 0.5, a, &ii).unwrap();
    Ok(())
}

/// Computes the second derivative of the J2 invariant w.r.t. the defining tensor
///
/// ```text
///  d²J2
/// ─────── = Psymdev   (σ must be symmetric)
/// dσ ⊗ dσ
/// ```
///
/// ## Output
///
/// * `d2` -- the second derivative of J2 (must be Symmetric)
///
/// ## Input
///
/// * `sigma` -- the given tensor (must be Symmetric or Symmetric2D)
pub fn deriv2_invariant_jj2(d2: &mut Tensor4, sigma: &Tensor2) -> Result<(), StrError> {
    if sigma.case() == Mandel::General {
        return Err("tensor 'sigma' must be Symmetric or Symmetric2D");
    }
    if d2.case() != Mandel::Symmetric {
        return Err("tensor 'd2' must be Symmetric");
    }
    d2.mat.fill(0.0);
    d2.mat.set(0, 0, TWO_BY_3);
    d2.mat.set(0, 1, -ONE_BY_3);
    d2.mat.set(0, 2, -ONE_BY_3);
    d2.mat.set(1, 0, -ONE_BY_3);
    d2.mat.set(1, 1, TWO_BY_3);
    d2.mat.set(1, 2, -ONE_BY_3);
    d2.mat.set(2, 0, -ONE_BY_3);
    d2.mat.set(2, 1, -ONE_BY_3);
    d2.mat.set(2, 2, TWO_BY_3);
    d2.mat.set(3, 3, 1.0);
    d2.mat.set(4, 4, 1.0);
    d2.mat.set(5, 5, 1.0);
    Ok(())
}

/// Holds auxiliary data to compute the second derivative of the J3 invariant
pub struct Deriv2InvariantJ3 {
    /// deviator tensor (Symmetric or Symmetric2D)
    pub s: Tensor2,

    /// identity tensor (Symmetric or Symmetric2D)
    pub ii: Tensor2,

    /// isotropic making projector Psymdev (Symmetric)
    pub psd: Tensor4,

    /// auxiliary fourth-order tensor (Symmetric)
    pub aa: Tensor4,

    /// auxiliary fourth-order tensor (Symmetric)
    pub bb: Tensor4,
}

impl Deriv2InvariantJ3 {
    /// Returns a new instance
    pub fn new(case: Mandel) -> Result<Self, StrError> {
        if case == Mandel::General {
            return Err("case must be Symmetric or Symmetric2D");
        }
        Ok(Deriv2InvariantJ3 {
            s: Tensor2::new(case),
            ii: Tensor2::identity(case),
            psd: Tensor4::constant_pp_symdev(true),
            aa: Tensor4::new(Mandel::Symmetric),
            bb: Tensor4::new(Mandel::Symmetric),
        })
    }

    /// Computes the second derivative of the J3 invariant w.r.t. the defining tensor
    ///
    /// ```text
    /// s := deviator(σ)
    ///
    ///  d²J3     1                    2
    /// ─────── = ─ qsd(s,I):Psymdev - ─ I ⊗ s
    /// dσ ⊗ dσ   2                    3
    ///
    /// (σ must be symmetric)
    /// ```
    ///
    /// ## Output
    ///
    /// * `d2` -- the second derivative of J3 (must be Symmetric)
    ///
    /// ## Input
    ///
    /// * `sigma` -- the given tensor (must be Symmetric or Symmetric2D)
    pub fn compute(&mut self, d2: &mut Tensor4, sigma: &Tensor2) -> Result<(), StrError> {
        if sigma.case() != self.s.case() {
            return Err("tensor 'sigma' is incompatible");
        }
        if d2.case() != Mandel::Symmetric {
            return Err("tensor 'd2' must be Symmetric");
        }
        sigma.deviator(&mut self.s).unwrap();
        t2_qsd_t2(&mut self.aa, 0.5, &mut self.s, &self.ii).unwrap(); // aa := 0.5 qsd(s,I)
        t2_dyad_t2(&mut self.bb, -TWO_BY_3, &self.ii, &self.s).unwrap(); // bb := -⅔ I ⊗ s
        mat_mat_mul(&mut d2.mat, 1.0, &self.aa.mat, &self.psd.mat).unwrap(); // d2 := 0.5 qsd(s,I) : Psd
        mat_update(&mut d2.mat, 1.0, &self.bb.mat).unwrap(); // d2 += -⅔ I ⊗ s
        Ok(())
    }
}

/// Holds auxiliary data to compute the second derivative of the deviatoric invariant
pub struct Deriv2InvariantSigmaD {
    /// first derivative of J2: dJ2/dσ (Symmetric or Symmetric2D)
    pub d1_jj2: Tensor2,

    /// second derivative of J2: d²J2/(dσ⊗dσ) (Symmetric)
    pub d2_jj2: Tensor4,
}

impl Deriv2InvariantSigmaD {
    /// Returns a new instance
    pub fn new(case: Mandel) -> Result<Self, StrError> {
        if case == Mandel::General {
            return Err("case must be Symmetric or Symmetric2D");
        }
        Ok(Deriv2InvariantSigmaD {
            d1_jj2: Tensor2::new(case),
            d2_jj2: Tensor4::new(Mandel::Symmetric),
        })
    }

    /// Computes the second derivative of the deviatoric invariant w.r.t. the defining tensor
    ///
    /// ```text
    ///  d²σd     d²J2      dJ2   dJ2
    /// ───── = a ───── - b ─── ⊗ ───
    /// dσ⊗dσ     dσ⊗dσ      dσ    dσ
    ///
    /// (σ must be symmetric)
    /// ```
    ///
    /// ```text
    ///          √3                  √3
    /// a = ─────────────   b = ─────────────
    ///     2 pow(J2,0.5)       4 pow(J2,1.5)
    /// ```
    ///
    /// ## Output
    ///
    /// * `d2` -- the second derivative of l (must be Symmetric)
    ///
    /// ## Input
    ///
    /// * `sigma` -- the given tensor (must be Symmetric or Symmetric2D)
    ///
    /// # Returns
    ///
    /// If `J2 > TOL_J2`, returns `J2` and the derivative in `d2`. Otherwise, returns None.
    pub fn compute(&mut self, d2: &mut Tensor4, sigma: &Tensor2) -> Result<Option<f64>, StrError> {
        if sigma.case() != self.d1_jj2.case() {
            return Err("tensor 'sigma' is incompatible");
        }
        if d2.case() != Mandel::Symmetric {
            return Err("tensor 'd2' must be Symmetric");
        }
        let jj2 = sigma.invariant_jj2();
        if jj2 > TOL_J2 {
            let a = 0.5 * SQRT_3 / f64::powf(jj2, 0.5);
            let b = 0.25 * SQRT_3 / f64::powf(jj2, 1.5);
            // sigma.deriv1_invariant_jj2(&mut self.d1_jj2).unwrap();
            deriv2_invariant_jj2(&mut self.d2_jj2, sigma).unwrap();
            t2_dyad_t2(d2, -b, &self.d1_jj2, &self.d1_jj2).unwrap();
            mat_update(&mut d2.mat, a, &self.d2_jj2.mat).unwrap();
            return Ok(Some(jj2));
        }
        Ok(None)
    }
}

/// Holds auxiliary data to compute the second derivative of the Lode invariant
pub struct Deriv2InvariantLode {
    /// auxiliary data to compute the second derivative of J3
    pub aux_jj3: Deriv2InvariantJ3,

    /// auxiliary second-order tensor (Symmetric or Symmetric2D)
    pub tt: Tensor2,

    /// first derivative of J2: dJ2/dσ (Symmetric or Symmetric2D)
    pub d1_jj2: Tensor2,

    /// first derivative of J3: dJ3/dσ (Symmetric or Symmetric2D)
    pub d1_jj3: Tensor2,

    /// second derivative of J2: d²J2/(dσ⊗dσ) (Symmetric)
    pub d2_jj2: Tensor4,

    /// second derivative of J3: d²J3/(dσ⊗dσ) (Symmetric)
    pub d2_jj3: Tensor4,

    /// dyadic product: dJ2/dσ ⊗ dJ2/dσ (Symmetric)
    pub d1_jj2_dy_d1_jj2: Tensor4,

    /// dyadic product: dJ2/dσ ⊗ dJ3/dσ (Symmetric)
    pub d1_jj2_dy_d1_jj3: Tensor4,

    /// dyadic product: dJ3/dσ ⊗ dJ2/dσ (Symmetric)
    pub d1_jj3_dy_d1_jj2: Tensor4,
}

impl Deriv2InvariantLode {
    /// Returns a new instance
    pub fn new(case: Mandel) -> Result<Self, StrError> {
        if case == Mandel::General {
            return Err("case must be Symmetric or Symmetric2D");
        }
        Ok(Deriv2InvariantLode {
            aux_jj3: Deriv2InvariantJ3::new(case).unwrap(),
            tt: Tensor2::new(case),
            d1_jj2: Tensor2::new(case),
            d1_jj3: Tensor2::new(case),
            d2_jj2: Tensor4::new(Mandel::Symmetric),
            d2_jj3: Tensor4::new(Mandel::Symmetric),
            d1_jj2_dy_d1_jj2: Tensor4::new(Mandel::Symmetric),
            d1_jj2_dy_d1_jj3: Tensor4::new(Mandel::Symmetric),
            d1_jj3_dy_d1_jj2: Tensor4::new(Mandel::Symmetric),
        })
    }

    /// Computes the second derivative of the Lode invariant w.r.t. the defining tensor
    ///
    /// ```text
    ///  d²l      d²J3         d²J2        dJ3   dJ2   dJ2   dJ3          dJ2   dJ2
    /// ───── = a ───── - b J3 ───── - b ( ─── ⊗ ─── + ─── ⊗ ─── ) + c J3 ─── ⊗ ───
    /// dσ⊗dσ     dσ⊗dσ        dσ⊗dσ        dσ    dσ    dσ    dσ           dσ    dσ
    ///
    /// (σ must be symmetric)
    /// ```
    ///
    /// ```text
    ///         3 √3               9 √3                45 √3
    /// a = ─────────────   b = ─────────────   c = ─────────────
    ///     2 pow(J2,1.5)       4 pow(J2,2.5)       8 pow(J2,3.5)
    /// ```
    ///
    /// ## Output
    ///
    /// * `d2` -- the second derivative of l (must be Symmetric)
    ///
    /// ## Input
    ///
    /// * `sigma` -- the given tensor (must be Symmetric or Symmetric2D)
    ///
    /// # Returns
    ///
    /// If `J2 > TOL_J2`, returns `J2` and the derivative in `d2`. Otherwise, returns None.
    pub fn compute(&mut self, d2: &mut Tensor4, sigma: &Tensor2) -> Result<Option<f64>, StrError> {
        if sigma.case() != self.tt.case() {
            return Err("tensor 'sigma' is incompatible");
        }
        if d2.case() != Mandel::Symmetric {
            return Err("tensor 'd2' must be Symmetric");
        }
        let jj2 = sigma.invariant_jj2();
        if jj2 > TOL_J2 {
            let jj3 = sigma.invariant_jj3();
            let a = 1.5 * SQRT_3 / f64::powf(jj2, 1.5);
            let b = 2.25 * SQRT_3 / f64::powf(jj2, 2.5);
            let c = 5.625 * SQRT_3 / f64::powf(jj2, 3.5);
            // sigma.deriv1_invariant_jj2(&mut self.d1_jj2).unwrap();
            // sigma.deriv1_invariant_jj3(&mut self.d1_jj3, &mut self.tt).unwrap();
            deriv2_invariant_jj2(&mut self.d2_jj2, sigma).unwrap();
            self.aux_jj3.compute(&mut self.d2_jj3, sigma).unwrap();
            t2_dyad_t2(&mut self.d1_jj2_dy_d1_jj2, 1.0, &self.d1_jj2, &self.d1_jj2).unwrap();
            t2_dyad_t2(&mut self.d1_jj2_dy_d1_jj3, 1.0, &self.d1_jj2, &self.d1_jj3).unwrap();
            t2_dyad_t2(&mut self.d1_jj3_dy_d1_jj2, 1.0, &self.d1_jj3, &self.d1_jj2).unwrap();
            mat_add(&mut d2.mat, a, &self.d2_jj3.mat, -b * jj3, &self.d2_jj2.mat).unwrap();
            mat_update(&mut d2.mat, -b, &self.d1_jj3_dy_d1_jj2.mat).unwrap();
            mat_update(&mut d2.mat, -b, &self.d1_jj2_dy_d1_jj3.mat).unwrap();
            mat_update(&mut d2.mat, c * jj3, &self.d1_jj2_dy_d1_jj2.mat).unwrap();
            return Ok(Some(jj2));
        }
        Ok(None)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::{Tensor2, Tensor4};
    use crate::{
        deriv2_invariant_jj2, deriv_inverse_tensor, deriv_inverse_tensor_sym, deriv_squared_tensor,
        deriv_squared_tensor_sym, Deriv1InvariantJ2, Deriv2InvariantJ3, Deriv2InvariantLode, Deriv2InvariantSigmaD,
        Mandel, SamplesTensor2, MN_TO_IJKL, SQRT_2,
    };
    use russell_chk::{approx_eq, deriv_central5};
    use russell_lab::{mat_approx_eq, Matrix};

    // Holds arguments for numerical differentiation corresponding to ∂aiᵢⱼ/∂aₖₗ
    struct ArgsNumDerivInverse {
        a_mat: Matrix, // temporary tensor (3x3 matrix form)
        a: Tensor2,    // temporary tensor
        ai: Tensor2,   // temporary inverse tensor
        i: usize,      // index j of ∂aiᵢⱼ/∂aₖₗ
        j: usize,      // index j of ∂aiᵢⱼ/∂aₖₗ
        k: usize,      // index j of ∂aiᵢⱼ/∂aₖₗ
        l: usize,      // index j of ∂aiᵢⱼ/∂aₖₗ
    }

    // Holds arguments for numerical differentiation corresponding to ∂aiₘ/∂aₙ (Mandel components)
    struct ArgsNumDerivInverseMandel {
        a: Tensor2,  // temporary tensor
        ai: Tensor2, // temporary inverse tensor
        m: usize,    // index of ∂aiₘ/∂aₙ (Mandel components)
        n: usize,    // index of ∂aiₘ/∂aₙ (Mandel components)
    }

    fn component_of_inverse(x: f64, args: &mut ArgsNumDerivInverse) -> f64 {
        let original = args.a_mat.get(args.k, args.l);
        args.a_mat.set(args.k, args.l, x);
        args.a.set_matrix(&args.a_mat).unwrap();
        args.a.inverse(&mut args.ai, 1e-10).unwrap().unwrap();
        args.a_mat.set(args.k, args.l, original);
        args.ai.get(args.i, args.j)
    }

    fn component_of_inverse_mandel(x: f64, args: &mut ArgsNumDerivInverseMandel) -> f64 {
        let original = args.a.vec[args.n];
        args.a.vec[args.n] = x;
        args.a.inverse(&mut args.ai, 1e-10).unwrap().unwrap();
        args.a.vec[args.n] = original;
        args.ai.vec[args.m]
    }

    fn numerical_deriv_inverse(a: &Tensor2) -> Matrix {
        let mut args = ArgsNumDerivInverse {
            a_mat: a.to_matrix(),
            a: Tensor2::new(Mandel::General),
            ai: Tensor2::new(Mandel::General),
            i: 0,
            j: 0,
            k: 0,
            l: 0,
        };
        let mut num_deriv = Matrix::new(9, 9);
        for m in 0..9 {
            for n in 0..9 {
                (args.i, args.j, args.k, args.l) = MN_TO_IJKL[m][n];
                let x = args.a_mat.get(args.k, args.l);
                let res = deriv_central5(x, &mut args, component_of_inverse);
                num_deriv.set(m, n, res);
            }
        }
        num_deriv
    }

    fn numerical_deriv_inverse_mandel(a: &Tensor2) -> Matrix {
        let mut args = ArgsNumDerivInverseMandel {
            a: a.to_general(),
            ai: Tensor2::new(Mandel::General),
            m: 0,
            n: 0,
        };
        let mut num_deriv = Tensor4::new(Mandel::General);
        for m in 0..9 {
            args.m = m;
            for n in 0..9 {
                args.n = n;
                let x = args.a.vec[args.n];
                let res = deriv_central5(x, &mut args, component_of_inverse_mandel);
                num_deriv.mat.set(m, n, res);
            }
        }
        num_deriv.to_matrix()
    }

    fn numerical_deriv_inverse_sym_mandel(a: &Tensor2) -> Matrix {
        let mut args = ArgsNumDerivInverseMandel {
            a: Tensor2::new(Mandel::Symmetric),
            ai: Tensor2::new(Mandel::Symmetric),
            m: 0,
            n: 0,
        };
        args.a.vec[0] = a.vec[0];
        args.a.vec[1] = a.vec[1];
        args.a.vec[2] = a.vec[2];
        args.a.vec[3] = a.vec[3];
        if a.vec.dim() > 4 {
            args.a.vec[4] = a.vec[4];
            args.a.vec[5] = a.vec[5];
        }
        let mut num_deriv = Tensor4::new(Mandel::Symmetric);
        for m in 0..6 {
            args.m = m;
            for n in 0..6 {
                args.n = n;
                let x = args.a.vec[args.n];
                let res = deriv_central5(x, &mut args, component_of_inverse_mandel);
                num_deriv.mat.set(m, n, res);
            }
        }
        num_deriv.to_matrix()
    }

    fn check_deriv_inverse(a: &Tensor2, tol: f64) {
        // compute inverse tensor
        let mut ai = Tensor2::new(a.case());
        a.inverse(&mut ai, 1e-10).unwrap().unwrap();

        // compute analytical derivative
        let mut dd_ana = Tensor4::new(Mandel::General);
        deriv_inverse_tensor(&mut dd_ana, &ai).unwrap();

        // check using index expression
        let arr = dd_ana.to_array();
        let mat = ai.to_matrix();
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    for l in 0..3 {
                        approx_eq(arr[i][j][k][l], -mat.get(i, k) * mat.get(l, j), 1e-14)
                    }
                }
            }
        }

        // check using numerical derivative
        let ana = dd_ana.to_matrix();
        let num = numerical_deriv_inverse(&a);
        let num_mandel = numerical_deriv_inverse_mandel(&a);
        mat_approx_eq(&ana, &num, tol);
        mat_approx_eq(&ana, &num_mandel, tol);
    }

    fn check_deriv_inverse_sym(a: &Tensor2, tol: f64) {
        // compute inverse tensor
        let mut ai = Tensor2::new(a.case());
        a.inverse(&mut ai, 1e-10).unwrap().unwrap();

        // compute analytical derivative
        let mut dd_ana = Tensor4::new(Mandel::Symmetric);
        deriv_inverse_tensor_sym(&mut dd_ana, &ai).unwrap();

        // check using index expression
        let arr = dd_ana.to_array();
        let mat = ai.to_matrix();
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    for l in 0..3 {
                        approx_eq(
                            arr[i][j][k][l],
                            -0.5 * (mat.get(i, k) * mat.get(j, l) + mat.get(i, l) * mat.get(j, k)),
                            1e-14,
                        )
                    }
                }
            }
        }

        // check using numerical derivative
        let ana = dd_ana.to_matrix();
        let num = numerical_deriv_inverse_sym_mandel(&a);
        mat_approx_eq(&ana, &num, tol);
    }

    #[test]
    fn deriv_inverse_tensor_works() {
        // general
        let s = &SamplesTensor2::TENSOR_T;
        let a = Tensor2::from_matrix(&s.matrix, Mandel::General).unwrap();
        check_deriv_inverse(&a, 1e-11);

        // symmetric
        let s = &SamplesTensor2::TENSOR_U;
        let a = Tensor2::from_matrix(&s.matrix, Mandel::Symmetric).unwrap();
        check_deriv_inverse(&a, 1e-7);

        // symmetric 2d
        let s = &SamplesTensor2::TENSOR_Y;
        let a = Tensor2::from_matrix(&s.matrix, Mandel::Symmetric2D).unwrap();
        check_deriv_inverse(&a, 1e-12);
    }

    #[test]
    fn deriv_inverse_tensor_sym_captures_errors() {
        let ai = Tensor2::new(Mandel::General);
        let mut dai_da = Tensor4::new(Mandel::Symmetric);
        assert_eq!(
            deriv_inverse_tensor_sym(&mut dai_da, &ai).err(),
            Some("tensor 'ai' must be Symmetric or Symmetric2D")
        );
        let ai = Tensor2::new(Mandel::Symmetric2D);
        let mut dai_da = Tensor4::new(Mandel::Symmetric2D);
        assert_eq!(
            deriv_inverse_tensor_sym(&mut dai_da, &ai).err(),
            Some("tensor 'dai_da' must be Symmetric")
        );
    }

    #[test]
    fn deriv_inverse_tensor_sym_works() {
        // symmetric
        let s = &SamplesTensor2::TENSOR_U;
        let a = Tensor2::from_matrix(&s.matrix, Mandel::Symmetric).unwrap();
        check_deriv_inverse_sym(&a, 1e-7);

        // symmetric 2d
        let s = &SamplesTensor2::TENSOR_Y;
        let a = Tensor2::from_matrix(&s.matrix, Mandel::Symmetric2D).unwrap();
        check_deriv_inverse_sym(&a, 1e-12);
    }

    // squared tensor ------------------------------------------------------------------------------

    // Holds arguments for numerical differentiation corresponding to ∂a²ᵢⱼ/∂aₖₗ
    struct ArgsNumDerivSquared {
        a_mat: Matrix, // temporary tensor (3x3 matrix form)
        a: Tensor2,    // temporary tensor
        a2: Tensor2,   // temporary squared tensor
        i: usize,      // index j of ∂aiᵢⱼ/∂aₖₗ
        j: usize,      // index j of ∂aiᵢⱼ/∂aₖₗ
        k: usize,      // index j of ∂aiᵢⱼ/∂aₖₗ
        l: usize,      // index j of ∂aiᵢⱼ/∂aₖₗ
    }

    // Holds arguments for numerical differentiation corresponding to ∂a²ₘ/∂aₙ (Mandel components)
    struct ArgsNumDerivSquaredMandel {
        a: Tensor2,  // temporary tensor
        a2: Tensor2, // temporary squared tensor
        m: usize,    // index of ∂aiₘ/∂aₙ (Mandel components)
        n: usize,    // index of ∂aiₘ/∂aₙ (Mandel components)
    }

    fn component_of_squared(x: f64, args: &mut ArgsNumDerivSquared) -> f64 {
        let original = args.a_mat.get(args.k, args.l);
        args.a_mat.set(args.k, args.l, x);
        args.a.set_matrix(&args.a_mat).unwrap();
        args.a.squared(&mut args.a2).unwrap();
        args.a_mat.set(args.k, args.l, original);
        args.a2.get(args.i, args.j)
    }

    fn component_of_squared_mandel(x: f64, args: &mut ArgsNumDerivSquaredMandel) -> f64 {
        let original = args.a.vec[args.n];
        args.a.vec[args.n] = x;
        args.a.squared(&mut args.a2).unwrap();
        args.a.vec[args.n] = original;
        args.a2.vec[args.m]
    }

    fn numerical_deriv_squared(a: &Tensor2) -> Matrix {
        let mut args = ArgsNumDerivSquared {
            a_mat: a.to_matrix(),
            a: Tensor2::new(Mandel::General),
            a2: Tensor2::new(Mandel::General),
            i: 0,
            j: 0,
            k: 0,
            l: 0,
        };
        let mut num_deriv = Matrix::new(9, 9);
        for m in 0..9 {
            for n in 0..9 {
                (args.i, args.j, args.k, args.l) = MN_TO_IJKL[m][n];
                let x = args.a_mat.get(args.k, args.l);
                let res = deriv_central5(x, &mut args, component_of_squared);
                num_deriv.set(m, n, res);
            }
        }
        num_deriv
    }

    fn numerical_deriv_squared_mandel(a: &Tensor2) -> Matrix {
        let mut args = ArgsNumDerivSquaredMandel {
            a: a.to_general(),
            a2: Tensor2::new(Mandel::General),
            m: 0,
            n: 0,
        };
        let mut num_deriv = Tensor4::new(Mandel::General);
        for m in 0..9 {
            args.m = m;
            for n in 0..9 {
                args.n = n;
                let x = args.a.vec[args.n];
                let res = deriv_central5(x, &mut args, component_of_squared_mandel);
                num_deriv.mat.set(m, n, res);
            }
        }
        num_deriv.to_matrix()
    }

    fn numerical_deriv_squared_sym_mandel(a: &Tensor2) -> Matrix {
        let mut args = ArgsNumDerivSquaredMandel {
            a: Tensor2::new(Mandel::Symmetric),
            a2: Tensor2::new(Mandel::Symmetric),
            m: 0,
            n: 0,
        };
        args.a.vec[0] = a.vec[0];
        args.a.vec[1] = a.vec[1];
        args.a.vec[2] = a.vec[2];
        args.a.vec[3] = a.vec[3];
        if a.vec.dim() > 4 {
            args.a.vec[4] = a.vec[4];
            args.a.vec[5] = a.vec[5];
        }
        let mut num_deriv = Tensor4::new(Mandel::Symmetric);
        for m in 0..6 {
            args.m = m;
            for n in 0..6 {
                args.n = n;
                let x = args.a.vec[args.n];
                let res = deriv_central5(x, &mut args, component_of_squared_mandel);
                num_deriv.mat.set(m, n, res);
            }
        }
        num_deriv.to_matrix()
    }

    fn check_deriv_squared(a: &Tensor2, tol: f64) {
        // compute analytical derivative
        let mut dd_ana = Tensor4::new(Mandel::General);
        deriv_squared_tensor(&mut dd_ana, &a).unwrap();

        // check using index expression
        let arr = dd_ana.to_array();
        let mat = a.to_matrix();
        let del = Matrix::diagonal(&[1.0, 1.0, 1.0]);
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    for l in 0..3 {
                        approx_eq(
                            arr[i][j][k][l],
                            mat.get(i, k) * del.get(j, l) + del.get(i, k) * mat.get(l, j),
                            1e-15,
                        )
                    }
                }
            }
        }

        // check using numerical derivative
        let ana = dd_ana.to_matrix();
        let num = numerical_deriv_squared(&a);
        let num_mandel = numerical_deriv_squared_mandel(&a);
        mat_approx_eq(&ana, &num, tol);
        mat_approx_eq(&ana, &num_mandel, tol);
    }

    fn check_deriv_squared_sym(a: &Tensor2, tol: f64) {
        // compute analytical derivative
        let mut dd_ana = Tensor4::new(Mandel::Symmetric);
        deriv_squared_tensor_sym(&mut dd_ana, &a).unwrap();

        // check using index expression
        let arr = dd_ana.to_array();
        let mat = a.to_matrix();
        let del = Matrix::diagonal(&[1.0, 1.0, 1.0]);
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    for l in 0..3 {
                        approx_eq(
                            arr[i][j][k][l],
                            0.5 * (mat.get(i, k) * del.get(j, l)
                                + mat.get(i, l) * del.get(j, k)
                                + del.get(i, k) * mat.get(j, l)
                                + del.get(i, l) * mat.get(j, k)),
                            1e-15,
                        )
                    }
                }
            }
        }

        // check using numerical derivative
        let ana = dd_ana.to_matrix();
        let num = numerical_deriv_squared_sym_mandel(&a);
        mat_approx_eq(&ana, &num, tol);
    }

    #[test]
    fn deriv_squared_tensor_captures_errors() {
        let a = Tensor2::new(Mandel::General);
        let mut da2_da = Tensor4::new(Mandel::Symmetric);
        assert_eq!(
            deriv_squared_tensor(&mut da2_da, &a).err(),
            Some("tensor 'da2_da' must be General")
        );
    }

    #[test]
    fn deriv_squared_tensor_works() {
        // general
        let s = &SamplesTensor2::TENSOR_T;
        let a = Tensor2::from_matrix(&s.matrix, Mandel::General).unwrap();
        check_deriv_squared(&a, 1e-10);

        // symmetric
        let s = &SamplesTensor2::TENSOR_U;
        let a = Tensor2::from_matrix(&s.matrix, Mandel::General).unwrap();
        check_deriv_squared(&a, 1e-10);

        // symmetric 2d
        let s = &SamplesTensor2::TENSOR_Y;
        let a = Tensor2::from_matrix(&s.matrix, Mandel::General).unwrap();
        check_deriv_squared(&a, 1e-10);
    }

    #[test]
    fn deriv_squared_tensor_sym_captures_errors() {
        let a = Tensor2::new(Mandel::General);
        let mut da2_da = Tensor4::new(Mandel::Symmetric);
        assert_eq!(
            deriv_squared_tensor_sym(&mut da2_da, &a).err(),
            Some("tensor 'a' must be Symmetric or Symmetric2D")
        );
        let a = Tensor2::new(Mandel::Symmetric2D);
        let mut da2_da = Tensor4::new(Mandel::Symmetric2D);
        assert_eq!(
            deriv_squared_tensor_sym(&mut da2_da, &a).err(),
            Some("tensor 'da2_da' must be Symmetric")
        );
    }

    #[test]
    fn deriv_squared_tensor_sym_works() {
        // symmetric
        let s = &SamplesTensor2::TENSOR_U;
        let a = Tensor2::from_matrix(&s.matrix, Mandel::Symmetric).unwrap();
        check_deriv_squared_sym(&a, 1e-10);

        // symmetric 2d
        let s = &SamplesTensor2::TENSOR_Y;
        let a = Tensor2::from_matrix(&s.matrix, Mandel::Symmetric2D).unwrap();
        check_deriv_squared_sym(&a, 1e-10);
    }

    // second derivative of invariants -------------------------------------------------------------

    enum Invariant {
        J2,
        J3,
        SigmaD,
        Lode,
    }

    // Holds arguments for numerical differentiation corresponding to [dInvariant²/dσ⊗dσ]ₘₙ (Mandel components)
    struct ArgsNumDeriv2InvariantMandel {
        inv: Invariant, // option
        sigma: Tensor2, // temporary tensor
        // d1: Tensor2,    // dInvariant/dσ (first derivative)
        // s: Tensor2,     // deviator(σ)
        m: usize, // index of [dInvariant²/dσ⊗dσ]ₘₙ (Mandel components)
        n: usize, // index of [dInvariant²/dσ⊗dσ]ₘₙ (Mandel components)
    }

    fn component_of_deriv1_inv_mandel(x: f64, args: &mut ArgsNumDeriv2InvariantMandel) -> f64 {
        let original = args.sigma.vec[args.n];
        args.sigma.vec[args.n] = x;
        let res = match args.inv {
            Invariant::J2 => {
                let mut aux = Deriv1InvariantJ2::new(args.sigma.case()).unwrap();
                aux.compute(&args.sigma).unwrap();
                // args.d1.mirror(&aux.result).unwrap();
                aux.result.vec[args.m]
            } //args.sigma.deriv1_invariant_jj2(&mut args.d1).unwrap(),
            Invariant::J3 => 0.0, //args.sigma.deriv1_invariant_jj3(&mut args.d1, &mut args.s).unwrap(),
            Invariant::SigmaD => {
                // args.sigma.deriv1_invariant_sigma_d(&mut args.d1).unwrap().unwrap();
                0.0
            }
            Invariant::Lode => {
                // args.sigma
                //     .deriv1_invariant_lode(&mut args.d1, &mut args.s)
                //     .unwrap()
                //     .unwrap();
                0.0
            }
        };
        args.sigma.vec[args.n] = original;
        // args.d1.vec[args.m]
        res
    }

    fn numerical_deriv2_inv_sym_mandel(sigma: &Tensor2, inv: Invariant) -> Matrix {
        let mut args = ArgsNumDeriv2InvariantMandel {
            inv,
            sigma: Tensor2::new(Mandel::Symmetric),
            // d1: Tensor2::new(Mandel::Symmetric),
            // s: Tensor2::new(Mandel::Symmetric),
            m: 0,
            n: 0,
        };
        args.sigma.vec[0] = sigma.vec[0];
        args.sigma.vec[1] = sigma.vec[1];
        args.sigma.vec[2] = sigma.vec[2];
        args.sigma.vec[3] = sigma.vec[3];
        if sigma.vec.dim() > 4 {
            args.sigma.vec[4] = sigma.vec[4];
            args.sigma.vec[5] = sigma.vec[5];
        }
        let mut num_deriv = Tensor4::new(Mandel::Symmetric);
        for m in 0..6 {
            args.m = m;
            for n in 0..6 {
                args.n = n;
                let x = args.sigma.vec[args.n];
                let res = deriv_central5(x, &mut args, component_of_deriv1_inv_mandel);
                num_deriv.mat.set(m, n, res);
            }
        }
        num_deriv.to_matrix()
    }

    fn check_deriv2_jj2(sigma: &Tensor2, tol: f64) {
        // compute analytical derivative
        let mut dd2_ana = Tensor4::new(Mandel::Symmetric);
        deriv2_invariant_jj2(&mut dd2_ana, &sigma).unwrap();

        // compare with Psymdev
        let pp_symdev = Tensor4::constant_pp_symdev(true);
        mat_approx_eq(&dd2_ana.mat, &pp_symdev.mat, 1e-15);

        // check using numerical derivative
        let ana = dd2_ana.to_matrix();
        let num = numerical_deriv2_inv_sym_mandel(&sigma, Invariant::J2);
        // println!("{}", ana);
        // println!("{}", num);
        mat_approx_eq(&ana, &num, tol);
    }

    fn check_deriv2_jj3(sigma: &Tensor2, tol: f64) {
        // compute analytical derivative
        let mut dd2_ana = Tensor4::new(Mandel::Symmetric);
        let mut aux = Deriv2InvariantJ3::new(sigma.case()).unwrap();
        aux.compute(&mut dd2_ana, &sigma).unwrap();

        // check using numerical derivative
        let ana = dd2_ana.to_matrix();
        let num = numerical_deriv2_inv_sym_mandel(&sigma, Invariant::J3);
        // println!("{}", ana);
        // println!("{}", num);
        mat_approx_eq(&ana, &num, tol);
    }

    fn check_deriv2_sigma_d(sigma: &Tensor2, tol: f64) {
        // compute analytical derivative
        let mut dd2_ana = Tensor4::new(Mandel::Symmetric);
        let mut aux = Deriv2InvariantSigmaD::new(sigma.case()).unwrap();
        aux.compute(&mut dd2_ana, &sigma).unwrap().unwrap();

        // check using numerical derivative
        let ana = dd2_ana.to_matrix();
        let num = numerical_deriv2_inv_sym_mandel(&sigma, Invariant::SigmaD);
        // println!("{}", ana);
        // println!("{}", num);
        mat_approx_eq(&ana, &num, tol);
    }

    fn check_deriv2_lode(sigma: &Tensor2, tol: f64) {
        // compute analytical derivative
        let mut dd2_ana = Tensor4::new(Mandel::Symmetric);
        let mut aux = Deriv2InvariantLode::new(sigma.case()).unwrap();
        aux.compute(&mut dd2_ana, &sigma).unwrap().unwrap();

        // check using numerical derivative
        let ana = dd2_ana.to_matrix();
        let num = numerical_deriv2_inv_sym_mandel(&sigma, Invariant::Lode);
        // println!("{}", ana);
        // println!("{}", num);
        mat_approx_eq(&ana, &num, tol);
    }

    #[test]
    fn deriv2_invariant_jj2_works() {
        // symmetric
        let sigma = Tensor2::from_matrix(&SamplesTensor2::TENSOR_U.matrix, Mandel::Symmetric).unwrap();
        check_deriv2_jj2(&sigma, 1e-11);

        // symmetric
        let sigma = Tensor2::from_matrix(&SamplesTensor2::TENSOR_S.matrix, Mandel::Symmetric).unwrap();
        check_deriv2_jj2(&sigma, 1e-11);

        // symmetric 2d
        let sigma = Tensor2::from_matrix(&SamplesTensor2::TENSOR_X.matrix, Mandel::Symmetric).unwrap();
        check_deriv2_jj2(&sigma, 1e-11);

        // symmetric 2d
        let sigma = Tensor2::from_matrix(&SamplesTensor2::TENSOR_Y.matrix, Mandel::Symmetric).unwrap();
        check_deriv2_jj2(&sigma, 1e-11);

        // zero
        let sigma = Tensor2::from_matrix(&SamplesTensor2::TENSOR_O.matrix, Mandel::Symmetric).unwrap();
        check_deriv2_jj2(&sigma, 1e-15);

        // one
        let sigma = Tensor2::from_matrix(&SamplesTensor2::TENSOR_I.matrix, Mandel::Symmetric).unwrap();
        check_deriv2_jj2(&sigma, 1e-12);
    }

    #[test]
    fn deriv2_invariant_jj3_works() {
        // symmetric
        let sigma = Tensor2::from_matrix(&SamplesTensor2::TENSOR_U.matrix, Mandel::Symmetric).unwrap();
        check_deriv2_jj3(&sigma, 1e-10);

        // symmetric
        let sigma = Tensor2::from_matrix(&SamplesTensor2::TENSOR_S.matrix, Mandel::Symmetric).unwrap();
        check_deriv2_jj3(&sigma, 1e-10);

        // symmetric 2d
        let sigma = Tensor2::from_matrix(&SamplesTensor2::TENSOR_X.matrix, Mandel::Symmetric).unwrap();
        check_deriv2_jj3(&sigma, 1e-10);

        // symmetric 2d
        let sigma = Tensor2::from_matrix(&SamplesTensor2::TENSOR_Y.matrix, Mandel::Symmetric).unwrap();
        check_deriv2_jj3(&sigma, 1e-10);

        // zero
        let sigma = Tensor2::from_matrix(&SamplesTensor2::TENSOR_O.matrix, Mandel::Symmetric).unwrap();
        check_deriv2_jj3(&sigma, 1e-15);

        // one
        let sigma = Tensor2::from_matrix(&SamplesTensor2::TENSOR_I.matrix, Mandel::Symmetric).unwrap();
        check_deriv2_jj3(&sigma, 1e-13);
    }

    #[test]
    fn deriv2_invariant_sigma_d_returns_none() {
        // identity
        let sigma = Tensor2::from_matrix(&SamplesTensor2::TENSOR_I.matrix, Mandel::Symmetric).unwrap();
        let mut d2 = Tensor4::new(Mandel::Symmetric);
        let mut aux = Deriv2InvariantSigmaD::new(sigma.case()).unwrap();
        assert_eq!(aux.compute(&mut d2, &sigma).unwrap(), None);
    }

    #[test]
    fn deriv2_invariant_sigma_d_works() {
        // symmetric
        let sigma = Tensor2::from_matrix(&SamplesTensor2::TENSOR_U.matrix, Mandel::Symmetric).unwrap();
        check_deriv2_sigma_d(&sigma, 1e-11);
    }

    #[test]
    fn deriv2_invariant_lode_returns_none() {
        // identity
        let sigma = Tensor2::from_matrix(&SamplesTensor2::TENSOR_I.matrix, Mandel::Symmetric).unwrap();
        let mut d2 = Tensor4::new(Mandel::Symmetric);
        let mut aux = Deriv2InvariantLode::new(sigma.case()).unwrap();
        assert_eq!(aux.compute(&mut d2, &sigma).unwrap(), None);
    }

    #[test]
    fn deriv2_invariant_lode_works() {
        // symmetric
        let sigma = Tensor2::from_matrix(&SamplesTensor2::TENSOR_U.matrix, Mandel::Symmetric).unwrap();
        check_deriv2_lode(&sigma, 1e-10);
    }

    #[test]
    fn deriv2_invariant_jj2_captures_errors() {
        let sigma = Tensor2::new(Mandel::General);
        let mut d2 = Tensor4::new(Mandel::Symmetric);
        assert_eq!(
            deriv2_invariant_jj2(&mut d2, &sigma).err(),
            Some("tensor 'sigma' must be Symmetric or Symmetric2D")
        );
        let sigma = Tensor2::new(Mandel::Symmetric2D);
        let mut d2 = Tensor4::new(Mandel::Symmetric2D);
        assert_eq!(
            deriv2_invariant_jj2(&mut d2, &sigma).err(),
            Some("tensor 'd2' must be Symmetric")
        );
    }

    #[test]
    fn second_deriv_jj3_handles_errors() {
        assert_eq!(
            Deriv2InvariantJ3::new(Mandel::General).err(),
            Some("case must be Symmetric or Symmetric2D")
        );
        let mut aux = Deriv2InvariantJ3::new(Mandel::Symmetric).unwrap();
        let mut d2 = Tensor4::new(Mandel::Symmetric);
        let sigma = Tensor2::new(Mandel::General);
        assert_eq!(
            aux.compute(&mut d2, &sigma).err(),
            Some("tensor 'sigma' is incompatible")
        );
        let sigma = Tensor2::new(Mandel::Symmetric2D);
        assert_eq!(
            aux.compute(&mut d2, &sigma).err(),
            Some("tensor 'sigma' is incompatible")
        );
        let sigma = Tensor2::new(Mandel::Symmetric);
        let mut d2 = Tensor4::new(Mandel::Symmetric2D);
        assert_eq!(
            aux.compute(&mut d2, &sigma).err(),
            Some("tensor 'd2' must be Symmetric")
        );
    }

    #[test]
    fn second_deriv_sigma_d_handles_errors() {
        assert_eq!(
            Deriv2InvariantSigmaD::new(Mandel::General).err(),
            Some("case must be Symmetric or Symmetric2D")
        );
        let mut aux = Deriv2InvariantSigmaD::new(Mandel::Symmetric).unwrap();
        let mut d2 = Tensor4::new(Mandel::Symmetric);
        let sigma = Tensor2::new(Mandel::General);
        assert_eq!(
            aux.compute(&mut d2, &sigma).err(),
            Some("tensor 'sigma' is incompatible")
        );
        let sigma = Tensor2::new(Mandel::Symmetric2D);
        assert_eq!(
            aux.compute(&mut d2, &sigma).err(),
            Some("tensor 'sigma' is incompatible")
        );
        let sigma = Tensor2::new(Mandel::Symmetric);
        let mut d2 = Tensor4::new(Mandel::Symmetric2D);
        assert_eq!(
            aux.compute(&mut d2, &sigma).err(),
            Some("tensor 'd2' must be Symmetric")
        );
    }

    #[test]
    fn second_deriv_lode_handles_errors() {
        assert_eq!(
            Deriv2InvariantLode::new(Mandel::General).err(),
            Some("case must be Symmetric or Symmetric2D")
        );
        let mut aux = Deriv2InvariantLode::new(Mandel::Symmetric).unwrap();
        let mut d2 = Tensor4::new(Mandel::Symmetric);
        let sigma = Tensor2::new(Mandel::General);
        assert_eq!(
            aux.compute(&mut d2, &sigma).err(),
            Some("tensor 'sigma' is incompatible")
        );
        let sigma = Tensor2::new(Mandel::Symmetric2D);
        assert_eq!(
            aux.compute(&mut d2, &sigma).err(),
            Some("tensor 'sigma' is incompatible")
        );
        let sigma = Tensor2::new(Mandel::Symmetric);
        let mut d2 = Tensor4::new(Mandel::Symmetric2D);
        assert_eq!(
            aux.compute(&mut d2, &sigma).err(),
            Some("tensor 'd2' must be Symmetric")
        );
    }

    #[test]
    fn example_second_deriv_jj3_lode() {
        let sigma = Tensor2::from_matrix(&SamplesTensor2::TENSOR_U.matrix, Mandel::Symmetric).unwrap();
        let mut s = Tensor2::new(Mandel::Symmetric);
        sigma.deviator(&mut s).unwrap();
        let mut aux = Deriv2InvariantJ3::new(sigma.case()).unwrap();
        let mut d2 = Tensor4::new(Mandel::Symmetric);
        aux.compute(&mut d2, &sigma).unwrap();

        // println!("sigma =\n{:.1}", sigma.to_matrix());
        // println!("sigma_mandel =\n{}", sigma.vec);
        // println!("s = \n{}", s.vec);
        // println!("d2 = \n{}", d2.mat);

        #[rustfmt::skip]
        let correct = [
            [-16.0/9.0        ,  14.0/9.0      ,   2.0/9.0       ,  2.0*SQRT_2/3.0 , -10.0*SQRT_2/3.0 , SQRT_2      ], 
            [ 14.0/9.0        ,   2.0/9.0      , -16.0/9.0       ,  2.0*SQRT_2/3.0 , 5.0*SQRT_2/3.0   , -2.0*SQRT_2 ], 
            [  2.0/9.0        , -16.0/9.0      ,  14.0/9.0       , -4.0*SQRT_2/3.0 , 5.0*SQRT_2/3.0   , SQRT_2      ], 
            [  2.0*SQRT_2/3.0 , 2.0*SQRT_2/3.0 , -4.0*SQRT_2/3.0 , -7.0/3.0        , 3.0              , 5.0         ], 
            [-10.0*SQRT_2/3.0 , 5.0*SQRT_2/3.0 ,  5.0*SQRT_2/3.0 ,  3.0            , 8.0/3.0          , 2.0         ], 
            [      SQRT_2     ,-2.0*SQRT_2     ,      SQRT_2     ,  5.0            , 2.0              , -1.0/3.0    ],
        ];
        mat_approx_eq(&d2.mat, &correct, 1e-15);

        let mut aux = Deriv2InvariantLode::new(sigma.case()).unwrap();
        aux.compute(&mut d2, &sigma).unwrap();

        // println!("d2 = \n{}", d2.mat);

        #[rustfmt::skip]
        let correct = [
            [-0.039528347708134,  0.0237434792780289,   0.0157848684301052,  0.0136392037983506, -0.0354377940510052,  0.0131589501434791],
            [0.0237434792780289, -0.0200332341113984,  -0.00371024516663052, 0.00899921464051518, 0.0234105185455438, -0.0229302648906723],
            [0.0157848684301052, -0.00371024516663052, -0.0120746232634746, -0.0226384184388658,  0.0120272755054614,  0.00977131474719321],
            [0.0136392037983506,  0.00899921464051518, -0.0226384184388658, -0.0635034452012119,  0.0103061398245104,  0.0374455252630319],
            [-0.0354377940510052, 0.0234105185455438,   0.0120272755054614,  0.0103061398245104, -0.0308487598599826,  0.0128121444219201],
            [0.0131589501434791, -0.0229302648906723,   0.00977131474719321, 0.0374455252630319,  0.0128121444219201, -0.0345929640882181],
        ];
        mat_approx_eq(&d2.mat, &correct, 1e-16);
    }
}
