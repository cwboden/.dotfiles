use assert_matches::assert_matches;

mod maze;

#[derive(Debug, Eq, PartialEq)]
enum Error {
    InvalidArgument(String),
    DuplicateArgument(String),
    MissingArgument(String),
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Algorithm {
    Queue,
    Stack,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Format {
    List,
    Map,
}

#[derive(Debug, Eq, PartialEq)]
struct PathFindingArgs {
    algorithm: Option<Algorithm>,
    output_format: Option<Format>,
}

impl PathFindingArgs {
    fn new() -> Self {
        Self {
            algorithm: None,
            output_format: None,
        }
    }

    fn add_algorithm(&mut self, algorithm: Algorithm) -> Result<()> {
        match self.algorithm {
            None => {
                self.algorithm = Some(algorithm);
                Ok(())
            }
            Some(_) => Err(Error::DuplicateArgument(
                "Cannot specify more than one algorithm mode".to_string(),
            )),
        }
    }

    fn add_output_format(&mut self, output_format: Format) -> Result<()> {
        match self.output_format {
            None => {
                self.output_format = Some(output_format);
                Ok(())
            }
            Some(_) => Err(Error::DuplicateArgument(
                "Cannot specify more than one output format mode".to_string(),
            )),
        }
    }

    fn validate(self) -> Result<Self> {
        if self.algorithm.is_none() {
            return Err(Error::MissingArgument(
                "Missing algorithm argument".to_string(),
            ));
        }
        if self.output_format.is_none() {
            return Err(Error::MissingArgument(
                "Missing output format argument".to_string(),
            ));
        }

        return Ok(self);
    }
}

fn parse_args(args: &[String]) -> Result<PathFindingArgs> {
    let mut parsed_args = PathFindingArgs::new();

    for arg in args {
        match arg.as_str() {
            "-q" | "--queue" => {
                parsed_args.add_algorithm(Algorithm::Queue)?;
            }
            "-s" | "--stack" => {
                parsed_args.add_algorithm(Algorithm::Stack)?;
            }
            "-l" | "--list" => {
                parsed_args.add_output_format(Format::List)?;
            }
            "-m" | "--map" => {
                parsed_args.add_output_format(Format::Map)?;
            }
            _ => {
                return Err(Error::InvalidArgument(format!(
                    "Invalid argument: '{}'",
                    arg
                )))
            }
        }
    }

    parsed_args.validate()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let parsed_args = parse_args(&args).unwrap();
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
                ])
                .unwrap();

                assert_eq!(parsed_args.algorithm, Some(*algorithm_expected));
                assert_eq!(parsed_args.output_format, Some(*output_format_expected));
            }
        }
    }

    #[test]
    fn parse_args_algorithm_duplicate_error() {
        assert_matches!(
            parse_args(&["-s".to_string(), "-q".to_string()]),
            Err(Error::DuplicateArgument(_))
        );
    }

    #[test]
    fn parse_args_output_format_duplicate_error() {
        assert_matches!(
            parse_args(&["-m".to_string(), "-l".to_string()]),
            Err(Error::DuplicateArgument(_))
        );
    }

    #[test]
    fn parse_args_error_invalid_argument() {
        assert_matches!(
            parse_args(&["bad".to_string()]),
            Err(Error::InvalidArgument(_))
        );
    }

    #[test]
    fn parse_args_validate_missing_algorithm() {
        assert_matches!(
            parse_args(&["-m".to_string()]),
            Err(Error::MissingArgument(_))
        );
    }

    #[test]
    fn parse_args_validate_missing_output_format() {
        assert_matches!(
            parse_args(&["-q".to_string()]),
            Err(Error::MissingArgument(_))
        );
    }
}
