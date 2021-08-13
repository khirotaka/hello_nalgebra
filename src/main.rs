extern crate nalgebra as na;

use rand;
use rand_distr::Normal;
use std::ops::Mul;

fn main() {
    let norm: Normal<f32> = Normal::new(0.0, 1.0).unwrap();
    let seed: [u8; 32] = [0; 32];       // 0 がseed値

    let mut rng: rand::rngs::StdRng = rand::SeedableRng::from_seed(seed);
    let a: na::DMatrix<f32> = na::DMatrix::<f32>::from_distribution(5, 5, &norm, &mut rng);
    println!("{}", a);
    println!("{:?}", a.shape());

    println!("NumPyでいう a.sum(), a.sum(1), a.sum(2)");
    println!("{}", &a.row_sum());
    println!("{}", &a.column_sum());
    println!("{}", &a.sum());

    let b: na::DVector<f32> = na::DVector::from_distribution(5, &norm, &mut rng);
    println!("{}", &b.transpose().mul(&a));
}
