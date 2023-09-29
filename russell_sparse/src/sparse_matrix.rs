use super::{to_i32, CooMatrix, CscMatrix, CsrMatrix, Symmetry};
use crate::StrError;
use russell_lab::{Matrix, Vector};
use russell_openblas::idamax;

pub struct SparseMatrix {
    coo: Option<CooMatrix>,
    csc: Option<CscMatrix>,
    csr: Option<CsrMatrix>,
}

impl SparseMatrix {
    pub fn new_coo(
        nrow: usize,
        ncol: usize,
        max_nnz: usize,
        symmetry: Option<Symmetry>,
        one_based: bool,
    ) -> Result<Self, StrError> {
        Ok(SparseMatrix {
            coo: Some(CooMatrix::new(nrow, ncol, max_nnz, symmetry, one_based)?),
            csc: None,
            csr: None,
        })
    }

    pub fn new_csc(
        nrow: usize,
        ncol: usize,
        col_pointers: Vec<i32>,
        row_indices: Vec<i32>,
        values: Vec<f64>,
        symmetry: Option<Symmetry>,
    ) -> Result<Self, StrError> {
        Ok(SparseMatrix {
            coo: None,
            csc: Some(CscMatrix::new(nrow, ncol, col_pointers, row_indices, values, symmetry)?),
            csr: None,
        })
    }

    pub fn new_csr(
        nrow: usize,
        ncol: usize,
        row_pointers: Vec<i32>,
        col_indices: Vec<i32>,
        values: Vec<f64>,
        symmetry: Option<Symmetry>,
    ) -> Result<Self, StrError> {
        Ok(SparseMatrix {
            coo: None,
            csc: None,
            csr: Some(CsrMatrix::new(nrow, ncol, row_pointers, col_indices, values, symmetry)?),
        })
    }

    pub fn from_coo(coo: CooMatrix) -> Self {
        SparseMatrix {
            coo: Some(coo),
            csc: None,
            csr: None,
        }
    }

    pub fn from_csc(csc: CscMatrix) -> Self {
        SparseMatrix {
            coo: None,
            csc: Some(csc),
            csr: None,
        }
    }

    pub fn from_csr(csr: CsrMatrix) -> Self {
        SparseMatrix {
            coo: None,
            csc: None,
            csr: Some(csr),
        }
    }

    /// Returns information about the dimensions and symmetry type
    ///
    /// Returns `(nrow, ncol, nnz, symmetry)`
    ///
    /// Priority: CSC -> CSR -> COO
    pub fn get_info(&self) -> (usize, usize, usize, Option<Symmetry>) {
        match &self.csc {
            Some(csc) => csc.get_info(),
            None => match &self.csr {
                Some(csr) => csr.get_info(),
                None => self.coo.as_ref().unwrap().get_info(), // unwrap OK because at least one mat must be available
            },
        }
    }

    /// Returns the maximum absolute value among all values
    ///
    /// Priority: CSC -> CSR -> COO
    pub fn get_max_abs_value(&self) -> f64 {
        let values = match &self.csc {
            Some(csc) => &csc.values,
            None => match &self.csr {
                Some(csr) => &csr.values,
                None => &self.coo.as_ref().unwrap().values, // unwrap OK because at least one mat must be available
            },
        };
        let n = to_i32(values.len());
        let idx = idamax(n, values, 1);
        f64::abs(values[idx as usize])
    }

    /// Performs the matrix-vector multiplication
    ///
    /// Priority: CSC -> CSR -> COO
    pub fn mat_vec_mul(&self, v: &mut Vector, alpha: f64, u: &Vector) -> Result<(), StrError> {
        match &self.csc {
            Some(csc) => csc.mat_vec_mul(v, alpha, u),
            None => match &self.csr {
                Some(csr) => csr.mat_vec_mul(v, alpha, u),
                None => self.coo.as_ref().unwrap().mat_vec_mul(v, alpha, u), // unwrap OK because at least one mat must be available
            },
        }
    }

    pub fn as_dense(&self) -> Matrix {
        match &self.csc {
            Some(csc) => csc.as_dense(),
            None => match &self.csr {
                Some(csr) => csr.as_dense(),
                None => self.coo.as_ref().unwrap().as_dense(), // unwrap OK because at least one mat must be available
            },
        }
    }

    pub fn to_dense(&self, a: &mut Matrix) -> Result<(), StrError> {
        match &self.csc {
            Some(csc) => csc.to_dense(a),
            None => match &self.csr {
                Some(csr) => csr.to_dense(a),
                None => self.coo.as_ref().unwrap().to_dense(a), // unwrap OK because at least one mat must be available
            },
        }
    }

    // COO ------------------------------------------------------------------------

    pub fn put(&mut self, i: usize, j: usize, aij: f64) -> Result<(), StrError> {
        match &mut self.coo {
            Some(coo) => coo.put(i, j, aij),
            None => Err("COO matrix is not available to put items"),
        }
    }

