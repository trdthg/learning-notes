extern crate json;
extern crate comrak;

pub mod path;
pub mod file;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

pub use file::*;
pub use path::*;

use comrak::{markdown_to_html, ComrakOptions};


pub fn bookbuild(project_name: &str) {

    let mut config_file = File::open(format!("{}/{}/{}/{}", project_name, "docs", ".rsbook", "config.json")).unwrap();
    let mut content = String::new();
    config_file.read_to_string(&mut content).unwrap();
    let config = json::parse(&content).unwrap();
    println!("{}", config["head"]["icon"]["href"]);

    bookbuild_newdir(".", project_name);

    let mut md_file = File::open(format!("{}/{}/{}", project_name, "docs", "README.md")).unwrap();
    let mut md_content = String::new();
    md_file.read_to_string(&mut md_content).unwrap();
    let md_html = markdown_to_html(&md_content, &ComrakOptions::default());

    let indexhtml_file = &format!("{}/{}/{}", project_name, "docs/.rsbook/dist", "index.html");
    write_file(indexhtml_file, &md_html);




}

pub fn bookbuild_newdir(base_dir: &str, project_name: &str) {
    let dirs = vec![
        "assets/", 
        "assets/js/", 
        "assets/css", 
        "assets/img", 
    ];
    let files = vec![
        "index.html",
    ];
    for path in dirs {
        let temp: &str = &format!("{}/{}/{}/{}", base_dir, project_name, "docs/.rsbook/dist", path);
        let temp = Path::new(temp);
        create_dirs(temp.to_str().unwrap());
    }
    for path in files {
        let temp: &str = &format!("{}/{}/{}/{}", base_dir, project_name, "docs/.rsbook/dist", path);
        let temp = Path::new(temp);
        create_file(temp.to_str().unwrap());
    }
}

pub fn booknew(base_dir: &str, project_name: &str) {
    // 1. create dist menu
    let dirs = vec![
        "docs/.rsbook/", 
        "docs/.rsbook/components/", 
        "docs/.rsbook/public/", 
        "docs/.rsbook/dist/", 
        "docs/.rsbook/styles/", 
    ];
    let files = vec![
        "docs/.rsbook/config.json",
        "docs/README.md",
        "deploy.sh",
        "deploy.cmd",
    ];
    for path in dirs {
        let temp: &str = &format!("{}/{}/{}", base_dir, project_name, path);
        let temp = Path::new(temp);
        create_dirs(temp.to_str().unwrap());
    }
    for path in files {
        let temp: &str = &format!("{}/{}/{}", base_dir, project_name, path);
        let temp = Path::new(temp);
        create_file(temp.to_str().unwrap());
    }
    // create default config file
    let content = r#"
    {
        "head": { 
            "title": "Trdthg\"s blog",
              "icon": {"href": "logo.png" }
        },
        "markdown": {
              "lineNumbers": false 
        },
        "themeConfig": {
            "lastUpdated": "Last Updated",
            "sidebarDepth": 3,
            "sidebar": {
                "/java/": ["java", "sourceread", "spring", "springboot", "stuffs"],
                "/js/": ["js", "vue"],
                "/python/": ["python"],
                "/rust/": ["rust", "lists", "wasm"],
                "/other/": ["other", "script", "datastructure"],
                "/": [""]
            }
        },
        "nav": [
            {"text": "Java",  "link": "/java/java"},
            {"text": "前端", "link": "/js/js" },
            {"text": "Python", "link": "/python/python"},
            {"text": "Rust", "link": "/rust/rust" },
            {"text": "其他", "link": "/other/other"},
            {"text": "Github", "link": "https://github.com/trdthg"}
        ]
    }
    "#;
    let default_config_file = &format!("{}/{}", project_name, "docs/.rsbook/config.json");
    let config_file = Path::new(default_config_file);
    write_file(config_file.to_str().unwrap(), content);
}

pub fn bookclean(project_name: &str) {

}
