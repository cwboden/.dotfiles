use crate::argparse::Argument;
use std::collections::HashSet;
use std::io::Write;

pub struct Parser<'a, T> {
    output_args: &'a mut T,
    arguments: Vec<Argument<'a, T>>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    ArgAlreadyUsed(String),
    ArgNotRecognized(String),
    CannotOverrideHelpArg,
    HelpTextRequested,
}

type Result<T> = std::result::Result<T, Error>;

impl<'a, T> Parser<'a, T> {
    const HELP_IDENTIFIERS: [&'static str; 2] = ["-h", "--help"];

    pub fn new(output_args: &'a mut T) -> Self {
        Self {
            output_args,
            arguments: Vec::new(),
        }
    }

    pub fn add_argument(&mut self, arg: Argument<'a, T>) -> Result<()> {
        if Self::HELP_IDENTIFIERS
            .iter()
            .any(|help_id| arg.identifiers.contains(help_id))
        {
            return Err(Error::CannotOverrideHelpArg);
        }

        if let Some(existing_arg) = self
            .arguments
            .iter()
            .find(|a| !a.identifiers.is_disjoint(&arg.identifiers))
        {
            let identifiers: Vec<_> = existing_arg
                .identifiers
                .intersection(&arg.identifiers)
                .collect();
            return Err(Error::ArgAlreadyUsed(format!(
                "Identifiers {:?} already used.",
                identifiers
            )));
        }

        self.arguments.push(arg);
        Ok(())
    }

    pub fn parse<S: Into<String> + Clone>(&mut self, args: &[S]) -> Result<()> {
        let args_set: HashSet<String> = args
            .iter()
            .map(|a| Into::<String>::into(a.clone()))
            .collect();
        if Self::HELP_IDENTIFIERS
            .iter()
            .any(|help_id| args_set.contains(&help_id.to_string()))
        {
            return Err(Error::HelpTextRequested);
        }

        for arg in args.iter() {
            let arg_string: String = arg.clone().into();
            let argument = self
                .arguments
                .iter()
                .find(|a| a.identifiers.contains(arg_string.as_str()))
                .ok_or(Error::ArgNotRecognized(format!(
                    "Argument '{}' not recognized.",
                    arg_string
                )))?;

            for func in argument.callbacks.iter() {
                func(self.output_args);
            }
        }
        Ok(())
    }

    pub fn print_help_text<W: Write>(&self, mut writer: W) {
        writer
            .write(b"-h --help : Display this help text\n")
            .unwrap();
        for argument in self.arguments.iter() {
            writer.write(format!("{}\n", argument).as_bytes()).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct TestArgs {
        arg_flag: bool,
        arg_number: u32,
    }

    #[test]
    fn parse_flag() {
        let mut test_args = TestArgs::default();
        let mut parser = Parser::new(&mut test_args);

        parser
            .add_argument(
                Argument::new()
                    .with_identifiers(&["--flag"])
                    .with_callback(|t: &mut TestArgs| t.arg_flag = true),
            )
            .unwrap();
        parser.parse(&["--flag"]).unwrap();

        assert!(test_args.arg_flag);
    }

    #[test]
    fn parse_flag_with_multiple_keys() {
        let keys = ["-f", "--flag", "--really-verbose-flag"];

        for &key in keys.iter() {
            let mut test_args = TestArgs::default();
            let mut parser = Parser::new(&mut test_args);

            parser
                .add_argument(
                    Argument::new()
                        .with_identifiers(&keys)
                        .with_callback(|t: &mut TestArgs| t.arg_flag = true),
                )
                .unwrap();
            parser.parse(&[key]).unwrap();

            assert!(test_args.arg_flag);
        }
    }

    #[test]
    fn can_register_multiple_flags() {
        let mut test_args = TestArgs::default();
        let mut parser = Parser::new(&mut test_args);

        parser
            .add_argument(
                Argument::new()
                    .with_identifiers(&["-f"])
                    .with_callback(|t: &mut TestArgs| t.arg_flag = true),
            )
            .unwrap();

        parser
            .add_argument(
                Argument::new()
                    .with_identifiers(&["-n"])
                    .with_callback(|t: &mut TestArgs| t.arg_number = 13),
            )
            .unwrap();

        parser.parse(&["-f", "-n"]).unwrap();

        assert!(test_args.arg_flag);
        assert_eq!(test_args.arg_number, 13);
    }

    #[test]
    fn cannot_register_flag_more_than_once() {
        let mut test_args = TestArgs::default();
        let mut parser = Parser::new(&mut test_args);
        parser
            .add_argument(
                Argument::new()
                    .with_identifiers(&["-f"])
                    .with_callback(|t: &mut TestArgs| t.arg_flag = true),
            )
            .unwrap();

        assert_eq!(
            parser.add_argument(
                Argument::new()
                    .with_identifiers(&["-f"])
                    .with_callback(|t: &mut TestArgs| t.arg_flag = true)
            ),
            Err(Error::ArgAlreadyUsed(
                "Identifiers [\"-f\"] already used.".to_string()
            ))
        );
    }

    #[test]
    fn cannot_override_help_arg() {
        let mut test_args = TestArgs::default();
        let mut parser = Parser::new(&mut test_args);

        assert_eq!(
            parser.add_argument(
                Argument::new()
                    .with_identifiers(&["-h"])
                    .with_callback(|t: &mut TestArgs| t.arg_flag = true)
            ),
            Err(Error::CannotOverrideHelpArg)
        );
    }

    #[test]
    fn parse_throws_error_for_unrecognized_arg() {
        let mut test_args = TestArgs::default();
        let mut parser = Parser::new(&mut test_args);
        parser
            .add_argument(
                Argument::new()
                    .with_identifiers(&["-f", "--flag"])
                    .with_callback(|t: &mut TestArgs| t.arg_flag = true),
            )
            .unwrap();

        assert_eq!(
            parser.parse(&["--different"]),
            Err(Error::ArgNotRecognized(
                "Argument '--different' not recognized.".to_string()
            ))
        );
    }

    #[test]
    fn parse_throws_error_for_help_text() {
        let mut test_args = TestArgs::default();
        let mut parser = Parser::new(&mut test_args);

        assert_eq!(parser.parse(&["--help"]), Err(Error::HelpTextRequested),);
    }

    #[test]
    fn parse_with_invalid_argument_throws_error_for_help_text() {
        let mut test_args = TestArgs::default();
        let mut parser = Parser::new(&mut test_args);
        parser
            .add_argument(Argument::new().with_identifiers(&["-f", "--flag"]))
            .unwrap();

        assert_eq!(
            parser.parse(&["-f", "--different", "--help"]),
            Err(Error::HelpTextRequested),
        );
    }

    #[test]
    fn print_help_text() {
        let mut test_args = TestArgs::default();
        let mut parser = Parser::new(&mut test_args);
        parser
            .add_argument(
                Argument::new()
                    .with_identifiers(&["-f"])
                    .with_help_text("this is the help text for a boolean flag"),
            )
            .unwrap();
        parser
            .add_argument(
                Argument::new()
                    .with_identifiers(&["-n"])
                    .with_help_text("this is the help text for a number flag"),
            )
            .unwrap();

        let mut buf = Vec::new();
        parser.print_help_text(&mut buf);

        assert_eq!(
            String::from_utf8(buf).unwrap(),
            "-h --help : Display this help text\n\
             -f : this is the help text for a boolean flag\n\
             -n : this is the help text for a number flag\n"
        );
    }
}
