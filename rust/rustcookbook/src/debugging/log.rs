use log::Level;

pub fn output_log() {
    // env_logger::init();
    log::debug!("{}", "this is a log msg only shown on debug mode");
    eprintln!("this is a log from std lib");
    log::error!("{}", "this is a log msg on staout");

    use env_logger::{Builder, Target};
    Builder::new().target(Target::Stdout).init();
    log::debug!("ssssssss");
}

pub fn customed_logger() {
    use log::{Metadata, Record};

    struct ConsoleLogger;
    impl log::Log for ConsoleLogger {
        // Determines if a log message with the specified metadata would be logged.
        fn enabled(&self, metadata: &Metadata) -> bool {
            println!(
                "{} ? {} => {}",
                metadata.level(),
                Level::Info,
                metadata.level() <= Level::Info
            );
            metadata.level() <= Level::Info
        }
        // Logs the `Record`.
        fn log(&self, record: &Record) {
            if self.enabled(record.metadata()) {
                println!("Rust says: {} - {}", record.level(), record.args());
            }
        }
        // Flushes any buffered records.
        fn flush(&self) {}
    }

    static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;
    log::set_logger(&CONSOLE_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Info);
    log::debug!("debug");
    log::warn!("warn");
    log::error!("error");
    log::info!("info");
    log::trace!("trace");
}

pub fn log_to_sys() {
    // use syslog::
}

pub fn customed_env() {
    use env_logger::Builder;
    use std::env;

    Builder::new()
        // .parse_env(env::var("MY_APP_LOG").unwrap())
        .init();
    for argument in env::args() {
        println!("=================={}", argument);
    }
}

pub fn env_test() {
    use std::env;

    println!("{:?}", env::current_dir());
    for argument in env::args() {
        println!("=================={}", argument);
    }
    for argument in env::args_os() {
        println!("*******************8{:?}", argument);
    }
}

use error_chain::error_chain;
error_chain! {
    foreign_links {
        Io(std::io::Error);
        SetLogger(log::SetLoggerError);
        LogConfig(log4rs::config::runtime::ConfigErrors);
    }
}
pub fn customed_log_path() -> Result<()> {
    use log::LevelFilter;
    use log4rs::append::file::FileAppender;
    use log4rs::config::{Appender, Config, Root};
    use log4rs::encode::pattern::PatternEncoder;

    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("log/output.log")?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))?;

    log4rs::init_config(config)?;

    log::info!("Hello, world!");
    log::debug!("debug");
    log::warn!("warn");
    log::error!("error");
    log::info!("info");
    log::trace!("trace");

    Ok(())
}

#[test]
fn test() {
    // output_log();
    // customed_logger();
    // env_test();
    // customed_env();
    customed_log_path();
}
