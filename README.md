# Russell - Rust Scientific Library

[![codecov](https://codecov.io/gh/cpmech/russell/branch/main/graph/badge.svg?token=PQWSKMZQXT)](https://codecov.io/gh/cpmech/russell)

![Bertrand Russell](zassets/Bertrand_Russell_1957.jpg)

([CC0](http://creativecommons.org/publicdomain/zero/1.0/deed.en). Photo: [Bertrand Russell](https://en.wikipedia.org/wiki/Bertrand_Russell))

**Russell** assists in the development of scientific computations using the Rust language. We focus on numerical methods and solvers for differential equations; however, anything is possible 😉.

An essential goal of this library is to bring the best (fastest) solutions while maintaining a very **clean** (and idiomatic) code, thoroughly tested (min coverage of 95%), and yet simple to use. The best solutions are brought by wrapping **powerful** libraries such as OpenBLAS, MUMPS, and SuiteSparse (UMFPACK).

Available crates:

- [![Crates.io](https://img.shields.io/crates/v/russell_chk.svg)](https://crates.io/crates/russell_chk) [chk](https://github.com/cpmech/russell/tree/main/russell_chk) Functions to check vectors and other data in tests
- [![Crates.io](https://img.shields.io/crates/v/russell_lab.svg)](https://crates.io/crates/russell_lab) [lab](https://github.com/cpmech/russell/tree/main/russell_lab) Matrix-vector laboratory including linear algebra tools
- [![Crates.io](https://img.shields.io/crates/v/russell_openblas.svg)](https://crates.io/crates/russell_openblas) [openblas](https://github.com/cpmech/russell/tree/main/russell_openblas) Thin wrapper to some OpenBLAS routines
- [![Crates.io](https://img.shields.io/crates/v/russell_sparse.svg)](https://crates.io/crates/russell_sparse) [sparse](https://github.com/cpmech/russell/tree/main/russell_sparse) Sparse matrix tools and solvers
- [![Crates.io](https://img.shields.io/crates/v/russell_stat.svg)](https://crates.io/crates/russell_stat) [stat](https://github.com/cpmech/russell/tree/main/russell_stat) Statistics calculations, probability distributions, and pseudo random numbers
- [![Crates.io](https://img.shields.io/crates/v/russell_tensor.svg)](https://crates.io/crates/russell_tensor) [tensor](https://github.com/cpmech/russell/tree/main/russell_tensor) Tensor analysis structures and functions for continuum mechanics

External recommended crate:

- [plotpy](https://github.com/cpmech/plotpy) Plotting tools using Python3/Matplotlib as an engine

## Examples

### Compute a singular value decomposition

```rust
use russell_lab::{sv_decomp, Matrix, Vector, StrError};

fn main() -> Result<(), StrError> {
    // set matrix
    let mut a = Matrix::from(&[
        [2.0, 4.0],
        [1.0, 3.0],
        [0.0, 0.0],
        [0.0, 0.0],
    ]);

    // allocate output structures
    let (m, n) = a.dims();
    let min_mn = if m < n { m } else { n };
    let mut s = Vector::new(min_mn);
    let mut u = Matrix::new(m, m);
    let mut vt = Matrix::new(n, n);

    // perform SVD
    sv_decomp(&mut s, &mut u, &mut vt, &mut a)?;

    // define correct data
    let s_correct = "┌      ┐\n\
                     │ 5.46 │\n\
                     │ 0.37 │\n\
                     └      ┘";
    let u_correct = "┌                         ┐\n\
                     │ -0.82 -0.58  0.00  0.00 │\n\
                     │ -0.58  0.82  0.00  0.00 │\n\
                     │  0.00  0.00  1.00  0.00 │\n\
                     │  0.00  0.00  0.00  1.00 │\n\
                     └                         ┘";
    let vt_correct = "┌             ┐\n\
                      │ -0.40 -0.91 │\n\
                      │ -0.91  0.40 │\n\
                      └             ┘";

    // check solution
    assert_eq!(format!("{:.2}", s), s_correct);
    assert_eq!(format!("{:.2}", u), u_correct);
    assert_eq!(format!("{:.2}", vt), vt_correct);

    // check SVD: a == u * s * vt
    let mut usv = Matrix::new(m, n);
    for i in 0..m {
        for j in 0..n {
            for k in 0..min_mn {
                usv[i][j] += u[i][k] * s[k] * vt[k][j];
            }
        }
    }
    let usv_correct = "┌     ┐\n\
                       │ 2 4 │\n\
                       │ 1 3 │\n\
                       │ 0 0 │\n\
                       │ 0 0 │\n\
                       └     ┘";
    assert_eq!(format!("{}", usv), usv_correct);
    Ok(())
}
```

### Solve a linear system

```rust
use russell_lab::{solve_lin_sys, Matrix, Vector, StrError};

fn main() -> Result<(), StrError> {
    // set matrix and right-hand side
    let mut a = Matrix::from(&[
        [1.0,  3.0, -2.0],
        [3.0,  5.0,  6.0],
        [2.0,  4.0,  3.0],
    ]);
    let mut b = Vector::from(&[5.0, 7.0, 8.0]);

    // solve linear system b := a⁻¹⋅b
    solve_lin_sys(&mut b, &mut a)?;

    // check
    let x_correct = "┌         ┐\n\
                     │ -15.000 │\n\
                     │   8.000 │\n\
                     │   2.000 │\n\
                     └         ┘";
    assert_eq!(format!("{:.3}", b), x_correct);
    Ok(())
}
```

### Solve a sparse linear system

```rust
use russell_lab::{Matrix, Vector, StrError};
use russell_sparse::{ConfigSolver, Solver, SparseTriplet};

fn main() -> Result<(), StrError> {

    // allocate a square matrix
    let neq = 5; // number of equations
    let nnz = 13; // number of non-zeros
    let mut trip = SparseTriplet::new(neq, neq, nnz)?;
    trip.put(0, 0,  1.0)?; // << (0, 0, a00/2)
    trip.put(0, 0,  1.0)?; // << (0, 0, a00/2)
    trip.put(1, 0,  3.0)?;
    trip.put(0, 1,  3.0)?;
    trip.put(2, 1, -1.0)?;
    trip.put(4, 1,  4.0)?;
    trip.put(1, 2,  4.0)?;
    trip.put(2, 2, -3.0)?;
    trip.put(3, 2,  1.0)?;
    trip.put(4, 2,  2.0)?;
    trip.put(2, 3,  2.0)?;
    trip.put(1, 4,  6.0)?;
    trip.put(4, 4,  1.0)?;

    // print matrix
    let mut a = Matrix::new(neq, neq);
    trip.to_matrix(&mut a)?;
    let correct = "┌                ┐\n\
                   │  2  3  0  0  0 │\n\
                   │  3  0  4  0  6 │\n\
                   │  0 -1 -3  2  0 │\n\
                   │  0  0  1  0  0 │\n\
                   │  0  4  2  0  1 │\n\
                   └                ┘";
    assert_eq!(format!("{}", a), correct);

    // allocate x and rhs
    let mut x = Vector::new(neq);
    let rhs = Vector::from(&[8.0, 45.0, -3.0, 3.0, 19.0]);

    // initialize, factorize, and solve
    let config = ConfigSolver::new();
    let mut solver = Solver::new(config, neq, nnz, None)?;
    solver.factorize(&trip)?;
    solver.solve(&mut x, &rhs)?;
    let correct = "┌          ┐\n\
                   │ 1.000000 │\n\
                   │ 2.000000 │\n\
                   │ 3.000000 │\n\
                   │ 4.000000 │\n\
                   │ 5.000000 │\n\
                   └          ┘";
    assert_eq!(format!("{:.6}", x), correct);
    Ok(())
}
```

## Todo list

- [x] Add complex numbers functions to `russell_openblas`
- [ ] Add more complex numbers functions to `russell_lab`
- [ ] Add fundamental functions to `russell_lab`
    - [ ] Implement the modified Bessel functions
- [ ] Implement some numerical methods in `russell_lab`
    - [ ] Implement Brent's solver
    - [ ] Implement solver for the cubic equation
    - [ ] Implement numerical derivation
    - [ ] Implement numerical Jacobian function
    - [ ] Implement Newton's method for nonlinear systems
    - [ ] Implement numerical quadrature
- [ ] Add interpolation and polynomials to `russell_lab`
    - [ ] Implement Chebyshev interpolation and polynomials
    - [ ] Implement Orthogonal polynomials
    - [ ] Implement Lagrange interpolation
- [x] Add probability distribution functions to `russell_stat`
- [x] Finalize drawing of ASCII histogram in `russell_stat`
- [ ] Implement standard continuum mechanics tensors in `russell_tensor`
- [ ] Implement more integration tests for linear algebra
- [ ] Implement more examples