    pub fn reset(&mut self) -> Result<(), StrError> {
        match &mut self.coo {
            Some(coo) => {
                coo.reset();
                Ok(())
            }
            None => Err("COO matrix is not available to reset nnz counter"),
        }
    }

    pub fn get_coo(&self) -> Result<&CooMatrix, StrError> {
        match &self.coo {
            Some(coo) => Ok(coo),
            None => Err("COO matrix is not available"),
        }
    }

    pub fn get_coo_mut(&mut self) -> Result<&mut CooMatrix, StrError> {
        match &mut self.coo {
            Some(coo) => Ok(coo),
            None => Err("COO matrix is not available"),
        }
    }

    // CSC ------------------------------------------------------------------------

    pub fn get_csc(&self) -> Result<&CscMatrix, StrError> {
        match &self.csc {
            Some(csc) => Ok(csc),
            None => Err("CSC matrix is not available"),
        }
    }

    pub fn get_csc_mut(&mut self) -> Result<&mut CscMatrix, StrError> {
        match &mut self.csc {
            Some(csc) => Ok(csc),
            None => Err("CSC matrix is not available"),
        }
    }

    /// Returns the CSC or creates a CSC from COO or updates the CSC from COO
    ///
    /// Priority: COO -> CSC
    pub fn get_csc_or_from_coo(&mut self) -> Result<&CscMatrix, StrError> {
        match &self.coo {
            Some(coo) => match &mut self.csc {
                Some(csc) => {
                    csc.update_from_coo(coo)?;
                    Ok(self.csc.as_ref().unwrap())
                }
                None => {
                    self.csc = Some(CscMatrix::from_coo(coo)?);
                    Ok(self.csc.as_ref().unwrap())
                }
            },
            None => match &self.csc {
                Some(csc) => Ok(csc),
                None => Err("CSC is not available and COO matrix is not available to convert to CSC"),
            },
        }
    }

    // CSR ------------------------------------------------------------------------

    pub fn get_csr(&self) -> Result<&CsrMatrix, StrError> {
        match &self.csr {
            Some(csr) => Ok(csr),
            None => Err("CSR matrix is not available"),
        }
    }

    pub fn get_csr_mut(&mut self) -> Result<&mut CsrMatrix, StrError> {
        match &mut self.csr {
            Some(csr) => Ok(csr),
            None => Err("CSR matrix is not available"),
        }
    }

