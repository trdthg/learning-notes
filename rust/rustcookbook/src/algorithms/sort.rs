use rand::Rng;
use std::time::{Duration, SystemTime};

pub fn sort_int() {
    let mut vec = vec![1, 5, 10, 2, 15];
    vec.sort();
    vec.sort_unstable();
    assert_eq!(vec, vec![1, 2, 5, 10, 15]);

    fn get_random_vec() -> Vec<Vec<i32>> {
        use rand::distributions::{Distribution, Uniform};
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
    let vecs = get_random_vec();
    let start = SystemTime::now();
    for mut vec in vecs {
        vec.sort();
    }
    println!("sort takes: {:?}", SystemTime::now().duration_since(start));

    let vecs = get_random_vec();
    let start = SystemTime::now();
    for mut vec in vecs {
        vec.sort_unstable();
    }
    println!(
        "sort_unstable takes: {:?}",
        SystemTime::now().duration_since(start)
    );
}

pub fn sort_float() {
    let mut vec = vec![1.1, 1.15, 5.5, 1.123, 2.0];

    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
}

pub fn sort_struct() {
    // 为了使 Person 可自然排序，你需要四个 traits：Eq、PartialEq、Ord，以及 PartialOrd。
    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
    struct Person {
        name: String,
        age: u32,
    }
    impl Person {
        pub fn new(name: &str, age: u32) -> Self {
            Person {
                name: name.to_string(),
                age,
            }
        }
    }
    let mut persons = vec![
        Person::new("aaa", 9),
        Person::new("bbb", 18),
        Person::new("ccc", 37),
    ];
    persons.sort();
    println!("persion: {:?}", persons);
    let mut persons = vec![
        Person::new("aaa", 9),
        Person::new("bbb", 18),
        Person::new("ccc", 37),
    ];
    persons.sort_by(|a, b| b.age.partial_cmp(&a.age).unwrap());
    println!("persion: {:?}", persons);
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn sort() {
        sort_int();
        sort_float();
        sort_struct();
    }
}
