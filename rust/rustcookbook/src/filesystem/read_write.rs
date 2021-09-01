use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use error_chain::{bail, error_chain};

error_chain! {
    foreign_links {
        Io(std::io::Error);
    }
    errors {
        Single {
            description("MyError!")
            display("Single Error")
        }
        Duple(t: String) {
            description("MyError!")
            display("Dutple {} Error", t)
        }
        Multi(len: u32, data: Vec<u32>) {
            description("MyError!")
            display("Multi len {} data {:?} Error", len, data)
        }
        Fuck {
            description("NMSL!")
            display("Single Error")
        }
    }
}

pub fn rw() -> Result<()> {
    use std::fs::File;
    use std::io::{BufRead, BufReader, Write};
    let path = "xxx.log";
    let mut f = File::create(path)?;
    write!(f, "Rust\n💖\nFun")?;

    let reader = BufReader::new(f);
    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}

pub fn is_same_test() -> Result<()> {
    use same_file::Handle;
    let path = "xxx.log";

    let stdout_handle = Handle::stdout()?;
    let handle = Handle::from_path(path)?;
    if stdout_handle == handle {
        // return Err(self::ErrorKind::Fuck.into())
        // return Err("sss".into())
        // bail!("sss")
        bail!(self::ErrorKind::Fuck)
    } else {
        let mut file = File::open(path)?;
        let reader = BufReader::new(file);
        for (num, line) in reader.lines().enumerate() {
            println!("{} : {}", num, line?.to_uppercase());
        }
    }
    Ok(())
}

pub fn unnormal_read() -> Result<()> {
    use memmap::Mmap;
    use std::io::Write;
    write!(
        File::create("memmapread.txt")?,
        "My hovercraft is full of eels!"
    )?;

    let f = File::open("memmapread.txt")?;
    let map = unsafe { Mmap::map(&f)? };
    let random_indexes = [0, 1, 2, 19, 22, 10, 11, 29];
    assert_eq!(&map[3..13], b"hovercraft");
    let random_chars: String = random_indexes.iter().map(|&x| map[x] as char).collect();
    println!("{}", random_chars);
    assert_eq!(&random_chars, "My loaf!");
    Ok(())
}

#[test]
pub fn test() {
    rw();
    is_same_test();
    unnormal_read();
}
