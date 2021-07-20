fn main() {
    // create_dir_test();    
    regex_test();
}

fn create_dir_test() {
    use std::fs;
    use std::path::Path;
    static BASE_DIR: &str = "release";
    fn create_dir(dir: &str) {
        if !Path::new(dir).exists() {
            match fs::create_dir_all(dir) {
                Ok(_) => println!("create {}", dir),
                Err(err) => panic!("create {}: {}", dir, err)
            }
        }
    }
    create_dir(&format!("{}/{}", BASE_DIR, "new_dir"));
    create_dir(&format!("{}/{}", BASE_DIR, "another_dir"));
}

fn regex_test() {
    extern crate regex;
    use regex::Regex;

    let re_mod = Regex::new(r"\{\{\s*project\s*\}\}").unwrap();
    let text = r#"
        {{ project }}
        {{project}}
        {{ project}}
        {{project }}
    "#;
    let replaced_text = re_mod.replace_all(text, "kk");
    println!("{}", replaced_text);
}
