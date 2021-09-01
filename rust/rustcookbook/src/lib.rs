pub mod algorithms;
pub mod cmd;
pub mod compressing;
pub mod concurrency;
pub mod cryptography;
pub mod database;
pub mod datastructures;
pub mod datetime;
pub mod debugging;
pub mod encoding;
pub mod filesystem;
pub mod operatingsystem;
pub mod webprogramming;

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn test() {}
}

// 遍历文件夹
use std::path::Path;
use std::path::PathBuf;
pub fn walk_dir(path: PathBuf) {
    // let path: PathBuf = Path::new(".").to_path_buf();
    let entrys = path.read_dir().unwrap();
    for entry in entrys {
        let path = entry.unwrap().path();
        if path.is_dir() {
            walk_dir(path)
        } else {
            println!("----------{}", path.display());
        }
    }
}

// 生成随机数列表
pub fn get_random_vec(num1: usize, num2: usize) -> Vec<Vec<i32>> {
    use rand::distributions::{Distribution, Uniform};
    use std::time::{Duration, SystemTime};
    let mut rng = rand::thread_rng();
    let uniform = Uniform::from(0..100);

    let start = SystemTime::now();
    let vecs: Vec<Vec<i32>> = (0..num1)
        .map(|_| {
            let vec_int: Vec<i32> = uniform.sample_iter(&mut rng).take(num2).collect();
            vec_int
        })
        .collect();
    println!(
        "create random data takes: {:?}",
        SystemTime::now().duration_since(start)
    );
    vecs
}
