use super::Matrix;
use crate::{to_i32, StrError, CBLAS_COL_MAJOR, CBLAS_NO_TRANS};

extern "C" {
    fn cblas_dgemm(
        layout: i32,
        transa: i32,
        transb: i32,
        m: i32,
        n: i32,
        k: i32,
        alpha: f64,
        a: *const f64,
        lda: i32,
        b: *const f64,
        ldb: i32,
        beta: f64,
        c: *mut f64,
        ldc: i32,
    );
}

/// Performs the matrix-matrix multiplication
///
/// ```text
///   c  :=  α ⋅  a   ⋅   b
/// (m,n)       (m,k)   (k,n)
/// ```
///
/// # Example
///
/// ```
/// use russell_lab::{mat_mat_mul, Matrix, StrError};
///
/// fn main() -> Result<(), StrError> {
///     let a = Matrix::from(&[
///         [1.0, 2.0],
///         [3.0, 4.0],
///         [5.0, 6.0],
///     ]);
///     let b = Matrix::from(&[
///         [-1.0, -2.0, -3.0],
///         [-4.0, -5.0, -6.0],
///     ]);
///     let mut c = Matrix::new(3, 3);
///     mat_mat_mul(&mut c, 1.0, &a, &b);
///     let correct = "┌             ┐\n\
///                    │  -9 -12 -15 │\n\
///                    │ -19 -26 -33 │\n\
///                    │ -29 -40 -51 │\n\
///                    └             ┘";
///     assert_eq!(format!("{}", c), correct);
///     Ok(())
/// }
/// ```
pub fn mat_mat_mul(c: &mut Matrix, alpha: f64, a: &Matrix, b: &Matrix) -> Result<(), StrError> {
    let (m, n) = c.dims();
    let k = a.ncol();
    if a.nrow() != m || b.nrow() != k || b.ncol() != n {
        return Err("matrices are incompatible");
    }
    if m == 0 || n == 0 {
        return Ok(());
    }
    if k == 0 {
        c.fill(0.0);
        return Ok(());
    }
    let m_i32: i32 = to_i32(m);
    let n_i32: i32 = to_i32(n);
    let k_i32: i32 = to_i32(k);
    let lda = m_i32;
    let ldb = k_i32;
    unsafe {
        cblas_dgemm(
            CBLAS_COL_MAJOR,
            CBLAS_NO_TRANS,
            CBLAS_NO_TRANS,
            m_i32,
            n_i32,
            k_i32,
            alpha,
            a.as_data().as_ptr(),
            lda,
            b.as_data().as_ptr(),
            ldb,
            0.0,
            c.as_mut_data().as_mut_ptr(),
            m_i32,
        );
    }
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::{mat_mat_mul, Matrix};
    use crate::mat_approx_eq;

    #[test]
    fn mat_mat_mul_fails_on_wrong_dims() {
        let a_2x1 = Matrix::new(2, 1);
        let a_1x2 = Matrix::new(1, 2);
        let b_2x1 = Matrix::new(2, 1);
        let b_1x3 = Matrix::new(1, 3);
        let mut c_2x2 = Matrix::new(2, 2);
        assert_eq!(
            mat_mat_mul(&mut c_2x2, 1.0, &a_2x1, &b_2x1),
            Err("matrices are incompatible")
        );
        assert_eq!(
            mat_mat_mul(&mut c_2x2, 1.0, &a_1x2, &b_2x1),
            Err("matrices are incompatible")
        );
        assert_eq!(
            mat_mat_mul(&mut c_2x2, 1.0, &a_2x1, &b_1x3),
            Err("matrices are incompatible")
        );
    }

    #[test]
    fn mat_mat_mul_0x0_works() {
        let a = Matrix::new(0, 0);
        let b = Matrix::new(0, 0);
        let mut c = Matrix::new(0, 0);
        mat_mat_mul(&mut c, 2.0, &a, &b).unwrap();

        let a = Matrix::new(1, 0);
        let b = Matrix::new(0, 1);
        let mut c = Matrix::new(1, 1);
        mat_mat_mul(&mut c, 2.0, &a, &b).unwrap();
        let correct = &[
            [0.0], //
        ];
        mat_approx_eq(&c, correct, 1e-15);
    }

    #[test]
    fn mat_mat_mul_works() {
        let a = Matrix::from(&[
            // 2 x 3
            [1.0, 2.00, 3.0],
            [0.5, 0.75, 1.5],
        ]);
        let b = Matrix::from(&[
            // 3 x 4
            [0.1, 0.5, 0.5, 0.75],
            [0.2, 2.0, 2.0, 2.00],
            [0.3, 0.5, 0.5, 0.50],
        ]);
        let mut c = Matrix::new(2, 4);
        // c := 2⋅a⋅b
        mat_mat_mul(&mut c, 2.0, &a, &b).unwrap();
        #[rustfmt::skip]
        let correct = &[
            [2.80, 12.0, 12.0, 12.50],
            [1.30,  5.0,  5.0, 5.25],
        ];
        mat_approx_eq(&c, correct, 1e-15);
    }
}
