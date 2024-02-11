use rand_distr::StandardNormal;
use rand::prelude::*;
use std::ops::AddAssign;

#[derive(Clone)]
struct Kahan {
    sum: f64,
    compensation: f64,
}

impl Kahan {

    fn new() -> Self {
        Self { sum: 0.0, compensation: 0.0 }
    }
    
    fn add(&mut self, x: f64) {
        let y = x - self.compensation;
        let t = self.sum + y;
        self.compensation = (t - self.sum) - y;
        self.sum = t;
    }
}

impl AddAssign for Kahan {
    fn add_assign(&mut self, rhs: Kahan) {
        self.add(rhs.sum);
    }
}

impl From<f64> for Kahan {
    fn from(x: f64) -> Self {
        Self { sum: x, compensation: 0.0 }
    }
}

impl From<Kahan> for f64 {
    fn from(x: Kahan) -> Self {
        x.sum
    }
}

struct Welford {
    n: u32,
    mean: Kahan,
    m2: Kahan,
}

impl Welford {

    fn new() -> Self {
        Welford {
            n: 0,
            mean: Kahan::new(),
            m2: Kahan::new(),
        }
    }

    fn add(&mut self, x: f64) {
        self.n += 1;
        let delta = x - self.mean.sum;
        self.mean += (delta / self.n as f64).into();
        let delta2 = x - self.mean.sum;
        self.m2 += (delta * delta2 as f64).into();
    }

    fn variance(&self) -> f64 {
        self.m2.sum / self.n as f64
    }
    
    fn average(&self) -> f64 {
        self.mean.clone().into()
    }

}

fn main() {
  let mut rng = thread_rng();
  let normal = StandardNormal;
  
  let mut stats = Welford::new();

  const N: u32 = 10000;
  for _ in 0..N {
    let x = normal.sample(&mut rng);
    stats.add(x);
  }

  let mean = stats.average();
  let variance = stats.variance();
  let stdev = variance.sqrt();

  println!("Mean: {}", mean);
  println!("Standard deviation: {}", stdev);
}
