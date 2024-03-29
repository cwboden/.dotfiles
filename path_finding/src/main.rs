mod maze;
mod solver;

use std::io::BufReader;

use ron::ser::{to_writer_pretty, PrettyConfig};

use crate::maze::Maze;
use crate::solver::Solver;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Algorithm {
    Queue,
    Stack,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Format {
    List,
    Map,
    RustyObjectNotation,
}

#[derive(Debug, Eq, PartialEq)]
struct PathFindingArgsRaw {
    algorithm: Option<Algorithm>,
    output_format: Option<Format>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct PathFindingArgs {
    pub algorithm: Algorithm,
    pub output_format: Format,
}

impl PathFindingArgsRaw {
    #[must_use]
    fn new() -> Self {
        Self {
            algorithm: None,
            output_format: None,
        }
    }

    fn add_algorithm(&mut self, algorithm: Algorithm) {
        match self.algorithm {
            None => {
                self.algorithm = Some(algorithm);
            }
            Some(_) => panic!("Cannot specify more than one algorithm mode"),
        }
    }

    fn add_output_format(&mut self, output_format: Format) {
        match self.output_format {
            None => {
                self.output_format = Some(output_format);
            }
            Some(_) => panic!("Cannot specify more than one output format mode"),
        }
    }

    fn validate(&self) -> PathFindingArgs {
        if self.algorithm.is_none() {
            panic!("Missing algorithm argument");
        }
        if self.output_format.is_none() {
            panic!("Missing output format argument");
        }

        PathFindingArgs {
            algorithm: self.algorithm.unwrap(),
            output_format: self.output_format.unwrap(),
        }
    }
}

fn parse_args(args: &[String]) -> PathFindingArgs {
    let mut parsed_args = PathFindingArgsRaw::new();

    for arg in args {
        match arg.as_str() {
            "-q" | "--queue" => {
                parsed_args.add_algorithm(Algorithm::Queue);
            }
            "-s" | "--stack" => {
                parsed_args.add_algorithm(Algorithm::Stack);
            }
            "-l" | "--list" => {
                parsed_args.add_output_format(Format::List);
            }
            "-m" | "--map" => {
                parsed_args.add_output_format(Format::Map);
            }
            "-r" | "--ron" => {
                parsed_args.add_output_format(Format::RustyObjectNotation);
            }
            "-h" | "--help" => {
                println!("\nPath finding algorithm for solving simple mazes.");
                println!("See README.md for more details.");
                println!("  -q --queue    Use queue-based search algorithm (Breadth-First Search)");
                println!("  -s --stack    Use stack-based search algorithm (Depth-First Search)");
                println!("  -m --map      Return output in Map format (See README.md)");
                println!("  -l --list     Return output in List format (See README.md)");
                println!("  -r --ron      Return output in RustyObjectNotation (.ron) format");
                std::process::exit(0);
            }
            _ => {
                panic!("Invalid argument: '{}'", arg)
            }
        }
    }

    parsed_args.validate()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Ignore the first argument, the name of the binary
    let parsed_args = parse_args(&args[1..]);

    let reader = BufReader::new(std::io::stdin());
    let maze = Maze::from_reader(reader);

    let solver = Solver::new(parsed_args.algorithm, maze);
    let result = solver.run();

    match parsed_args.output_format {
        Format::RustyObjectNotation => {
            to_writer_pretty(
                std::io::stdout(),
                &result,
                PrettyConfig::new().depth_limit(4).indentor("\t".to_owned()),
            )
            .unwrap();
        }
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_args_all_arguments() {
        for (algorithm_arg_string, algorithm_expected) in [
            ("-q", Algorithm::Queue),
            ("--queue", Algorithm::Queue),
            ("-s", Algorithm::Stack),
            ("--stack", Algorithm::Stack),
        ]
        .iter()
        {
            for (output_format_arg_string, output_format_expected) in [
                ("-l", Format::List),
                ("--list", Format::List),
                ("-m", Format::Map),
                ("--map", Format::Map),
                ("-r", Format::RustyObjectNotation),
                ("--ron", Format::RustyObjectNotation),
            ]
            .iter()
            {
                let parsed_args = parse_args(&[
                    algorithm_arg_string.to_string(),
                    output_format_arg_string.to_string(),
                ]);

                assert_eq!(parsed_args.algorithm, *algorithm_expected);
                assert_eq!(parsed_args.output_format, *output_format_expected);
            }
        }
    }
}
