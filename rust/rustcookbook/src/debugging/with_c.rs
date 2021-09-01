use error_chain::error_chain;
use std::ffi::CString;
use std::os::raw::c_char;

error_chain! {
    foreign_links {
        NulError(::std::ffi::NulError);
        Io(::std::io::Error);
    }
}

extern "C" {
    fn hello();
    fn greet(name: *const c_char);
}

fn prompt(s: &str) -> Result<String> {
    use std::io::Write;
    print!("{}", s);
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub fn buffer_flush() {
    use std::fs::File;
    use std::io::{BufWriter, Write};
    // This method will continuously call [`write`] until there is no more data to be written
    let mut buffer = BufWriter::new(File::create("xxx.log").unwrap());

    buffer.write_all(b"some bytes").unwrap();
    println!("{:?}", buffer);
    // 清空一个buffer
    buffer.flush().unwrap();
    println!("{:?}", buffer);
}

pub fn try_extern_c() -> Result<()> {
    println!("Hello, world!");
    unsafe {
        hello();
    }
    let name = prompt("What's your name? ")?;
    let c_name = CString::new(name)?;
    unsafe {
        greet(c_name.as_ptr());
        greet(CString::new("sss".to_string())?.as_ptr())
    }
    Ok(())
}

#[test]
pub fn test() {
    buffer_flush();
    try_extern_c();
}
