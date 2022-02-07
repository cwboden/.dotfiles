use maze::Format;

mod maze;
mod solver;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Algorithm {
    Queue,
    Stack,
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

fn parse_args(args: &[String]) -> PathFindingArgs{
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
            _ => {
                panic!("Invalid argument: '{}'", arg)
            }
        }
    }

    parsed_args.validate()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    parse_args(&args);
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
