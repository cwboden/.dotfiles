use assert_matches::assert_matches;

#[derive(Debug, Eq, PartialEq)]
enum Error {
    InvalidArgument(String),
    DuplicateArgument(String),
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Eq, PartialEq)]
enum Algorithm {
    Queue,
    Stack,
}

#[derive(Debug, Eq, PartialEq)]
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

    Ok(parsed_args)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_args_algorithm_queue() {
        assert_eq!(
            parse_args(&["-q".to_string()]).unwrap().algorithm,
            Some(Algorithm::Queue)
        );
        assert_eq!(
            parse_args(&["--queue".to_string()]).unwrap().algorithm,
            Some(Algorithm::Queue),
        );
    }

    #[test]
    fn parse_args_algorithm_stack() {
        assert_eq!(
            parse_args(&["-s".to_string()]).unwrap().algorithm,
            Some(Algorithm::Stack)
        );
        assert_eq!(
            parse_args(&["--stack".to_string()]).unwrap().algorithm,
            Some(Algorithm::Stack)
        );
    }

    #[test]
    fn parse_args_algorithm_duplicate_error() {
        assert_matches!(
            parse_args(&["-s".to_string(), "-q".to_string()]),
            Err(Error::DuplicateArgument(_))
        );
    }

    #[test]
    fn parse_args_output_format_list() {
        assert_eq!(
            parse_args(&["-l".to_string()]).unwrap().output_format,
            Some(Format::List)
        );
        assert_eq!(
            parse_args(&["--list".to_string()]).unwrap().output_format,
            Some(Format::List)
        );
    }

    #[test]
    fn parse_args_output_format_map() {
        assert_eq!(
            parse_args(&["-m".to_string()]).unwrap().output_format,
            Some(Format::Map)
        );
        assert_eq!(
            parse_args(&["--map".to_string()]).unwrap().output_format,
            Some(Format::Map)
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
}
