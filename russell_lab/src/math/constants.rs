/// π <https://oeis.org/A000796>
pub const PI: f64 =
    3.14159265358979323846264338327950288419716939937510582097494459230781640628620899862803482534211706798214;

/// sqrt(π) <https://oeis.org/A002161>
pub const SQRT_PI: f64 =
    1.772453850905516027298167483341145182797549456122387128213807789852911284591032181374950656738544665_f64;

/// Euler constant <https://oeis.org/A001620>
pub const EULER: f64 =
    0.577215664901532860606512090082402431042159335939923598805767234884867726777664670936947063291746749_f64;

/// sqrt(2) <https://oeis.org/A002193>
pub const SQRT_2: f64 =
    1.41421356237309504880168872420969807856967187537694807317667973799073247846210703885038753432764157f64;

/// sqrt(3) <https://oeis.org/A002194>
pub const SQRT_3: f64 = 1.7320508075688772935274463415058723669428052538103806280558069794519330169088000370811461867572485756756261414154f64;

/// sqrt(6) <https://oeis.org/A010464>
pub const SQRT_6: f64 =
    2.44948974278317809819728407470589139196594748065667012843269256725096037745731502653985943310464023f64;

/// sqrt(2/3) <https://oeis.org/A157697>
pub const SQRT_2_BY_3: f64 =
    0.816496580927726032732428024901963797321982493552223376144230855750320125819105008846619811034880078272864f64;

/// sqt(3/2) <https://oeis.org/A115754>
pub const SQRT_3_BY_2: f64 =
    1.22474487139158904909864203735294569598297374032833506421634628362548018872865751326992971655232011f64;

/// 1/3
pub const ONE_BY_3: f64 =
    0.33333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333f64;

/// 2/3
pub const TWO_BY_3: f64 =
    0.66666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666f64;

/// 1/sqrt(2) = cos(pi/4) <https://oeis.org/A010503>
pub const ONE_BY_SQRT_2: f64 =
    0.707106781186547524400844362104849039284835937688474036588339868995366239231053519425193767163820786367506f64;

/// cos(π/8) = cos(22.5°) <https://oeis.org/A144981>
pub const COS_PI_BY_8: f64 =
    0.923879532511286756128183189396788286822416625863642486115097731280535007501102358714839934850344596097963f64;

/// sin(π/8) = sin(22.5°) <https://oeis.org/A182168>
pub const SIN_PI_BY_8: f64 =
    0.382683432365089771728459984030398866761344562485627041433800635627546033960089692237013785342283547148424f64;

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::{
        COS_PI_BY_8, ONE_BY_3, ONE_BY_SQRT_2, PI, SIN_PI_BY_8, SQRT_2, SQRT_2_BY_3, SQRT_3, SQRT_3_BY_2, SQRT_6,
        SQRT_PI, TWO_BY_3,
    };
    use crate::approx_eq;

    #[test]
    fn constants_are_correct() {
        assert_eq!(PI, std::f64::consts::PI);
        approx_eq(SQRT_PI, f64::sqrt(PI), 1e-15);
        assert_eq!(SQRT_2, std::f64::consts::SQRT_2);
        assert_eq!(SQRT_3, f64::sqrt(3.0));
        assert_eq!(SQRT_6, f64::sqrt(6.0));
        assert_eq!(SQRT_2_BY_3, f64::sqrt(2.0 / 3.0));
        assert_eq!(SQRT_3_BY_2, f64::sqrt(3.0 / 2.0));
        assert_eq!(ONE_BY_3, 1.0 / 3.0);
        assert_eq!(TWO_BY_3, 2.0 / 3.0);
        assert_eq!(ONE_BY_SQRT_2, std::f64::consts::FRAC_1_SQRT_2);
        assert_eq!(COS_PI_BY_8, f64::cos(std::f64::consts::PI / 8.0));
        assert_eq!(SIN_PI_BY_8, f64::sin(std::f64::consts::PI / 8.0));
    }
}
