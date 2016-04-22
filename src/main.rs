#![allow(dead_code)]
#![allow(unused_variables)]
extern crate clap;

use clap::{App, Arg, ArgMatches, SubCommand};

fn main() {
    
    use shannon::ShannonEntropy;
    
    println!("{}", "Hello".entropy());
    println!("Hello, world!");
}

mod shannon {
    use std::collections::BTreeMap;
    use std::ascii::AsciiExt;

    pub trait ShannonEntropy {
        fn entropy(&self) -> f32;
    }

    impl<T: AsRef<str>> ShannonEntropy for T {
        fn entropy(&self) -> f32 {
            shannon_entropy(self.as_ref())
        }
    }

    pub fn shannon_entropy(s: &str) -> f32 {
        if s.is_empty() {
            return 0.0;
        }

        let mut char_map: BTreeMap<char, usize> = BTreeMap::new();
        let mut ascii_map: [usize; 128] = [0; 128];
        let mut s_len = 0;

        for ch in s.chars() {
            s_len += 1;
            if ch.is_ascii() {
                ascii_map[ch as usize] += 1;
            } else {
                println!("non-ascii char found.");
                *char_map.entry(ch).or_insert(0) += 1;
            }
        }

        let s_len = (s_len as f32).round();
        let log_div = (2.0 as f32).ln();

        let result = char_map.values()
                             .chain(ascii_map.into_iter())
                             .inspect(|&c| if *c > 0 { println!("{} - {}", c, 1) })
                             .fold(0.0, |acc, &c| match c {
                                 0 => acc,
                                 c => acc + (c as f32 * (c as f32 / s_len).ln()),
                             })
                             .abs();

        result / (s_len * log_div)
    }

    #[cfg(test)]
    mod tests {
        use super::ShannonEntropy;

        #[test]
        fn test_shannon() {
            let test_strings = vec![// Make sure we're consistent
                                    ("horse staple battery", "horse staple battery".entropy()),
                                    // All-ASCII strings hit the fast path
                                    ("hello world", 2.845351),
                                    ("hello worldd", 2.8553884),
                                    ("a", 0.0),
                                    ("", 0.0)];

            for (test, answer) in test_strings {
                assert_eq!(answer, test.entropy());
            }
        }
    }
}

fn date_exercise() {
    let matches = parse_args();

    if let Some(ref matches) = matches.subcommand_matches("month") {
        let day = match matches.value_of("day") {
            Some(day) => day,
            None => {
                println!("month command requires day of week and ordinal");
                return;
            }
        };

        let ordinal = match matches.value_of("ord") {
            Some(ordinal) => ordinal,
            None => {
                println!("month command requires day of week and ordinal");
                return;
            }
        };

        println!("{} {}", day, ordinal);
    } else if let Some(ref matches) = matches.subcommand_matches("year") {
        println!("year received.");
    }

    // println!("{:?}", matches);
}

fn parse_args<'a>() -> ArgMatches<'a> {
    App::new("My Program")
        .author("Me, me@mail.com")
        .version("0.1.0")
        .about("Explain brief shit")
        .subcommand(SubCommand::with_name("month")
                        .about("Allows creation of month-relative date streams")
                        .arg(Arg::with_name("day")
                                 .short("d")
                                 .help("day of the week")
                                 .takes_value(true))
                        .arg(Arg::with_name("ord")
                                 .short("o")
                                 .help("ordinal value of repeated date")
                                 .takes_value(true)))
        .subcommand(SubCommand::with_name("week")
                        .about("Allow creation of week-relative date streams.")
                        .arg(Arg::with_name("day")
                                 .short("d")
                                 .help("day of the week")
                                 .takes_value(true)))
        .subcommand(SubCommand::with_name("year")
                        .about("Allows immortals to sort out their calendars.")
                        .arg(Arg::with_name("day")
                                 .short("d")
                                 .help("day of the week")
                                 .takes_value(true)))
        .get_matches()
}