use log::debug;
use rand;
use termion;

// Play Thunderball
// Enter 5 numbers from 1 to 39 and 1 Thunderball from 1 to 14. Or select Lucky Dip.
// £1.00 per play
// Play my saved numbers
pub const MAIN_TOTAL: usize = 5;
pub const MAIN_MAX: u64 = 39;
pub const EXTRA_TOTAL: usize = 1;
pub const EXTRA_MAX: u64 = 14;

#[derive(Default, Debug, Copy, Clone)]
pub struct Play {
    pub main: [u64; MAIN_TOTAL],
    pub extra: [u64; EXTRA_TOTAL],
}

impl Play {
    fn new() -> Self {
        Play {
            ..Default::default()
        }
    }
}

impl std::fmt::Display for Play {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // let mut comma_separated = String::new();

        // let len = self.main.len();
        // for (index, number) in self.main.iter().enumerate() {
        //   comma_separated.push_str(&number.to_string());
        //   if index + 1 != len {
        //     comma_separated.push_str(", ");
        //   }
        // }

        // let comma_separated = self.main.into_iter().map(|i| i.to_string()).collect::<Vec<_>>().join(", ");
        let comma_separated = self
            .main
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "[{}, {:?}]", comma_separated, self.extra)
    }
}

pub struct Plays(pub Vec<Play>);

impl Plays {
    fn new() -> Self {
        Plays(Vec::new())
    }
}

// https://stackoverflow.com/questions/31497584/how-can-i-wrap-another-type-and-add-fields-and-methods-to-it
// https://stackoverflow.com/a/31497621/241993
impl std::ops::Deref for Plays {
    type Target = Vec<Play>;
    fn deref(&self) -> &Vec<Play> {
        &self.0
    }
}

impl std::ops::DerefMut for Plays {
    fn deref_mut(&mut self) -> &mut Vec<Play> {
        &mut self.0
    }
}

// https://stackoverflow.com/questions/30320083/how-to-print-a-vec
// https://stackoverflow.com/questions/33759072/why-doesnt-vect-implement-the-display-trait
// https://stackoverflow.com/questions/30633177/implement-fmtdisplay-for-vect
// https://stackoverflow.com/a/33759549/241993
// https://gist.github.com/whatalnk/f308517fe2c497790b6ee8fcb34488ba
impl std::fmt::Display for Plays {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let mut comma_separated = String::new();

        let len = self.0.len();
        for (index, number) in self.0.iter().enumerate() {
            comma_separated.push_str("\t");
            comma_separated.push_str(&number.to_string());
            if index + 1 != len {
                comma_separated.push_str(",\n");
            }
        }

        write!(f, "[\n{}\n]", comma_separated)
    }
}

pub fn run(no_of_plays: u128) -> Result<(), std::io::Error> {
    let (width, _) = termion::terminal_size().unwrap_or((160, 48));
    let separator = "-".repeat(width as usize);
    // let x = termion::color::Rgb(r, !r, 2 * ((r % 128) as i8 - 64).abs() as u8);
    let color = termion::color::Fg(termion::color::AnsiValue::rgb(128 / 32, 0, 128 / 32));

    let mut rng = rand::thread_rng();
    let mut plays = Plays::new();
    for _ in 0..no_of_plays {
        let mut play: Play = Play::new();

        for index in 0..play.main.len() {
            // no duplicates
            loop {
                use rand::Rng;
                let number = rng.gen_range(1, MAIN_MAX + 1);
                if !play.main.iter().any(|&x| x == number) {
                    play.main[index] = number;
                    break;
                } else {
                    debug!("play.main duplicate found");
                }
            }
        }

        for index in 0..play.extra.len() {
            // no duplicates
            loop {
                use rand::Rng;
                let number = rng.gen_range(1, EXTRA_MAX + 1);
                if !play.extra.iter().any(|&x| x == number) {
                    play.extra[index] = number;
                    break;
                } else {
                    debug!("play.extra duplicate found");
                }
            }
        }

        play.main.sort();
        play.extra.sort();

        plays.push(play);
    }

    println!("{}", termion::clear::All);
    println!("{}{}{}", color, separator, termion::style::Reset);
    println!("{}thunderballs{}", color, termion::style::Reset);
    println!("{}", plays);
    println!("{}{}{}", color, separator, termion::style::Reset);

    Ok(())
}
