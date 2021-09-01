use error_chain::error_chain;

use crate::concurrency::explicit_threads::walk_dir;

error_chain! {

    foreign_links {
        IO(std::io::Error);
        WalkDir(walkdir::Error);
        SystemTime(std::time::SystemTimeError);
        Glob(glob::GlobError);
        Pattern(glob::PatternError);
    }
}

pub fn find_readed_in_24hours() -> Result<()> {
    use walkdir::WalkDir;

    use std::fs::Metadata;

    let current_dir = std::env::current_dir()?;
    println!(
        "Entries modified in the last 24 hours in {:?}:",
        current_dir
    );

    for entry in WalkDir::new(current_dir) {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            continue;
        }
        let metadata: Metadata = path.metadata()?;
        let last_modified = metadata.modified()?.elapsed()?.as_secs();
        if last_modified > 24 * 60 * 60 {
            println!(
                "Last modified: {:?} hours, is read only: {:?}, size: {:?} bytes, filename: {:?}",
                last_modified as f32 / 3600 as f32,
                metadata.permissions().readonly(),
                metadata.len() / 1000,
                path.file_name().ok_or("No filename")?
            );
        }
    }

    Ok(())
}

pub fn contains_loop() -> Result<()> {
    use std::path::{Path, PathBuf};

    fn is_contains_loop<P: AsRef<Path>>(path: P) -> Result<Option<(PathBuf, PathBuf)>> {
        let path = path.as_ref();
        let mut path_buf = path.to_path_buf();
        println!("{:?}", path);
        while path_buf.pop() {
            println!("{:?}", path_buf);
            if same_file::is_same_file(path_buf.as_path(), path)? {
                return Ok(Some((path.to_path_buf(), path_buf)));
            } else if let Some(looped_paths) = is_contains_loop(&path_buf)? {
                return Ok(Some(looped_paths));
            }
        }
        return Ok(None);
    }

    if let Some((path, path_buf)) = is_contains_loop("/tmp/foo/bar/baz/qux/bar/baz")? {
        println!("{:?} == {:?}", path, path_buf);
    }

    Ok(())
}

pub fn contains_repeat() -> Result<()> {
    use std::collections::HashMap;
    use std::path::Path;

    use walkdir::WalkDir;

    fn is_contains_repeat<P: AsRef<Path>>(path: P) -> Result<()> {
        let mut filenames = HashMap::new();

        for entry in WalkDir::new(path)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|e| !e.file_type().is_dir())
        {
            let f_name = entry.file_name().to_string_lossy().to_string();
            let counter = filenames.entry(f_name.clone()).or_insert(0);
            *counter += 1;

            if *counter == 2 {
                println!("{}", f_name);
            }
        }
        let mut sorted_hashmap = filenames.into_iter().collect::<Vec<(String, i32)>>();
        // println!("{:?}", sorted_hashmap);
        sorted_hashmap.sort_by(|a, b| a.1.cmp(&b.1));
        println!("{:?}", sorted_hashmap);
        Ok(())
    }

    is_contains_repeat(std::env::current_dir()?)?;

    Ok(())
}

pub fn skip_hiddens() -> Result<()> {
    use walkdir::{DirEntry, WalkDir};

    fn is_not_hidden(entry: &DirEntry) -> bool {
        entry.file_name().to_string_lossy().starts_with('.')
    }

    for entry in WalkDir::new(".")
        .min_depth(1) // 确定递归深度
        .max_depth(3)
        .follow_links(true) // 确保软连接也会被查询
        .into_iter()
        .filter_entry(|e| is_not_hidden(e)) // filter_entry 判断是否为隐藏
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        let sec = entry.metadata()?.modified()?;

        if f_name.ends_with(".json") && sec.elapsed()?.as_secs() < 86400 {
            println!("{}", f_name);
        }
    }

    Ok(())
}

pub fn find_all_png() -> Result<()> {
    for entry in glob::glob("**/*.png")? {
        println!("{}", entry?.display());
    }

    let option = glob::MatchOptions {
        case_sensitive: false, // 大小写不敏感
        ..Default::default()   // 其他默认
    };

    // glob_with能自定义参数
    for entry in glob::glob_with("**/OUT.*.png", option)? {
        println!("{}", entry?.display());
    }

    for entry in walkdir::WalkDir::new(".") {
        let entry = entry?;
        if entry.file_name().to_string_lossy().ends_with(".png") {
            println!("{}", entry.path().display())
        }
    }
    Ok(())
}

#[test]
pub fn test() {
    // find_readed_in_24hours();
    // contains_repeat();
    // if let Err(errors) = contains_loop() {
    //     errors.iter()
    //         .enumerate()
    //         .for_each(|(index, error)| println!("└> {} - {}", index, error));
    // }
    find_all_png();
}
