use structopt::StructOpt;

// Our CLI arguments. (help and version are automatically generated)
// Documentation on how to use:
// https://docs.rs/structopt/0.2.10/structopt/index.html#how-to-derivestructopt
#[derive(
    StructOpt, Debug, Default, Clone, Hash, PartialEq, PartialOrd, Eq, Ord,
)]
#[structopt(global_settings = &[
        structopt::clap::AppSettings::ColoredHelp,
        structopt::clap::AppSettings::VersionlessSubcommands,
        structopt::clap::AppSettings::ArgRequiredElseHelp,
        ])]
pub struct Cli {
    #[structopt(
        short = "l",
        long = "log_level",
        help = "The level to configure the logger.",
        default_value = LogLevel::default().as_str(),
        possible_values = &LogLevel::possible_values(),
    )]
    pub log_level: LogLevel,
    #[structopt(
        short = "c",
        long = "config",
        help = "The config file to use.",
        parse(from_os_str),
        default_value = "config.yml"
    )]
    pub config: std::path::PathBuf,
}

#[derive(StructOpt, Debug, Clone, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl std::str::FromStr for LogLevel {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<LogLevel, Self::Err> {
        use LogLevel::*;
        match s {
            "error" => Ok(Error),
            "warn" => Ok(Warn),
            "info" => Ok(Info),
            "debug" => Ok(Debug),
            "trace" => Ok(Trace),
            log_level => {
                Err(format!("error: invalid log_level={}", log_level).into())
            }
        }
    }
}

impl std::default::Default for LogLevel {
    fn default() -> Self {
        LogLevel::Info
    }
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        use LogLevel::*;
        match self {
            Error => "error",
            Warn => "warn",
            Info => "info",
            Debug => "debug",
            Trace => "trace",
        }
    }

    pub fn possible_values() -> &'static [&'static str] {
        &["error", "warn", "info", "debug", "trace"]
    }
}

pub fn new() -> Cli {
    Cli::from_args()
}

// #[cfg(test)]
// mod tests {
//     use pretty_assertions::assert_eq;
//     use serde_json;

//     #[test]
//     fn test_deserialize_good() {
//         let expected = super::ClientCredentialsGrant {
//             access_token: "c875cc35-b712-4904-9ff1-9de9dc2b6014".to_string(),
//             token_type:   "Bearer".to_string(),
//             expires_in:   std::time::Duration::from_secs(3600),
//         };
//         let s = r#"
// {
//     "access_token": "c875cc35-b712-4904-9ff1-9de9dc2b6014",
//     "token_type": "Bearer",
//     "expires_in": 3600
// }
//         "#;
//         let actual =
// serde_json::from_str::<super::ClientCredentialsGrant>(s).unwrap();

//         assert_eq!(actual, expected);
//     }
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
