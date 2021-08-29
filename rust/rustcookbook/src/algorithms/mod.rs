mod random;
mod sort;

pub fn get_random_vec() -> Vec<Vec<i32>> {
    use rand::distributions::{Distribution, Uniform};
    use std::time::{Duration, SystemTime};
    let mut rng = rand::thread_rng();
    let uniform = Uniform::from(0..100);

    let start = SystemTime::now();
    let vecs: Vec<Vec<i32>> = (0..100)
        .map(|_| {
            let vec_int: Vec<i32> = uniform.sample_iter(&mut rng).take(1000).collect();
            vec_int
        })
        .collect();
    println!(
        "create random data takes: {:?}",
        SystemTime::now().duration_since(start)
    );
    vecs
}
