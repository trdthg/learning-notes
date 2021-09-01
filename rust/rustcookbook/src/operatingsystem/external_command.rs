use error_chain::{bail, error_chain};

error_chain! {
    foreign_links {
        IO(std::io::Error);
        String(std::string::FromUtf8Error);
        Regex(regex::Error);
        Env(std::env::VarError);
    }
}

pub fn run_cmd() -> Result<()> {
    use std::process::Command;

    use regex::Regex;

    #[derive(PartialEq, Default, Clone, Debug)]
    struct Commit {
        hash: String,
        message: String,
    }

    let output = Command::new("git").arg("log").arg("--oneline").output()?;
    if !output.status.success() {
        bail!("command executed with failing code error");
    }

    let pattern = Regex::new(
        r"(?x)
                               ([0-9a-fA-F]+) # commit hash
                               (.*)           # The commit message",
    )?;
    String::from_utf8(output.stdout)?
        .lines()
        .filter_map(|line| pattern.captures(line))
        .map(|cap| Commit {
            hash: cap[1].to_string(),
            message: cap[2].trim().to_string(),
        })
        .take(5)
        .for_each(|x| println!("{:?}", x));

    Ok(())
}

fn run_python() -> Result<()> {
    use std::collections::HashSet;
    use std::io::Write;
    use std::process::{Command, Stdio};

    let mut child = Command::new("python")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    child
        .stdin
        .as_mut()
        .ok_or("open stdout failed!")?
        .write_all(b"import this; copyright(); credits(); exit()")?;

    let output = child.wait_with_output()?;
    if output.status.success() {
        let raw_output = String::from_utf8(output.stdout)?;
        let words = raw_output
            .split_whitespace()
            .map(|s| s.to_lowercase())
            .collect::<HashSet<_>>();
        println!("Found {} unique words:", words.len());
        println!("{:#?}", words);
    }

    Ok(())
}

pub fn like_grep() -> Result<()> {
    use std::process::{Command, Stdio};

    let current_dir = std::env::current_dir()?;
    let mut command1 = Command::new("du")
        .arg("-ah")
        .arg(&current_dir)
        .stdout(Stdio::piped())
        .spawn()?;

    if let Some(output1) = command1.stdout.take() {
        let mut command2 = Command::new("sort")
            .arg("-hr")
            .stdout(Stdio::piped())
            .stdin(output1)
            .spawn()?;

        command1.wait()?;

        if let Some(output2) = command2.stdout.take() {
            let command3 = Command::new("head")
                .args(&["-n", "10"])
                .stdin(output2)
                .stdout(Stdio::piped())
                .spawn()?;

            let output3 = command3.wait_with_output()?;
            command2.wait();

            println!(
                "Top 10 biggest files and directories in '{}':\n{}",
                current_dir.display(),
                String::from_utf8(output3.stdout).unwrap()
            );
        }
    }

    Ok(())
}

pub fn eprint_to_file() -> Result<()> {
    use std::process::{Command, Stdio};
    let outputs = std::fs::File::create("xxx.log")?;
    let errors = outputs.try_clone()?;

    Command::new("ls")
        .args(&[".", "oops"])
        .stdout(Stdio::from(outputs))
        .stderr(Stdio::from(errors))
        .spawn()?
        .wait_with_output()?;

    Ok(())
}

pub fn read_env() -> Result<()> {
    // 从环境变量 `CONFIG` 读取配置路径 `config_path`。
    // 如果 `CONFIG` 未设置，采用默认配置路径。
    let config_path = std::env::var("CONFIG.env".to_string()).unwrap_or("CONFIG.env".to_string());
    let config: String = std::fs::read_to_string(config_path)?;
    println!(
        "----------------------------------------------------Config: {}",
        config
    );

    Ok(())
}

#[test]
fn test() {
    // run_cmd();

    // if let Err(errors) = run_python() {
    //     errors
    //         .iter()
    //         .enumerate()
    //         .for_each(|(index, error)| println!("└> {} - {}", index, error));
    // }

    // like_grep();
    read_env();
    eprint_to_file();
}
