fn main() {
    create_dir_test();    
    regex_test();
    read_dir_test();
    comrak_test();
    mut_test();
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

fn read_dir_test() {
    use std::path::Path;
    use std::fs;
    use std::process;
    
    let src: &str = "release";
    let dir = Path::new(src);
    let entrys = match fs::read_dir(dir) {
        Ok(entrys) => entrys,
        Err(err) => { 
            println!("- {}: {}", err.to_string(), src);
            process::exit(1);
        }
    };
    for entry in entrys {
        if let Ok(entry) = entry {
            let child = entry.path();
            let file_name = child.to_str().unwrap();
            println!("- {}", file_name);
            let dirs: Vec<&str> = file_name.rsplitn(2, "\\").collect();
            for dir in dirs.iter() {
                println!("    - {}", dir);
            }
            println!("- {}", file_name);
            let dirs: Vec<&str> = file_name.splitn(2, "\\").collect();
            for dir in dirs.iter() {
                println!("    - {}", dir);
            }
        }
        
    }
    // struct Wrapper<T = String>(Vec<T>);
    // use std::ops::Deref;
    // impl<T> Deref for Wrapper<Vec<T>> {
    //     type Target = Vec<T>;
    //     fn deref(&self) -> &Vec<T> {
    //         &self
    //     }
    // }
    // use std::fmt;
    // impl<T> fmt::Display for Wrapper<Vec<T>> {
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //         write!(f, "[{}]", self.0.join(", "))
    //     }
    // }
}

fn comrak_test() {
    use comrak::{ComrakOptions, markdown_to_html};
    let md_str = r#"
    # sadwd##ssss##
    # [kkk](sss)
    # ![kkk](sss), sssss
    ```python
    print("Hello World!")
    ```
    "#;
    let html = markdown_to_html(md_str, &ComrakOptions::default());
    println!("{}", html);

}

fn mut_test() {
    
    struct A {
        val: i32,
        next: B,
    }
    struct B {
        val: i32,
        next: C,
    }
    struct C {
        val: i32,
    }
    let c = C { val: 3 };
    let mut a = A { val: 1, next: B { val: 2, next: c}};
    a.next.next.val = 1;
}