mod maze;
mod solver;

use dotfiles::argparse;

use ron::ser::{to_writer_pretty, PrettyConfig};
use std::io::BufReader;

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

fn parse_args(args: Vec<String>) -> PathFindingArgs {
    let mut parsed_args = PathFindingArgsRaw::new();
    let mut parser = argparse::Parser::new(&mut parsed_args);

    for (ids, algorithm, help_text) in [
        (
            ["-q", "--queue"],
            Algorithm::Queue,
            "Use queue-based search algorithm (Breadth-First Search)",
        ),
        (
            ["-s", "--stack"],
            Algorithm::Stack,
            "Use stack-based search algorithm (Depth-First Search)",
        ),
    ]
    .iter()
    {
        parser
            .add_argument(
                argparse::Argument::default()
                    .with_identifiers(ids)
                    .with_callback(move |a: &mut PathFindingArgsRaw| a.add_algorithm(*algorithm))
                    .with_help_text(help_text),
            )
            .unwrap();
    }

    for (ids, format, help_text) in [
        (
            ["-l", "--list"],
            Format::List,
            "Return output in Map format (See README.md)",
        ),
        (
            ["-m", "--map"],
            Format::Map,
            "Return output in List format (See README.md)",
        ),
        (
            ["-r", "--ron"],
            Format::RustyObjectNotation,
            "Return output in RustyObjectNotation (.ron) format",
        ),
    ]
    .iter()
    {
        parser
            .add_argument(
                argparse::Argument::default()
                    .with_identifiers(ids)
                    .with_callback(move |a: &mut PathFindingArgsRaw| a.add_output_format(*format))
                    .with_help_text(help_text),
            )
            .unwrap();
    }

    match parser.parse(&args) {
        Ok(_) => (),
        Err(e) => {
            parser.print_help_text(std::io::stdout());

            if e == argparse::Error::HelpTextRequested {
                std::process::exit(0)
            } else {
                std::process::exit(2)
            }
        }
    }

    parsed_args.validate()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Ignore the first argument, the name of the binary
    let parsed_args = parse_args(args);

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
                let parsed_args = parse_args(vec![
                    algorithm_arg_string.to_string(),
                    output_format_arg_string.to_string(),
                ]);

                assert_eq!(parsed_args.algorithm, *algorithm_expected);
                assert_eq!(parsed_args.output_format, *output_format_expected);
            }
        }
    }
}
