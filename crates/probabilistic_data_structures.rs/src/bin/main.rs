use better_panic;
use log::{debug, info, trace};
use std::error::Error;
extern crate probabilistic_data_structures;

pub mod lib;

type ErrorType = Box<dyn Error + 'static>;

fn main() -> Result<(), ErrorType> {
    std::env::set_var("RUST_BACKTRACE", "1");
    std::env::set_var("RUST_LOG", "trace");

    better_panic::Settings::debug()
        .most_recent_first(true)
        .backtrace_first(true)
        .lineno_suffix(true)
        .verbosity(better_panic::Verbosity::Full)
        .install();

    let cli = lib::cli::new();

    let log_level = cli.log_level.as_str();
    let rust_log = format!("{}", log_level);
    std::env::set_var("RUST_LOG", rust_log);
    env_logger::builder().format_timestamp(None).init();
    // env_logger::from_env(Env::default().format_timestamp(None).
    // default_filter_or("warn")).init();

    print_separator();
    print_library_info();
    info!("cli={:?}", cli);

    let lines: Vec<String> = read_lines()?;
    trace!("lines={:?}", lines);

    let b = 5;
    let mut pcsa = probabilistic_data_structures::pcsa::PCSA::new(b)?;
    for line in lines {
        pcsa.add(line)?;
    }

    debug!("pcsa={:?}", pcsa);
    info!("pcsa.cardinality()={:?}", pcsa.cardinality());
    print_separator();

    Ok(())
}

fn read_lines() -> std::io::Result<Vec<String>> {
    use std::{
        fs::File,
        io::{prelude::*, BufReader},
    };

    let file = File::open("testdata/countdistinct/elements.txt")?;
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        let l = line?;
        trace!("line={}", l);
        lines.push(l);
    }

    Ok(lines)
}

fn print_separator() {
    let (width, _height) = lib::terminal_utils::dimensions_or_exit();
    let separator: String = "-"
        .repeat((width - "[2020-08-20T10:11:28Z INFO  main] ".len()) as usize);
    // https://www.lihaoyi.com/post/BuildyourownCommandLinewithANSIescapecodes.html
    const COLOUR_BORDER: &str = "\x1b[44m";
    const RESET: &str = "\x1b[0m";
    trace!("{}{}{}", COLOUR_BORDER, separator, RESET);
}

fn print_library_info() {
    let pkg_name = env!("CARGO_PKG_NAME");
    let pkg_version = env!("CARGO_PKG_VERSION");
    let pkg_repository = env!("CARGO_PKG_REPOSITORY");
    trace!("main: {}@{} -> {}", pkg_name, pkg_version, pkg_repository);
}
