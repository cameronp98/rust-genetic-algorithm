/// A genetic algorithm in Rust
/// Copyright (C) 2015  Andrew Schwartzmeyer

use Problem;
use rand::distributions::Range;
use std::f64::consts;

impl Problem {
    /// Fitness function for the evolutionary computation benchmark
    /// problems evaluated on the hybercube of dimension p, where
    /// f(x*) = 0.
    ///
    /// # [Problems](https://www.cs.cmu.edu/afs/cs/project/jair/pub/volume24/ortizboyer05a-html/node6.html)
    ///
    /// * Ackley: 20+e-20*exp(-0.2*sqrt((1/p)*sum(x_i^2)))-exp((1/p)*sum(cos(2*pi*x_i))), x*=(0, ...)
    /// * Griewangk: 1+sum(x_i^2/4000)-prod(cos(x_i/sqrt(i))), x*=(0, ...)
    /// * Schwefel: 418.9829*p+sum(x_i*sin(sqrt(abs(x_i)))), x*=(420.9687, ...)
    pub fn fitness(&self, x: &[f64]) -> f64 {
        let p = x.len() as f64;
        match *self {
            Problem::Ackley => {
                20_f64 + consts::E - 20_f64 *
                    (-0.2_f64 * (p.recip() * (x.iter().fold(0_f64, |sum, x| {
                        sum + x.powi(2)
                    })).sqrt())).exp() -
                    (p.recip() * x.iter().fold(0_f64, |sum, x| {
                        sum + (2_f64 * consts::PI * x).cos()
                    })).exp()
            }
            Problem::Griewangk => {
                1_f64 + x.iter().fold(0_f64, |sum, x| {
                    sum + x.powi(2)/4000_f64
                }) - x.iter().enumerate().fold(1_f64, |prod, (i, x)| {
                    prod * (x/((i + 1) as f64).sqrt()).cos()
                })
            }
            Problem::Schwefel => {
                418.9829_f64 * p + x.iter().fold(0_f64, |sum, i| {
                    sum + i * i.abs().sqrt().sin()
                })
            }
        }
    }

    /// Domain for the given problem.
    pub fn domain(&self) -> Range<f64> {
        match *self {
            Problem::Ackley => Range::new(-30_f64, 30_f64),
            Problem::Schwefel => Range::new(-512.03_f64, 511.97_f64),
            Problem::Griewangk => Range::new(-600_f64, 600_f64),
        }
    }
}
