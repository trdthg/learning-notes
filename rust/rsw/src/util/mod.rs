extern crate regex;

use std::path::Path;
use std::process;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

static MD_STR: &str = r#"
---
title: {{ project }} 
author: RustWriter
template: index
---

# {{ project }}
This is written in rust writer. Simple, free and happy.
"#;

static HTML_STR: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta http-equiv="X-UA-Compatible" content="ie=edge">
    <meta name="author" content="{{ author }}" />
    <title>{{ title }}</title>
</head>
<body>
    {{ content }}
</body>
</html>
"#;

pub fn create_not_exists(dir: &str) {
    if !Path::new(dir).exists() {
        match fs::create_dir_all(dir) {
            Ok(_) => println!("create {}", dir),
            Err(err) => panic!("create {}: {}", dir, err)
        }
    }
}

pub fn write_file(file_name: &str, content: &str) {
    let index_md = Path::new(&file_name);
    let mut file_index_md = match File::create(index_md) {
        Ok(file) => file,
        Err(err) => panic!("create {}: {}", file_name, err),
    };
    match file_index_md.write_all(content.as_bytes()) {
        Ok(_) => println!("write {}", file_name),
        Err(err) => panic!("write {}: {}", file_name, err),
    };

}

pub fn init_work_space(project_name: &str, public_dir: &str, src_dir: &str) {
    // 新建文件夹
    let path = Path::new(project_name);
    if path.exists() {
        println!("{} has exists", project_name);
        process::exit(0x0100);
    }
    let project_src = format!("{}/{}", project_name, src_dir);
    let project_public = format!("{}/{}", project_name, public_dir);
    create_not_exists(&project_src);
    create_not_exists(&project_public);

    // 创建index.md
    let index_md_name = format!("{}/{}", &project_src, "index.md");
    let re_project = Regex::new(r"\{\{\s*project\s*\}\}").unwrap();
    let md_text = String::from(re_project.replace_all(MD_STR, project_name));
    write_file(&index_md_name, &md_text);

    // 创建__index.html
    let index_tpl_name = format!("{}/{}", &project_public, "__index.html");
    write_file(&index_tpl_name, HTML_STR);

    // 创建rsw.toml
    write_file(&format!("{}/{}", project_name, "rsw.toml"), &format!("site_name = \"{}\"\nsite_url = \"http://localhost\"", project_name));
    println!("{} created successfully", project_name);
} 