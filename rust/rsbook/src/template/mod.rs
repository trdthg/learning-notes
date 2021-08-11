extern crate regex;

use std::path::Path;

use crate::util::file::*;
use crate::Config;

use std::fs;
use std::fs::File;
use std::io::prelude::*;

use comrak::{markdown_to_html, ComrakOptions};
use regex::Regex;

pub fn create_default_index(docs: &str, dist: &str, config: Config) {
    let content = &fs::read_to_string("assets/index.html").unwrap();

    // title
    let re_module = Regex::new(r"\{\{\s*title\s*\}\}").unwrap();
    let content = &String::from(re_module.replace_all(content, &config.title));
    // content
    let re_module = Regex::new(r"\{\{\s*page_content\s*\}\}").unwrap();
    let md_content = fs::read_to_string(format!("{}/README.md", docs)).unwrap();
    let md_html = markdown_to_html(&md_content, &ComrakOptions::default());
    let md_content = &String::from(re_module.replace_all(content, md_html));

    let default_config_file = &format!("{}/{}", dist, "index.html");
    let config_file = Path::new(default_config_file);
    write_file(config_file.to_str().unwrap(), md_content);

}

pub fn create_github_min_css(dist: &str) {
    let content = &fs::read_to_string("assets/css/github.min.css").unwrap();
    let default_config_file = &format!("{}/{}", dist, "assets/css/github.min.css");
    let config_file = Path::new(default_config_file);
    write_file(config_file.to_str().unwrap(), content);
}

pub fn create_index_css(dist: &str) {
    let content = &fs::read_to_string("assets/css/index.css").unwrap();
    let default_config_file = &format!("{}/{}", dist, "assets/css/index.css");
    let config_file = Path::new(default_config_file);
    write_file(config_file.to_str().unwrap(), content);
}

pub fn creage_highlight_min_js(dist: &str) {
    let content = &fs::read_to_string("assets/js/highlight.min.js").unwrap_or("sss".to_string());
    let default_config_file = &format!("{}/{}", dist, "assets/js/highlight.min.js");
    let config_file = Path::new(default_config_file);
    write_file(config_file.to_str().unwrap(), content);
}


pub fn create_default_config(project_name: &str) {
    // create default config file
    let content = &fs::read_to_string("assets/config.json").unwrap();
    let default_config_file = &format!("{}/{}", project_name, "docs/.rsbook/config.json");
    let config_file = Path::new(default_config_file);
    write_file(config_file.to_str().unwrap(), content);
}

pub fn create_hello_vue(project_name: &str) {
    let content = r#"
    <template>
    <div>
        <h1 class="h1">Hello World</h1>
    </div>
    </template>

    <style scoped>
    h1 {
        color: rgb(209, 88, 88);
    }
    </style>

    <script>
    export default {
        
    }
    </script>
    "#;
    let default_config_file = &format!("{}/{}", project_name, "docs/.rsbook/components/hello.vue");
    let config_file = Path::new(default_config_file);
    write_file(config_file.to_str().unwrap(), content);
}

pub fn create_README(project_name: &str) {
    let content = &fs::read_to_string("assets/python.md").unwrap();
    // let md_html = markdown_to_html(&md_content, &ComrakOptions::default());
    let default_config_file = &format!("{}/{}", project_name, "docs/README.md");
    let config_file = Path::new(default_config_file);
    write_file(config_file.to_str().unwrap(), content);
} 