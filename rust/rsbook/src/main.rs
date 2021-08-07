#[macro_use]
extern crate clap;

use std::fs;
use std::path::Path;
use std::process;

use clap::App;
use regex::Regex;
use chrono::Utc;

use rsbook::util;

fn main() {
    let args = std::env::args();
    println!("{:?}", args);
    let mut project_name;
    let mut base_dir;
    let start_time = Utc::now().timestamp_millis() as f64;
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    if let Some(matches) = matches.subcommand_matches("new") {
        // 得到文件base路径
        project_name = matches.value_of("PROJECT_NAME").unwrap();
        base_dir = matches.value_of("base_dir").unwrap_or(".");
        // 创建基本文件结构
        println!("creating new project {} in {} ... ", project_name, base_dir);
        util::booknew(base_dir, project_name);
    }

    if let Some (_) = matches.subcommand_matches("build"){
        // build呗
        let project_name = "tmp";
        println!("building project {} ... ", project_name);
        util::bookbuild(project_name);
    }

    if let Some(_) = matches.subcommand_matches("clean") {
        let project_name = "tmp";
        println!("building project {} ... ", project_name);
        util::bookclean(project_name);
    }
    let end_time = Utc::now().timestamp_millis() as f64;
    println!("{}", "BUILD SUCCESS");
    println!("Total time: {} s", (end_time - start_time)/1000.00);
    println!("Finished at: {}", Utc::now().to_string())
}
