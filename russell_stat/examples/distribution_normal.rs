use russell_stat::*;

fn main() -> Result<(), StrError> {
    // generate samples
    let mut rng = get_rng();
    let dist = DistributionNormal::new(0.0, 1.0)?;
    let nsamples = 10_000;
    let mut data = vec![0.0; nsamples];
    for i in 0..nsamples {
        data[i] = dist.sample(&mut rng);
    }
    println!("{}", statistics(&data));

    // text-plot
    let stations = (0..20).map(|i| -4.0 + (i as f64) * 0.5).collect::<Vec<f64>>();
    let mut hist = Histogram::new(&stations)?;
    hist.set_bar_char('🍕').set_bar_max_len(30);
    hist.count(&data);
    println!("{:.2}", hist);
    Ok(())
}

/* Sample output

min = -3.466424128646902
max = 3.608012748101761
mean = 0.003299589990111208
std_dev = 0.9760553437435371

[-4.00,-3.50) |    0
[-3.50,-3.00) |   10
[-3.00,-2.50) |   41
[-2.50,-2.00) |  156 🍕🍕
[-2.00,-1.50) |  443 🍕🍕🍕🍕🍕🍕
[-1.50,-1.00) |  869 🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕
[-1.00,-0.50) | 1450 🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕
[-0.50, 0.00) | 1971 🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕
[ 0.00, 0.50) | 1996 🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕
[ 0.50, 1.00) | 1499 🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕
[ 1.00, 1.50) |  951 🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕🍕
[ 1.50, 2.00) |  419 🍕🍕🍕🍕🍕🍕
[ 2.00, 2.50) |  150 🍕🍕
[ 2.50, 3.00) |   34
[ 3.00, 3.50) |   10
[ 3.50, 4.00) |    1
[ 4.00, 4.50) |    0
[ 4.50, 5.00) |    0
[ 5.00, 5.50) |    0
         sum = 10000
*/
