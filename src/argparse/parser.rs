use crate::argparse::Argument;

pub struct Parser<'a, T> {
    output_args: &'a mut T,
    arguments: Vec<Argument<'a, T>>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    ArgAlreadyUsed(String),
    ArgNotRecognized(String),
}

type Result<T> = std::result::Result<T, Error>;

impl<'a, T> Parser<'a, T> {
    pub fn new(output_args: &'a mut T) -> Self {
        Self {
            output_args,
            arguments: Vec::new(),
        }
    }

    pub fn add_argument(&mut self, arg: Argument<'a, T>) -> Result<()> {
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

    pub fn parse<S: Into<String> + Clone>(self, args: &[S]) -> Result<()> {
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
        parser.add_argument(
            Argument::new()
                .with_identifiers(&["-f"])
                .with_callback(|t: &mut TestArgs| t.arg_flag = true),
        );

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
}
