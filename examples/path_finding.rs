use assert_matches::assert_matches;

#[derive(Debug, Eq, PartialEq)]
enum Error {
    InvalidArgument(String),
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Eq, PartialEq)]
enum Mode {
    Queue,
    Stack,
}

#[derive(Debug, Eq, PartialEq)]
struct PathFindingArgs {
    mode: Option<Mode>,
}

impl PathFindingArgs {
    fn new() -> Self {
        Self { mode: None }
    }

    fn add_mode(&mut self, mode: Mode) -> Result<()> {
        match self.mode {
            None => {
                self.mode = Some(mode);
                Ok(())
            }
            Some(_) => Err(Error::InvalidArgument(
                "Cannot specify more than one mode".to_string(),
            )),
        }
    }
}

fn parse_args(args: &[String]) -> Result<PathFindingArgs> {
    let mut parsed_args = PathFindingArgs::new();

    for arg in args {
        if arg == "-q" || arg == "--queue" {
            parsed_args.add_mode(Mode::Queue)?;
        } else if arg == "-s" || arg == "--stack" {
            parsed_args.add_mode(Mode::Stack)?;
        } else {
            return Err(Error::InvalidArgument(format!("Invalid argument: '{}'", arg)));
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
    fn parse_args_mode_queue() {
        assert_eq!(
            parse_args(&["-q".to_string()]).unwrap().mode,
            Some(Mode::Queue)
        );
        assert_eq!(
            parse_args(&["--queue".to_string()]).unwrap().mode,
            Some(Mode::Queue),
        );
    }

    #[test]
    fn parse_args_mode_stack() {
        assert_eq!(
            parse_args(&["-s".to_string()]).unwrap().mode,
            Some(Mode::Stack)
        );
        assert_eq!(
            parse_args(&["--stack".to_string()]).unwrap().mode,
            Some(Mode::Stack)
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
