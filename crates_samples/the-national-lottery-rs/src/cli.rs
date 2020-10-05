use structopt::StructOpt;

// Our CLI arguments. (help and version are automatically generated)
// Documentation on how to use:
// https://docs.rs/structopt/0.2.10/structopt/index.html#how-to-derivestructopt
#[derive(StructOpt, Debug)]
#[structopt(raw(global_settings = "&[
        structopt::clap::AppSettings::ColoredHelp,
        structopt::clap::AppSettings::VersionlessSubcommands,
        structopt::clap::AppSettings::ArgRequiredElseHelp,
        ]"))]
pub struct Cli {
    #[structopt(
        short = "g",
        long = "game",
        help = "The game to play",
        raw(default_value = "Game::default().as_str()"),
        raw(possible_values = "&Game::possible_values()")
    )]
    pub game: Game,
    #[structopt(
        short = "n",
        long = "no_of_plays",
        help = "The number of plays",
        default_value = "7"
    )]
    pub no_of_plays: u128,
}

#[derive(Debug)]
pub enum Game {
    EuroMillions,
    Thunderball,
}

impl std::str::FromStr for Game {
    type Err = Box<std::error::Error>;

    fn from_str(s: &str) -> Result<Game, Self::Err> {
        use Game::*;
        match s {
            "euromillions" => Ok(EuroMillions),
            "thunderball" => Ok(Thunderball),
            game => Err(format!("game={} invalid", game).into()),
        }
    }
}

impl std::default::Default for Game {
    fn default() -> Self {
        Game::EuroMillions
    }
}

impl Game {
    pub fn as_str(&self) -> &'static str {
        use Game::*;
        match self {
            &EuroMillions => "euromillions",
            &Thunderball => "thunderball",
        }
    }

    pub fn possible_values() -> &'static [&'static str] {
        &["euromillions", "thunderball"]
    }
}

pub fn new() -> Cli {
    let args = Cli::from_args();
    args
}
