use crate::{erf, ProbabilityDistribution, StrError, SQRT_2, SQRT_PI};
use rand::Rng;
use rand_distr::{Distribution, LogNormal};

const LOGNORMAL_MIN_X: f64 = 1e-15;

/// Defines the Lognormal distribution
pub struct DistributionLognormal {
    mu_logx: f64,  // mean (μ) of log(x)
    sig_logx: f64, // standard deviation (σ) of log(x)
    a: f64,        // 1 / (sig_logx sqrt(2 π))
    b: f64,        // -1 / (2 sig_logx²)

    sampler: LogNormal<f64>, // sampler
}

impl DistributionLognormal {
    /// Creates a new Lognormal distribution
    ///
    /// # Input
    ///
    /// * `mu_logx` -- mean (μ) of log(x)
    /// * `sig_logx` -- standard deviation (σ) of log(x)
    pub fn new(mu_logx: f64, sig_logx: f64) -> Result<Self, StrError> {
        Ok(DistributionLognormal {
            mu_logx,
            sig_logx,
            a: 1.0 / (sig_logx * SQRT_2 * SQRT_PI),
            b: -1.0 / (2.0 * sig_logx * sig_logx),
            sampler: LogNormal::new(mu_logx, sig_logx).map_err(|_| "invalid parameters")?,
        })
    }

    /// Creates a new Lognormal distribution given mean and standard deviation parameters
    ///
    /// # Input
    ///
    /// * `mu` -- mean μ
    /// * `sig` -- standard deviation σ
    ///
    /// This function will hence calculate `mu_logx` (mean of log(x)) and `sig_logx` (std-dev of log(x))
    pub fn new_from_mu_sig(mu: f64, sig: f64) -> Result<Self, StrError> {
        let d = sig / mu;
        let v = f64::ln(1.0 + d * d);
        let sig_logx = f64::sqrt(v);
        let mu_logx = f64::ln(mu) - v / 2.0;
        Ok(DistributionLognormal {
            mu_logx,
            sig_logx,
            a: 1.0 / (sig_logx * SQRT_2 * SQRT_PI),
            b: -1.0 / (2.0 * sig_logx * sig_logx),
            sampler: LogNormal::new(mu_logx, sig_logx).map_err(|_| "invalid parameters")?,
        })
    }
}

impl ProbabilityDistribution for DistributionLognormal {
    /// Implements the Probability Density Function (CDF)
    fn pdf(&self, x: f64) -> f64 {
        if x < LOGNORMAL_MIN_X {
            return 0.0;
        }
        self.a * f64::exp(self.b * f64::powf(f64::ln(x) - self.mu_logx, 2.0)) / x
    }

    /// Implements the Cumulative Density Function (CDF)
    fn cdf(&self, x: f64) -> f64 {
        if x < LOGNORMAL_MIN_X {
            return 0.0;
        }
        (1.0 + erf((f64::ln(x) - self.mu_logx) / (self.sig_logx * SQRT_2))) / 2.0
    }

    /// Returns the Mean
    fn mean(&self) -> f64 {
        f64::exp(self.mu_logx + self.sig_logx * self.sig_logx / 2.0)
    }

    /// Returns the Variance
    fn variance(&self) -> f64 {
        let ss = self.sig_logx * self.sig_logx;
        (f64::exp(ss) - 1.0) * f64::exp(2.0 * self.mu_logx + ss)
    }

    /// Generates a pseudo-random number belonging to this probability distribution
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        self.sampler.sample(rng)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use crate::{DistributionLognormal, ProbabilityDistribution, StrError};
    use russell_chk::assert_approx_eq;

