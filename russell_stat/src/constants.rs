/// Euler constant <https://oeis.org/A001620>
pub const EULER:f64 = 0.577215664901532860606512090082402431042159335939923598805767234884867726777664670936947063291746749_f64;

/// π <https://oeis.org/A000796>
pub const PI: f64 = 3.14159265358979323846264338327950288419716939937510582097494459230781640628620899862803482534211706798214_f64;

/// sqrt(π) <https://oeis.org/A002161>
pub const SQRT_PI:f64 = 1.772453850905516027298167483341145182797549456122387128213807789852911284591032181374950656738544665_f64;

/// sqrt(2) <https://oeis.org/A002193>
pub const SQRT_2: f64 = 1.41421356237309504880168872420969807856967187537694807317667973799073247846210703885038753432764157_f64;

/// sqrt(6) <https://oeis.org/A010464>
pub const SQRT_6: f64 = 2.44948974278317809819728407470589139196594748065667012843269256725096037745731502653985943310464023_f64;

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::{PI, SQRT_2, SQRT_6, SQRT_PI};
    use russell_chk::assert_approx_eq;

    #[test]
    fn constants_are_correct() {
        assert_eq!(PI, std::f64::consts::PI);
        assert_approx_eq!(SQRT_PI, f64::sqrt(PI), 1e-15);
        assert_eq!(SQRT_2, 2_f64.sqrt());
        assert_eq!(SQRT_6, 6_f64.sqrt());
    }
}