    /// Returns the CSR or creates a CSR from COO or updates the CSR from COO
    ///
    /// Priority: COO -> CSR
    pub fn get_csr_or_from_coo(&mut self) -> Result<&CsrMatrix, StrError> {
        match &self.coo {
            Some(coo) => match &mut self.csr {
                Some(csr) => {
                    csr.update_from_coo(coo)?;
                    Ok(self.csr.as_ref().unwrap())
                }
                None => {
                    self.csr = Some(CsrMatrix::from_coo(coo)?);
                    Ok(self.csr.as_ref().unwrap())
                }
            },
            None => match &self.csr {
                Some(csr) => Ok(csr),
                None => Err("CSR is not available and COO matrix is not available to convert to CSR"),
            },
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::SparseMatrix;
    use crate::Samples;
    use russell_chk::vec_approx_eq;
    use russell_lab::{Matrix, Vector};

    #[test]
    fn new_functions_work() {
        // COO
        SparseMatrix::new_coo(1, 1, 1, None, false).unwrap();
        assert_eq!(
            SparseMatrix::new_coo(0, 1, 1, None, false).err(),
            Some("nrow must be ≥ 1")
        );
        // CSC
        SparseMatrix::new_csc(1, 1, vec![0, 1], vec![0], vec![0.0], None).unwrap();
        assert_eq!(
            SparseMatrix::new_csc(0, 1, vec![0, 1], vec![0], vec![0.0], None).err(),
            Some("nrow must be ≥ 1")
        );
        // CSR
        SparseMatrix::new_csr(1, 1, vec![0, 1], vec![0], vec![0.0], None).unwrap();
        assert_eq!(
            SparseMatrix::new_csr(0, 1, vec![0, 1], vec![0], vec![0.0], None).err(),
            Some("nrow must be ≥ 1")
        );
    }

    #[test]
    fn getters_work() {
        // un-mutable
        // ┌       ┐
        // │ 10 20 │
        // └       ┘
        let (coo, csc, csr, _) = Samples::rectangular_1x2(false, false, false);
        let mut a = Matrix::new(1, 2);
        let x = Vector::from(&[2.0, 1.0]);
        let mut wrong = Vector::new(2);
        // COO
        let coo_mat = SparseMatrix::from_coo(coo);
        assert_eq!(coo_mat.get_info(), (1, 2, 2, None));
        assert_eq!(coo_mat.get_max_abs_value(), 20.0);
        assert_eq!(coo_mat.get_coo().unwrap().get_info(), (1, 2, 2, None));
        assert_eq!(coo_mat.get_csc().err(), Some("CSC matrix is not available"));
        assert_eq!(coo_mat.get_csr().err(), Some("CSR matrix is not available"));
        // CSC
        let csc_mat = SparseMatrix::from_csc(csc);
        assert_eq!(csc_mat.get_info(), (1, 2, 2, None));
        assert_eq!(csc_mat.get_max_abs_value(), 20.0);
        assert_eq!(csc_mat.get_csc().unwrap().get_info(), (1, 2, 2, None));
        assert_eq!(csc_mat.get_coo().err(), Some("COO matrix is not available"));
        assert_eq!(csc_mat.get_csr().err(), Some("CSR matrix is not available"));
        // CSR
        let csr_mat = SparseMatrix::from_csr(csr);
        assert_eq!(csr_mat.get_info(), (1, 2, 2, None));
        assert_eq!(csr_mat.get_max_abs_value(), 20.0);
        assert_eq!(csr_mat.get_csr().unwrap().get_info(), (1, 2, 2, None));
        assert_eq!(csr_mat.get_csc().err(), Some("CSC matrix is not available"));
        assert_eq!(csr_mat.get_coo().err(), Some("COO matrix is not available"));

        // COO, CSC, CSR
        let mut ax = Vector::new(1);
        for mat in [&coo_mat, &csc_mat, &csr_mat] {
            mat.mat_vec_mul(&mut ax, 2.0, &x).unwrap();
            vec_approx_eq(&ax.as_data(), &[80.0], 1e-15);
            assert_eq!(
                mat.mat_vec_mul(&mut wrong, 1.0, &x).err(),
                Some("v.ndim must equal nrow")
            );
            mat.to_dense(&mut a).unwrap();
            assert_eq!(a.dims(), (1, 2));
            assert_eq!(a.get(0, 0), 10.0);
            assert_eq!(a.get(0, 1), 20.0);
            let aa = mat.as_dense();
            assert_eq!(aa.dims(), (1, 2));
            assert_eq!(aa.get(0, 0), 10.0);
            assert_eq!(aa.get(0, 1), 20.0);
        }
    }

    #[test]
    fn setters_work() {
        // mutable
        // ┌       ┐
        // │ 10 20 │
        // └       ┘
        let (coo, csc, csr, _) = Samples::rectangular_1x2(false, false, false);
        // COO
        let mut coo_mat = SparseMatrix::from_coo(coo);
        assert_eq!(coo_mat.get_coo_mut().unwrap().get_info(), (1, 2, 2, None));
        assert_eq!(coo_mat.get_csc_mut().err(), Some("CSC matrix is not available"));
        assert_eq!(coo_mat.get_csr_mut().err(), Some("CSR matrix is not available"));
        // CSC
        let mut csc_mat = SparseMatrix::from_csc(csc);
        assert_eq!(csc_mat.get_csc_mut().unwrap().get_info(), (1, 2, 2, None));
        assert_eq!(csc_mat.get_coo_mut().err(), Some("COO matrix is not available"));
        assert_eq!(csc_mat.get_csr_mut().err(), Some("CSR matrix is not available"));
        assert_eq!(csc_mat.get_csc_or_from_coo().unwrap().get_info(), (1, 2, 2, None));
        assert_eq!(
            csc_mat.get_csr_or_from_coo().err(),
            Some("CSR is not available and COO matrix is not available to convert to CSR")
        );
        assert_eq!(
            csc_mat.put(0, 0, 0.0).err(),
            Some("COO matrix is not available to put items")
        );
        assert_eq!(
            csc_mat.reset().err(),
            Some("COO matrix is not available to reset nnz counter")
        );
        // CSR
        let mut csr_mat = SparseMatrix::from_csr(csr);
        assert_eq!(csr_mat.get_csr_mut().unwrap().get_info(), (1, 2, 2, None));
        assert_eq!(csr_mat.get_csc_mut().err(), Some("CSC matrix is not available"));
        assert_eq!(csr_mat.get_coo_mut().err(), Some("COO matrix is not available"));
        assert_eq!(csr_mat.get_csr_or_from_coo().unwrap().get_info(), (1, 2, 2, None));
        assert_eq!(
            csr_mat.get_csc_or_from_coo().err(),
            Some("CSC is not available and COO matrix is not available to convert to CSC")
        );
        assert_eq!(
            csr_mat.put(0, 0, 0.0).err(),
            Some("COO matrix is not available to put items")
        );
        assert_eq!(
            csr_mat.reset().err(),
            Some("COO matrix is not available to reset nnz counter")
        );

        // COO
        let mut coo = SparseMatrix::new_coo(2, 2, 1, None, false).unwrap();
        coo.put(0, 0, 1.0).unwrap();
        assert_eq!(
            coo.put(1, 1, 2.0).err(),
            Some("COO matrix: max number of items has been reached")
        );
        coo.reset().unwrap();
        coo.put(1, 1, 2.0).unwrap();
    }
}
