use clap::{App, Arg, SubCommand};

pub fn clap_() {
    let matches = App::new("my test program")
        .author("trdthg")
        .version("0.1.0")
        .about("teach arguements parse")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("A cool file"),
        )
        .arg(
            Arg::with_name("num")
                .short("n")
                .long("number")
                .takes_value(true),
        )
        .subcommands(vec![
            SubCommand::with_name("sub3").arg(Arg::with_name("arg")),
            SubCommand::with_name("sub2").args(&[
                Arg::from_usage("[debug] -d 'turns on debugging info'"),
                Arg::with_name("input")
                    .index(1)
                    .help("the input file to use"),
            ]),
        ])
        .get_matches();
    let file_name = matches.value_of("file").unwrap_or("no file");
    match matches.subcommand_name() {
        Some("sub1") => {}
        Some("sub2") => {}
        Some("sub3") => {}
        _ => {}
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn test() {
        // clap_();
    }
}
