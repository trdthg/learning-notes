use ansi_term::Colour;

use std::thread;
use std::time::{Duration, SystemTime};

pub fn find_max(arr: &[i32]) -> Option<i32> {
    // 生成短期线程
    const THRESHOLD: usize = 8;

    if arr.len() <= THRESHOLD {
        return arr.iter().cloned().max();
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at(mid);

    crossbeam::scope(|s| {
        let thread_l = s.spawn(|_| find_max(left));
        let thread_r = s.spawn(|_| find_max(right));

        let max_l = thread_l.join().unwrap().unwrap();
        let max_r = thread_r.join().unwrap()?;

        Some(max_l.max(max_r))
    })
    .unwrap()
}

pub fn find_max_std(arr: &'static [i32]) -> Option<i32> {
    const THRESHOLD: usize = 8;
    if arr.len() <= THRESHOLD {
        return arr.iter().cloned().max();
    }
    let mid = arr.len() / 2;
    let (left, right) = arr.split_at(mid);
    let thread_l = std::thread::spawn(move || find_max_std(left));
    let thread_r = std::thread::spawn(move || find_max_std(right));
    let max_l: i32 = thread_l.join().unwrap().unwrap();
    let max_r: i32 = thread_r.join().unwrap().unwrap();
    Some(max_l.max(max_r))
}

pub fn find_max_test() {
    println!(
        "{}",
        Colour::Blue.paint("-------------------- concurrency 1 --------------------")
    );
    let mut arr = get_random_vec(1, 1000);
    let arr = arr.get_mut(0).unwrap();
    let max = find_max(&arr);

    static mut arr_static: &[i32] = &[1, 25, -4, 10];
    let start = SystemTime::now();
    unsafe {
        let max = find_max_std(arr_static);
    }
    println!(
        "find_max takes: {:?}",
        SystemTime::now().duration_since(start)
    );
}

pub fn concurrency_channel() {
    use crossbeam_channel::bounded;

    let (snd1, rcv1) = bounded::<i32>(1);
    let (snd2, rcv2) = bounded(1);

    let n_msgs = 4;
    let n_workers = 2;

    crossbeam::scope(|s| {
        // __________   _____________
        //|snd1>rcv1|-> |snd2.clone()=rcv  >

        // Source(管道源)开始发送msg
        s.spawn(|_| {
            for i in 0..n_msgs {
                snd1.send(i).unwrap();
                println!("Source sent {}", i);
            }
            // 关闭信道 —— 这是退出的必要条件
            // for 巡海在工作线程中
            drop(snd1);
        });

        // 管道的接收端(rcv1)对接这工人的发送端(snd2.clone()), 共有两个工人进行msg传递
        for _ in 0..n_workers {
            let (sendr, recvr) = (snd2.clone(), rcv1.clone()); // !!!
            s.spawn(move |_| {
                thread::sleep(Duration::from_millis(500));
                // 工人尝试接收从源发送的msg
                for msg in recvr.iter() {
                    println!("worker {:?} received {}. ", thread::current().id(), msg);
                    // 接收后就发送出去
                    sendr.send(msg * 2).unwrap();
                }
            });
        }
        drop(snd2);

        // 尝试接收最终数据
        for msg in rcv2.iter() {
            println!("Sink received {}", msg);
        }
    })
    .unwrap();
}

pub fn concurrency_channel_std() {
    use std::sync::mpsc;

    let n_workers = 2;
    let n_msgs = 4;

    let (sender, receiver) = mpsc::channel::<i32>();

    use std::sync::{Arc, Mutex};
    let receiver = Arc::new(Mutex::new(receiver));
    let (sender2, receiver2) = mpsc::channel::<i32>();
    let sender_c = sender.clone();
    std::thread::spawn(move || {
        for i in 0..n_msgs {
            sender_c.send(i).unwrap();
            println!("source send {}", i);
        }
    });
    let mut threads = Vec::new();
    for i in 0..n_workers {
        let (sender_copy, receiver_copy) = (sender2.clone(), receiver.clone());
        let thread_tmp = std::thread::spawn(move || {
            // 不断尝试接受msg, 需要有一个停止信号, 否则就不会终止循环
            loop {
                let msg = receiver_copy.lock().unwrap().try_recv();
                if let Ok(msg) = msg {
                    if msg == 0 {
                        break;
                    }
                    thread::sleep(Duration::from_millis(500));
                    sender_copy.send(msg * 2).unwrap();
                    println!("worker {:?} received {}", thread::current().id(), msg);
                }
            }
        });
        // drop(sender_copy);
        threads.push(thread_tmp);
    }

    let sender_q = sender.clone();
    std::thread::spawn(move || {
        let stdin = std::io::stdin();
        let mut s = String::new();
        stdin.read_line(&mut s).unwrap();
        if s == "quit" {
            for i in 0..n_workers {
                sender_q.send(0).unwrap();
                println!("trying to stop worker {}", i);
            }
        }
        sender2.send(0).unwrap();
    });
    loop {
        if let Ok(msg) = receiver2.try_recv() {
            if msg == 0 {
                break;
            }
            println!("Sink msg {}", msg);
        }
    }
}

pub fn spsc() {
    use crossbeam_channel::unbounded;

    let (snd, rcv) = unbounded();
    let n_msgs = 5;
    crossbeam::scope(|s| {
        s.spawn(|_| {
            for i in 0..n_msgs {
                snd.send(i).unwrap();
            }
        });
    })
    .unwrap();
    drop(snd);
    for msg in rcv.iter() {
        println!("{}", msg);
    }
}

pub fn spsc_std() {
    use std::sync::mpsc;
    let (snd, rcv) = mpsc::channel();
    let n_msgs = 5;
    std::thread::spawn(move || {
        (0..n_msgs).for_each(|i| {
            snd.send(i).unwrap();
        })
    });
    for msg in rcv.iter() {
        println!("{}", msg);
    }
    // loop {
    //     if let Ok(msg) = rcv.try_recv() {
    //         println!("{}", msg);
    //     }
    // }
    // rcv.
}

pub fn keep_static_globally() {
    use lazy_static::lazy_static;
    use std::sync::Mutex;
    lazy_static! {
        static ref FRUIT: Mutex<Vec<String>> = Mutex::new(Vec::new());
    }
    fn insert(fruit: &str) {
        if let Ok(mut db) = FRUIT.try_lock() {
            db.push(fruit.to_owned());
            thread::sleep(Duration::from_millis(500));
        }
    }
    insert("apple");
    insert("banana");
    insert("peach");
    {
        let db = FRUIT
            .lock()
            .map_err(|_| "Failed to acquire MutexGuard")
            .unwrap();
        db.iter()
            .enumerate()
            .for_each(|(i, fruit)| println!("{}: {}", i, fruit));
    }
    insert("grape");
}

use std::path::Path;
use std::path::PathBuf;
pub fn walk_dir(path: PathBuf) {
    let entrys = path.read_dir().unwrap();
    for entry in entrys {
        let path = entry.unwrap().path();
        if path.is_dir() {
            walk_dir(path)
        } else {
            println!("----------{}", path.display());
            // get_SHA256(path);
        }
    }
}

// use ring:;
pub fn get_SHA256<P: AsRef<Path>>(file_path: P) {
    use std::fs::File;
    use std::io::BufReader;
    use std::io::Read;

    use ring::digest::{Context, Digest, SHA256};

    let file = File::open(file_path).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut buffer = [0; 1024];
    let mut context = Context::new(&SHA256);
    loop {
        // 毒气到的字节数
        let count = buf_reader.read(&mut buffer).unwrap();
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }
    let sha: Digest = context.finish();
    println!("{:?}", sha);
}

pub fn is_iso(path: &Path) -> bool {
    match path.extension() {
        Some(e) => {
            if e.to_string_lossy().to_lowercase() == "log" {
                true
            } else {
                false
            }
        }
        _ => false,
    }
}

pub fn compute_digest(path: &Path) {
    use ring::digest::{Context, Digest, SHA256};
    use threadpool::ThreadPool;
    use walkdir::WalkDir;

    let pool = threadpool::ThreadPool::new(num_cpus::get());
    let (tx, rx) = std::sync::mpsc::channel();

    for entry in WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.path().is_dir() && is_iso(e.path()))
    {
        let path = entry.path().to_owned();
        let tx = tx.clone();
        pool.execute(move || {
            let digest = get_SHA256(path.clone());
            tx.send((path, digest)).unwrap();
        });
    }
    drop(tx);
    for (path, digest) in rx.iter() {
        println!("{:?} {:?}", path.display(), digest);
    }
}