    // Data from the following R-code (run with Rscript lognormal.R):
    /*
    X <- seq(0, 3, 0.25)
    N <- c(0, 0.5, 1)    # mu_logx
    Z <- c(0.25, 0.5, 1) # sig_logx
    Y <- matrix(ncol=4)
    first <- TRUE
    for (n in N) {
        for (z in Z) {
            pdf <- dlnorm(X, n, z)
            cdf <- plnorm(X, n, z)
            for (i in 1:length(X)) {
                if (first) {
                    Y <- rbind(c(X[i], n, z, pdf[i], cdf[i]))
                    first <- FALSE
                } else {
                    Y <- rbind(Y, c(X[i], n, z, pdf[i], cdf[i]))
                }
            }
        }
    }
    write.table(format(Y, digits=15), "/tmp/lognormal.dat", row.names=FALSE, col.names=c("x","mu_logx","sig_logx","pdf","cdf"), quote=FALSE)
    print("file </tmp/lognormal.dat> written")
    */

    #[test]
    fn lognormal_works() -> Result<(), StrError> {
        #[rustfmt::skip]
        // x mu_logx sig_logx pdf cdf
        let data = [
            [0.00000000000000e+00, 0.00000000000000e+00, 2.50000000000000e-01, 0.00000000000000e+00, 0.00000000000000e+00],
            [2.50000000000000e-01, 0.00000000000000e+00, 2.50000000000000e-01, 1.34267554901169e-06, 1.46828105639037e-08],
            [5.00000000000000e-01, 0.00000000000000e+00, 2.50000000000000e-01, 6.83494950964322e-02, 2.78061786230952e-03],
            [7.50000000000000e-01, 0.00000000000000e+00, 2.50000000000000e-01, 1.09740697536329e+00, 1.24922017154603e-01],
            [1.00000000000000e+00, 0.00000000000000e+00, 2.50000000000000e-01, 1.59576912160573e+00, 5.00000000000000e-01],
            [1.25000000000000e+00, 0.00000000000000e+00, 2.50000000000000e-01, 8.57158800562558e-01, 8.13957381056098e-01],
            [1.50000000000000e+00, 0.00000000000000e+00, 2.50000000000000e-01, 2.85553775719259e-01, 9.47583382357198e-01],
            [1.75000000000000e+00, 0.00000000000000e+00, 2.50000000000000e-01, 7.44506640209658e-02, 9.87404566114261e-01],
            [2.00000000000000e+00, 0.00000000000000e+00, 2.50000000000000e-01, 1.70873737741080e-02, 9.97219382137690e-01],
            [2.25000000000000e+00, 0.00000000000000e+00, 2.50000000000000e-01, 3.68149296198414e-03, 9.99410103486230e-01],
            [2.50000000000000e+00, 0.00000000000000e+00, 2.50000000000000e-01, 7.72680788226295e-04, 9.99876409413847e-01],
            [2.75000000000000e+00, 0.00000000000000e+00, 2.50000000000000e-01, 1.61511159303679e-04, 9.99973994740764e-01],
            [3.00000000000000e+00, 0.00000000000000e+00, 2.50000000000000e-01, 3.40783542739241e-05, 9.99994447300413e-01],
            [0.00000000000000e+00, 0.00000000000000e+00, 5.00000000000000e-01, 0.00000000000000e+00, 0.00000000000000e+00],
            [2.50000000000000e-01, 0.00000000000000e+00, 5.00000000000000e-01, 6.83494950964322e-02, 2.78061786230952e-03],
            [5.00000000000000e-01, 0.00000000000000e+00, 5.00000000000000e-01, 6.10455304190183e-01, 8.28285190016985e-02],
            [7.50000000000000e-01, 0.00000000000000e+00, 5.00000000000000e-01, 9.01557703729440e-01, 2.82522523985944e-01],
            [1.00000000000000e+00, 0.00000000000000e+00, 5.00000000000000e-01, 7.97884560802865e-01, 5.00000000000000e-01],
            [1.25000000000000e+00, 0.00000000000000e+00, 5.00000000000000e-01, 5.77803754559390e-01, 6.72305064288457e-01],
            [1.50000000000000e+00, 0.00000000000000e+00, 5.00000000000000e-01, 3.82869771988593e-01, 7.91297126615529e-01],
            [1.75000000000000e+00, 0.00000000000000e+00, 5.00000000000000e-01, 2.43717185734742e-01, 8.68479321024816e-01],
            [2.00000000000000e+00, 0.00000000000000e+00, 5.00000000000000e-01, 1.52613826047546e-01, 9.17171480998301e-01],
            [2.25000000000000e+00, 0.00000000000000e+00, 5.00000000000000e-01, 9.51845919064197e-02, 9.47583382357198e-01],
            [2.50000000000000e+00, 0.00000000000000e+00, 5.00000000000000e-01, 5.95309165702857e-02, 9.66567581591083e-01],
            [2.75000000000000e+00, 0.00000000000000e+00, 5.00000000000000e-01, 3.74756056628331e-02, 9.78473828111750e-01],
            [3.00000000000000e+00, 0.00000000000000e+00, 5.00000000000000e-01, 2.37944822614366e-02, 9.85997794426055e-01],
            [0.00000000000000e+00, 0.00000000000000e+00, 1.00000000000000e+00, 0.00000000000000e+00, 0.00000000000000e+00],
            [2.50000000000000e-01, 0.00000000000000e+00, 1.00000000000000e+00, 6.10455304190183e-01, 8.28285190016985e-02],
            [5.00000000000000e-01, 0.00000000000000e+00, 1.00000000000000e+00, 6.27496077115924e-01, 2.44108595785583e-01],
            [7.50000000000000e-01, 0.00000000000000e+00, 1.00000000000000e+00, 5.10361006313849e-01, 3.86795057134097e-01],
            [1.00000000000000e+00, 0.00000000000000e+00, 1.00000000000000e+00, 3.98942280401433e-01, 5.00000000000000e-01],
            [1.25000000000000e+00, 0.00000000000000e+00, 1.00000000000000e+00, 3.11306099911394e-01, 5.88288108142545e-01],
            [1.50000000000000e+00, 0.00000000000000e+00, 1.00000000000000e+00, 2.44973651710510e-01, 6.57432169485154e-01],
            [1.75000000000000e+00, 0.00000000000000e+00, 1.00000000000000e+00, 1.94925228762767e-01, 7.12129233150075e-01],
            [2.00000000000000e+00, 0.00000000000000e+00, 1.00000000000000e+00, 1.56874019278981e-01, 7.55891404214417e-01],
            [2.25000000000000e+00, 0.00000000000000e+00, 1.00000000000000e+00, 1.27623257329531e-01, 7.91297126615529e-01],
            [2.50000000000000e+00, 0.00000000000000e+00, 1.00000000000000e+00, 1.04871066889650e-01, 8.20242786104215e-01],
            [2.75000000000000e+00, 0.00000000000000e+00, 1.00000000000000e+00, 8.69686432997110e-02, 8.44135545087431e-01],
            [3.00000000000000e+00, 0.00000000000000e+00, 1.00000000000000e+00, 7.27282561399947e-02, 8.64031392358576e-01],
            [0.00000000000000e+00, 5.00000000000000e-01, 2.50000000000000e-01, 0.00000000000000e+00, 0.00000000000000e+00],
            [2.50000000000000e-01, 5.00000000000000e-01, 2.50000000000000e-01, 2.77269555237383e-12, 2.25836044469474e-14],
            [5.00000000000000e-01, 5.00000000000000e-01, 2.50000000000000e-01, 3.61331963982621e-05, 9.09364730874628e-07],
            [7.50000000000000e-01, 5.00000000000000e-01, 2.50000000000000e-01, 1.48685582863174e-02, 8.14319615596294e-04],
            [1.00000000000000e+00, 5.00000000000000e-01, 2.50000000000000e-01, 2.15963866052752e-01, 2.27501319481792e-02],
            [1.25000000000000e+00, 5.00000000000000e-01, 2.50000000000000e-01, 6.91436702328259e-01, 1.34054939312812e-01],
            [1.50000000000000e+00, 5.00000000000000e-01, 2.50000000000000e-01, 9.90441925092644e-01, 3.52663458031298e-01],
            [1.75000000000000e+00, 5.00000000000000e-01, 2.50000000000000e-01, 8.86306636504786e-01, 5.94239054137485e-01],
            [2.00000000000000e+00, 5.00000000000000e-01, 2.50000000000000e-01, 5.92006289789127e-01, 7.80117089512224e-01],
            [2.25000000000000e+00, 5.00000000000000e-01, 2.50000000000000e-01, 3.27261680101392e-01, 8.93198842948483e-01],
            [2.50000000000000e+00, 5.00000000000000e-01, 2.50000000000000e-01, 1.59562642404320e-01, 9.52059872506144e-01],
            [2.75000000000000e+00, 5.00000000000000e-01, 2.50000000000000e-01, 7.14949095869118e-02, 9.79641663205033e-01],
            [3.00000000000000e+00, 5.00000000000000e-01, 2.50000000000000e-01, 3.02593564587463e-02, 9.91677324435874e-01],
            [0.00000000000000e+00, 5.00000000000000e-01, 5.00000000000000e-01, 0.00000000000000e+00, 0.00000000000000e+00],
            [2.50000000000000e-01, 5.00000000000000e-01, 5.00000000000000e-01, 2.59100402199152e-03, 8.07812434683331e-05],
            [5.00000000000000e-01, 5.00000000000000e-01, 5.00000000000000e-01, 9.25649645938870e-02, 8.50956125608893e-03],
            [7.50000000000000e-01, 5.00000000000000e-01, 5.00000000000000e-01, 3.07587593706764e-01, 5.75862078067135e-02],
            [1.00000000000000e+00, 5.00000000000000e-01, 5.00000000000000e-01, 4.83941449038287e-01, 1.58655253931457e-01],
            [1.25000000000000e+00, 5.00000000000000e-01, 5.00000000000000e-01, 5.47587019433349e-01, 2.89887673601522e-01],
            [1.50000000000000e+00, 5.00000000000000e-01, 5.00000000000000e-01, 5.22500074623600e-01, 4.25019061690486e-01],
            [1.75000000000000e+00, 5.00000000000000e-01, 5.00000000000000e-01, 4.52704707931436e-01, 5.47454054425741e-01],
            [2.00000000000000e+00, 5.00000000000000e-01, 5.00000000000000e-01, 3.70259858375548e-01, 6.50360661899309e-01],
            [2.25000000000000e+00, 5.00000000000000e-01, 5.00000000000000e-01, 2.92270139950110e-01, 7.32983177624653e-01],
            [2.50000000000000e+00, 5.00000000000000e-01, 5.00000000000000e-01, 2.25670788129207e-01, 7.97459591674771e-01],
            [2.75000000000000e+00, 5.00000000000000e-01, 5.00000000000000e-01, 1.71896410182678e-01, 8.46893784494822e-01],
            [3.00000000000000e+00, 5.00000000000000e-01, 5.00000000000000e-01, 1.29888747211947e-01, 8.84390483611032e-01],
            [0.00000000000000e+00, 5.00000000000000e-01, 1.00000000000000e+00, 0.00000000000000e+00, 0.00000000000000e+00],
            [2.50000000000000e-01, 5.00000000000000e-01, 1.00000000000000e+00, 2.69362457557087e-01, 2.96276493666574e-02],
            [5.00000000000000e-01, 5.00000000000000e-01, 1.00000000000000e+00, 3.91569816025209e-01, 1.16405868261998e-01],
            [7.50000000000000e-01, 5.00000000000000e-01, 1.00000000000000e+00, 3.90050919958956e-01, 2.15441347245306e-01],
            [1.00000000000000e+00, 5.00000000000000e-01, 1.00000000000000e+00, 3.52065326764300e-01, 3.08537538725987e-01],
            [1.25000000000000e+00, 5.00000000000000e-01, 1.00000000000000e+00, 3.07153753476980e-01, 3.90945167383459e-01],
            [1.50000000000000e+00, 5.00000000000000e-01, 1.00000000000000e+00, 2.64775742972154e-01, 4.62342133410526e-01],
            [1.75000000000000e+00, 5.00000000000000e-01, 1.00000000000000e+00, 2.27562274900022e-01, 5.23769178110117e-01],
            [2.00000000000000e+00, 5.00000000000000e-01, 1.00000000000000e+00, 1.95784908012604e-01, 5.76578148239245e-01],
            [2.25000000000000e+00, 5.00000000000000e-01, 1.00000000000000e+00, 1.68940693936602e-01, 6.22073163500329e-01],
            [2.50000000000000e+00, 5.00000000000000e-01, 1.00000000000000e+00, 1.46331855780068e-01, 6.61401363163834e-01],
            [2.75000000000000e+00, 5.00000000000000e-01, 1.00000000000000e+00, 1.27274743909652e-01, 6.95534826353484e-01],
            [3.00000000000000e+00, 5.00000000000000e-01, 1.00000000000000e+00, 1.11167283015234e-01, 7.25284270276039e-01],
            [0.00000000000000e+00, 1.00000000000000e+00, 2.50000000000000e-01, 0.00000000000000e+00, 0.00000000000000e+00],
            [2.50000000000000e-01, 1.00000000000000e+00, 2.50000000000000e-01, 1.04870989008785e-19, 6.79373979955605e-22],
            [5.00000000000000e-01, 1.00000000000000e+00, 2.50000000000000e-01, 3.49864215711348e-10, 6.32490898750125e-12],
            [7.50000000000000e-01, 1.00000000000000e+00, 2.50000000000000e-01, 3.68970865858203e-06, 1.29738437649915e-07],
            [1.00000000000000e+00, 1.00000000000000e+00, 2.50000000000000e-01, 5.35320903059541e-04, 3.16712418331199e-05],
            [1.25000000000000e+00, 1.00000000000000e+00, 2.50000000000000e-01, 1.02156414444636e-02, 9.43621791840118e-04],
            [1.50000000000000e+00, 1.00000000000000e+00, 2.50000000000000e-01, 6.29205045691068e-02, 8.70011928979628e-03],
            [1.75000000000000e+00, 1.00000000000000e+00, 2.50000000000000e-01, 1.93250888489884e-01, 3.90737909821670e-02],
            [2.00000000000000e+00, 1.00000000000000e+00, 2.50000000000000e-01, 3.75663841130233e-01, 1.09834024868100e-01],
            [2.25000000000000e+00, 1.00000000000000e+00, 2.50000000000000e-01, 5.32829681161895e-01, 2.24740927562080e-01],
            [2.50000000000000e+00, 1.00000000000000e+00, 2.50000000000000e-01, 6.03509898642491e-01, 3.68873996655332e-01],
            [2.75000000000000e+00, 1.00000000000000e+00, 2.50000000000000e-01, 5.79655259123555e-01, 5.18505735005762e-01],
            [3.00000000000000e+00, 1.00000000000000e+00, 2.50000000000000e-01, 4.92110729150050e-01, 6.53375270437936e-01],
            [0.00000000000000e+00, 1.00000000000000e+00, 5.00000000000000e-01, 0.00000000000000e+00, 0.00000000000000e+00],
            [2.50000000000000e-01, 1.00000000000000e+00, 5.00000000000000e-01, 3.61331963982621e-05, 9.09364730874628e-07],
            [5.00000000000000e-01, 1.00000000000000e+00, 5.00000000000000e-01, 5.16350884349193e-03, 3.54216744666189e-04],
            [7.50000000000000e-01, 1.00000000000000e+00, 5.00000000000000e-01, 3.86055388369465e-02, 5.00673010533327e-03],
            [1.00000000000000e+00, 1.00000000000000e+00, 5.00000000000000e-01, 1.07981933026376e-01, 2.27501319481792e-02],
            [1.25000000000000e+00, 1.00000000000000e+00, 5.00000000000000e-01, 1.90911217720882e-01, 6.01264576196979e-02],
            [1.50000000000000e+00, 1.00000000000000e+00, 5.00000000000000e-01, 2.62317431988743e-01, 1.17206103768475e-01],
            [1.75000000000000e+00, 1.00000000000000e+00, 5.00000000000000e-01, 3.09349476565862e-01, 1.89221586742651e-01],
            [2.00000000000000e+00, 1.00000000000000e+00, 5.00000000000000e-01, 3.30464565983484e-01, 2.69704930734910e-01],
            [2.25000000000000e+00, 1.00000000000000e+00, 5.00000000000000e-01, 3.30147308364215e-01, 3.52663458031298e-01],
            [2.50000000000000e+00, 1.00000000000000e+00, 5.00000000000000e-01, 3.14712244350577e-01, 4.33520370594661e-01],
            [2.75000000000000e+00, 1.00000000000000e+00, 5.00000000000000e-01, 2.90061756093321e-01, 5.09255357914229e-01],
            [3.00000000000000e+00, 1.00000000000000e+00, 5.00000000000000e-01, 2.60838872701926e-01, 5.78174100802873e-01],
            [0.00000000000000e+00, 1.00000000000000e+00, 1.00000000000000e+00, 0.00000000000000e+00, 0.00000000000000e+00],
            [2.50000000000000e-01, 1.00000000000000e+00, 1.00000000000000e+00, 9.25649645938870e-02, 8.50956125608893e-03],
            [5.00000000000000e-01, 1.00000000000000e+00, 1.00000000000000e+00, 1.90297804810106e-01, 4.52137277902241e-02],
            [7.50000000000000e-01, 1.00000000000000e+00, 1.00000000000000e+00, 2.32162198388357e-01, 9.89283283290916e-02],
            [1.00000000000000e+00, 1.00000000000000e+00, 1.00000000000000e+00, 2.41970724519143e-01, 1.58655253931457e-01],
            [1.25000000000000e+00, 1.00000000000000e+00, 1.00000000000000e+00, 2.36020867689781e-01, 2.18621734185903e-01],
            [1.50000000000000e+00, 1.00000000000000e+00, 1.00000000000000e+00, 2.22876045876283e-01, 2.76077206174196e-01],
            [1.75000000000000e+00, 1.00000000000000e+00, 1.00000000000000e+00, 2.06899223293204e-01, 3.29829428833277e-01],
            [2.00000000000000e+00, 1.00000000000000e+00, 1.00000000000000e+00, 1.90297804810106e-01, 3.79477701120085e-01],
            [2.25000000000000e+00, 1.00000000000000e+00, 1.00000000000000e+00, 1.74166691541200e-01, 4.25019061690486e-01],
            [2.50000000000000e+00, 1.00000000000000e+00, 1.00000000000000e+00, 1.59018793463368e-01, 4.66643794056493e-01],
            [2.75000000000000e+00, 1.00000000000000e+00, 1.00000000000000e+00, 1.45060158635938e-01, 5.04627990352803e-01],
            [3.00000000000000e+00, 1.00000000000000e+00, 1.00000000000000e+00, 1.32335751529021e-01, 5.39276943682299e-01],
        ];
        for row in data {
            let [x, mu_logx, sig_logx, pdf, cdf] = row;
            let d = DistributionLognormal::new(mu_logx, sig_logx)?;
            assert_approx_eq!(d.pdf(x), pdf, 1e-14);
            assert_approx_eq!(d.cdf(x), cdf, 1e-14);
        }
        Ok(())
    }

    #[test]
    fn new_from_mu_sig_works() -> Result<(), StrError> {
        let (mu, sig) = (1.0, 0.25);
        let d = DistributionLognormal::new_from_mu_sig(mu, sig)?;
        let ss = d.sig_logx * d.sig_logx;
        let mean = f64::exp(d.mu_logx + ss / 2.0);
        let var = (f64::exp(ss) - 1.0) * f64::exp(2.0 * d.mu_logx + ss);
        assert_approx_eq!(mean, mu, 1e-15);
        assert_approx_eq!(f64::sqrt(var), sig, 1e-15);
        Ok(())
    }

    #[test]
    fn mean_and_variance_work() -> Result<(), StrError> {
        let (mu, sig) = (1.0, 0.25);
        let d = DistributionLognormal::new_from_mu_sig(mu, sig)?;
        assert_approx_eq!(d.mean(), mu, 1e-14);
        assert_approx_eq!(d.variance(), sig * sig, 1e-14);
        Ok(())
    }
}