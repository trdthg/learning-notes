
use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;


pub fn create_dirs(dir: &str) {
    if !Path::new(dir).exists() {
        match fs::create_dir_all(dir) {
            Err(why) => panic!("create {}: {}", dir, why.to_string()),
            Ok(_) => println!("create {}", dir),
        };
    }
}

pub fn create_file(path: &str) {
    let path = Path::new(&path);
    let mut file = match File::create(path) {
        Err(why) => panic!("create {}: {}", path.display(), why.to_string()),
        Ok(file) => file,
    };
    // match file_index_md.write_all(content.as_bytes()) {
    //     Err(why) => panic!("write {}: {}", fpath.display(), why.to_string()),
    //     Ok(_) => println!("write {}", path.display()),
    // };
}

pub fn write_file(path: &str, content: &str) {
    let mut f = File::create(path).unwrap();
    f.write_all(content.as_bytes());
}