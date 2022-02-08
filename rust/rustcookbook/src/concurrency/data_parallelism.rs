use rayon::prelude::*;
use std::thread;
use std::time::Duration;

pub fn have_a_try() {
    let mut arr = [0, 7, 2, 1];
    arr.par_iter_mut().for_each(|n| {
        println!("1");
        thread::sleep(Duration::from_millis(5000));
        *n += 1
    });
    println!("{:?}", arr);

    assert!(arr.par_iter().all(|n| *n <= 8));
    assert!(arr.par_iter().all(|&n| n <= 8));
    let res: Option<&i32> = arr.par_iter().find_any(|n| **n < 2);
    let res: Option<i32> = arr.par_iter().find_any(|&&n| n < 2).map(|x| *x);
}

pub fn sort_parallely() {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    let mut vec: Vec<String> = vec![String::new(); 100_000];
    vec.par_iter_mut().for_each(|p| {
        let mut rng = thread_rng();
        *p = (0..5).map(|_| rng.sample(&Alphanumeric) as char).collect();
        println!("{}", p);
    });
    vec.par_sort_unstable();
}

pub fn map_reduce() {
    struct Person {
        age: i32,
    }
    let v: Vec<Person> = vec![
        Person { age: 17 },
        Person { age: 11 },
        Person { age: 34 },
        Person { age: 11 },
        Person { age: 45 },
        Person { age: 12 },
        Person { age: 26 },
        Person { age: 22 },
    ];
    // 总人数
    let num_over_30 = v.par_iter().filter(|&x| x.age > 30).count() as f32;
    // 总年龄
    let sum_over_30 = v.par_iter().map(|x| x.age).filter(|&age| age > 30).reduce(
        || 0,
        |x, y| {
            println!("x: {} y: {}", x, y);
            x + y
        },
    );
    let alt_sum_30: i32 = v.par_iter().map(|x| x.age).filter(|&x| x > 30).sum();
    let avg_over_30 = sum_over_30 as f32 / num_over_30;
    let alt_avg_over_30 = alt_sum_30 as f32 / num_over_30;
    println!(
        "{} ------------------------------------- {}",
        sum_over_30, alt_avg_over_30
    );
    println!("{}", std::f32::EPSILON);
}

use error_chain::error_chain;

error_chain! {
    foreign_links {
        Image(image::ImageError);
        Io(std::io::Error);
        Glob(glob::PatternError);
    }
}

use std::path::Path;
pub fn make_thumbnail<PA, PB>(original: PA, thumb_dir: PB, longest_edge: u32) -> Result<()>
where
    PA: AsRef<Path>,
    PB: AsRef<Path>,
{
    use image::imageops::FilterType;
    let img = image::open(original.as_ref())?;
    let file_path = thumb_dir.as_ref().join(original);
    let thumbnail = img.resize(longest_edge, longest_edge, FilterType::Nearest);
    thumbnail.save(file_path)?;
    Ok(())
}
pub fn make_thumbnail_test() -> Result<()> {
    use glob::MatchOptions;
    // find all the jpg file undr the given dirlet options: MatchOptions = Default::default();
    let options: MatchOptions = Default::default();
    let files: Vec<_> = glob::glob_with("*png", options)?
        .filter_map(|x| x.ok())
        .collect();
    if files.len() == 0 {
        error_chain::bail!("No .jpg files found in current directory");
    }

    // set and create the thumbnails' dir
    let thumb_dir = "./thumbnails";
    std::fs::create_dir_all(thumb_dir)?;

    // 为每个图片创建略缩图, 错误处理并打印
    // let image_failures = files
    //     .par_iter()
    //     .map(|path| {
    //         make_thumbnail(path, thumb_dir, 300)
    //             .map_err(|e| e.chain_err(|| path.display().to_string()))
    //     })
    //     .filter_map(|x| x.err())
    //     .collect::<Vec<_>>();
    // image_failures
    //     .iter()
    //     .for_each(|err| println!("err: {}", err));

    files
        .par_iter()
        .map(|path| {
            make_thumbnail(path, thumb_dir, 300)
                .map_err(|e| e.chain_err(|| path.display().to_string()))
        })
        .filter_map(|x| x.err())
        .collect::<Vec<_>>()
        .iter()
        .for_each(|err| println!("err: {}", err));
    Ok(())
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn test() {
        have_a_try();
        // sort_parallely();
        // map_reduce();
        // make_thumbnail_test();
    }
}
