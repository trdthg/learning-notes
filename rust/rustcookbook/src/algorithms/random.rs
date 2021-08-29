use rand::Rng;
use std::collections::HashMap;

pub fn generate_random() {
    let mut rng = rand::thread_rng();
    let n1: u8 = rng.gen();
    let n2 = rng.gen::<u16>();
    println!("u8: {}", n1);
    println!("f16: {}", n2);
    println!("f64: {}", rng.gen::<f64>());
}

pub fn generate_random_with_range() {
    let mut rng = rand::thread_rng();
    let _a: f64 = rng.gen_range(0.0..10.0);
    println!("f64: {}", rng.gen_range(0.0..10.0));
    println!("i32: {}", rng.gen_range(1..2));
}

pub fn uniform_range() {
    use rand::distributions::{Distribution, Uniform};
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..6);

    let mut map = HashMap::new();
    for _ in 1..10000 {
        let throw = die.sample(&mut rng);
        if let Some(x) = map.get_mut(&throw) {
            *x += 1;
        } else {
            map.insert(throw, 1);
        }
    }
    println!("map_uniform_i32: {:?}", map);
    let rand_vec: Vec<i32> = die.sample_iter(&mut rng).take(30).collect();
    println!("rand_vec: {:?}", rand_vec);
}

use rand_distr::NormalError;
pub fn special_range() -> Result<(), NormalError> {
    use rand_distr::{Distribution, Normal};
    use std::collections::BTreeMap;
    let mut rng = rand::thread_rng();
    // let die = Normal::from(1..6);
    let normal = Normal::new(2.0, 3.0)?;

    let mut map = BTreeMap::new();
    for _ in 1..10000 {
        let throw = normal.sample(&mut rng);
        let throw = throw as i32;
        if let Some(x) = map.get_mut(&throw) {
            *x += 1;
        } else {
            map.insert(throw, 1);
        }
    }

    println!("map_normal_f64: {:?}", map.values());
    Ok(())
}

pub fn genreate_for_struct() {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    use rand::distributions::{Distribution, Standard};
    impl Distribution<Point> for Standard {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
            let (rand_x, rand_y) = rng.gen();
            Point {
                x: rand_x,
                y: rand_y,
            }
        }
    }
    let mut rng = rand::thread_rng();
    let rand_tuple = rng.gen::<(i32, bool, f64)>();
    let rand_point: Point = rng.gen();
    println!("rand_tuple: {:?}\nrand_point: {:?}", rand_tuple, rand_point);
}

pub fn generate_random_string() {
    use rand::distributions::Alphanumeric;

    let rng = rand::thread_rng();
    let rand_string: String = rng
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    println!("rand_string: {}", rand_string);
}

pub fn generate_random_string_from_string() {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 30;
    let mut rng = rand::thread_rng();
    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let rand_index = rng.gen_range(0..CHARSET.len());
            CHARSET[rand_index] as char
        })
        .collect();
    println!("password: {}", password);
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn rand() {
        generate_random();
        generate_random_with_range();
        uniform_range();
        special_range();
        genreate_for_struct();
        generate_random_string();
        generate_random_string_from_string();
    }

    #[test]
    fn sort() {}
}