pub fn draw_image() {
    use image::{ImageBuffer, Pixel, Rgb};
    use num::complex::Complex;

    // 将强度值转换为 RGB 值的函数
    // 基于 http://www.efg2.com/Lab/ScienceAndEngineering/Spectra.htm
    fn wavelength_to_rgb(wavelength: u32) -> Rgb<u8> {
        let wave = wavelength as f32;

        let (r, g, b) = match wavelength {
            380..=439 => ((440. - wave) / (440. - 380.), 0.0, 1.0),
            440..=489 => (0.0, (wave - 440.) / (490. - 440.), 1.0),
            490..=509 => (0.0, 1.0, (510. - wave) / (510. - 490.)),
            510..=579 => ((wave - 510.) / (580. - 510.), 1.0, 0.0),
            580..=644 => (1.0, (645. - wave) / (645. - 580.), 0.0),
            645..=780 => (1.0, 0.0, 0.0),
            _ => (0.0, 0.0, 0.0),
        };

        let factor = match wavelength {
            380..=419 => 0.3 + 0.7 * (wave - 380.) / (420. - 380.),
            701..=780 => 0.3 + 0.7 * (780. - wave) / (780. - 700.),
            _ => 1.0,
        };

        let (r, g, b) = (
            normalize(r, factor),
            normalize(g, factor),
            normalize(b, factor),
        );
        Rgb::from_channels(r, g, b, 0)
    }

    // 将茱莉亚集距离映射为强度值
    fn julia(c: Complex<f32>, x: u32, y: u32, width: u32, height: u32, max_iter: u32) -> u32 {
        let width = width as f32;
        let height = height as f32;

        let mut z = Complex {
            // scale and translate the point to image coordinates
            re: 3.0 * (x as f32 - 0.5 * width) / width,
            im: 2.0 * (y as f32 - 0.5 * height) / height,
        };

        let mut i = 0;
        for t in 0..max_iter {
            if z.norm() >= 2.0 {
                break;
            }
            z = z * z + c;
            i = t;
        }
        i
    }

    // 规格 RGB 颜色值范围内的强度值
    fn normalize(color: f32, factor: f32) -> u8 {
        ((color * factor).powf(0.8) * 255.) as u8
    }

    let (width, height) = (1920, 1080);
    let mut img = ImageBuffer::new(width, height);

    let pool = threadpool::ThreadPool::new(num_cpus::get());
    let (tx, rx) = std::sync::mpsc::channel();
    for y in 0..height {
        let tx = tx.clone();
        pool.execute(move || {
            for x in 0..width {
                // 生成像素
                let iterations = 300;
                let c = Complex::new(-0.8, 0.156);
                let i = julia(c, x, y, width, height, iterations);
                let pixel = wavelength_to_rgb(380 + i * 400 / iterations);
                tx.send((x, y, pixel)).expect("Could not send data!");
            }
        })
    }
    for _ in 0..(width * height) {
        let (x, y, pixel) = rx.recv().unwrap();
        img.put_pixel(x, y, pixel);
    }
    let _ = img.save("output.png").unwrap();
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn test() {
        // find_max_test();
        // concurrency_channel();
        // concurrency_channel_std();
        spsc();
        spsc_std();
        // keep_static_globally();

        // walk_dir(Path::new("./sss").to_path_buf());
        // get_SHA256(&Path::new("sadofsbidskdajfdsbhfkjdskla.log"));
        // compute_digest(&Path::new("./sss"));

        // draw_image();
    }
}

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
