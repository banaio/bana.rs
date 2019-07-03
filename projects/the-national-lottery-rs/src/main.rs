use log::info;

mod cli;
mod euromillions;
mod thunderball;

fn main() -> Result<(), std::io::Error> {
    // std::env::set_var("RUST_BACKTRACE", "full");
    std::env::set_var("RUST_BACKTRACE", "1");
    std::env::set_var("RUST_LOG", "trace");
    env_logger::init();

    let cli = cli::new();
    info!("cli={:?}", cli);

    match cli.game {
        cli::Game::EuroMillions => euromillions::run(cli.no_of_plays),
        cli::Game::Thunderball => thunderball::run(cli.no_of_plays),
    }
}
