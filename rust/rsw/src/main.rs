use std::fs;

#[macro_use]
extern crate clap;

use clap::App;

use rsw::util::*;

static PUBLIC_DIR: &str = "public";
static SRC_DIR: &str = "src";
static BUILD_DIR: &str = "build";

fn main() {
    // 相对于当前文件找到YAML文件，类似于找到模块的方式
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    if let Some(matches) = matches.subcommand_matches("new") {
        let project_name = matches.value_of("PROJECT").unwrap();
        init_work_space(project_name, PUBLIC_DIR, SRC_DIR)
    }
    
    if let Some(_) = matches.subcommand_matches("clean") {
        fs::remove_dir_all(BUILD_DIR).unwrap();
    }

    if let Some(_) = matches.subcommand_matches("build") {
    
    }
      
}