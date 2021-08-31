use std::os::raw::c_char;

use rustcookbook::database::sqlite::insert_db;
use rustcookbook::{debugging::log::output_log};
use rustcookbook::*;

// extern {
//     fn hello();
//     fn greet(name: * const std::os::raw::c_char);
// }


// use error_chain::error_chain;
// use std::ffi::CString;

// error_chain! {
//     foreign_links {
//         NulError(::std::ffi::NulError);
//         Io(::std::io::Error);
//     }
// }
// fn prompt(s: &str) -> Result<String> {
//     use std::io::Write;
//     print!("{}", s);
//     std::io::stdout().flush()?;
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input)?;
//     Ok(input.trim().to_string())
// }


fn main() -> Result<()> {
    println!("Hello, world!");
    // let name = prompt("What's your name? ")?;
    // let c_name = CString::new(name).unwrap();
    // unsafe { 
    //     hello();
    //     greet(c_name.as_ptr()) 
    // }
    // Ok(())
    
}