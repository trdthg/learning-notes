use std::convert::TryInto;
use std::fs::File;
use std::io::{BufReader, Read, Write};

pub fn read_file_offset(name: &str, offset: isize) -> char {
    let mut s = String::new();
    let mut reader = BufReader::new(File::open(name).unwrap());
    reader.read_to_string(&mut s).unwrap();

    let ptr: *mut u8 = s.as_mut_ptr();
    unsafe { *ptr.offset(offset) as char }
}

pub fn read_file_offset_until(name: &str, offset: isize) -> char {
    let mut s = String::new();
    let mut reader = BufReader::new(File::open(name).unwrap());
    reader.read_to_string(&mut s).unwrap();

    let ptr: *mut u8 = s.as_mut_ptr();
    unsafe { *ptr.offset(offset) as char }
}

pub fn append_file(name: &str, str: &str) {
    let mut s = String::new();
    let mut f = File::open(name).unwrap();
    let mut reader = BufReader::new(f);
    reader.read_to_string(&mut s).unwrap();

    // f.write(buf)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{convert::TryInto, mem::size_of};

    #[test]
    fn a() {
        let s: &str = "123";
        let ptr: *const u8 = s.as_ptr();

        unsafe {
            println!("{}", *ptr.offset(1) as char);
            println!("{}", *ptr.offset(2) as char);
        }

        use std::io::{BufReader, Read};
        let mut s = String::new();
        let mut reader = BufReader::new(File::open("hello.c").unwrap());
        use std::fs::File;
        reader.read_to_string(&mut s).unwrap();
        let ptr: *mut u8 = s.as_mut_ptr();
        unsafe {
            *ptr.offset(10) = b'@';
            let len: isize = s.len().try_into().unwrap();
            let off = len + 100;
            for i in off..off + 1000 {
                print!("{}", *ptr.offset(i) as char);
            }
        }
    }

    #[test]
    fn b() {
        let c = read_file_offset("hello.c", 10);
        println!("{}", c);
    }
}
