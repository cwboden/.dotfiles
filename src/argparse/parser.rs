use std::collections::HashMap;

pub struct Parser<'a, T> {
    output_args: &'a mut T,
    input_arg_to_funcs: HashMap<&'a str, Box<dyn Fn(&mut T)>>,
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
            input_arg_to_funcs: HashMap::new(),
        }
    }

    pub fn add_flag<F>(&mut self, identifiers: &[&'a str], func: F) -> Result<()>
    where
        F: 'static + Fn(&mut T) + Copy,
    {
        for identifier in identifiers.iter() {
            if self
                .input_arg_to_funcs
                .insert(identifier, Box::new(func))
                .is_some()
            {
                return Err(Error::ArgAlreadyUsed(format!(
                    "Identifier '{}' already used.",
                    identifier
                )));
            }
        }
        Ok(())
    }

    pub fn parse(self, args: &[&str]) -> Result<()> {
        for arg in args.iter() {
            let func = self
                .input_arg_to_funcs
                .get(arg)
                .ok_or(Error::ArgNotRecognized(format!(
                    "Argument '{}' not recognized.",
                    arg
                )))?;
            func(self.output_args);
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
    }

    #[test]
    fn parse_flag() {
        let mut test_args = TestArgs::default();
        let mut parser = Parser::new(&mut test_args);

        parser
            .add_flag(&["--flag"], |t| t.arg_flag = true)
            .unwrap();
        parser.parse(&["--flag"]);

        assert!(test_args.arg_flag);
    }

    #[test]
    fn parse_flag_with_multiple_keys() {
        let keys = [
            "-f",
            "--flag",
            "--really-verbose-flag",
        ];

        for key in keys.iter() {
            let mut test_args = TestArgs::default();
            let mut parser = Parser::new(&mut test_args);

            parser.add_flag(&keys, |t| t.arg_flag = true).unwrap();
            parser.parse(&[&key]);

            assert!(test_args.arg_flag);
        }
    }

    #[test]
    fn cannot_register_flag_more_than_once() {
        let mut test_args = TestArgs::default();
        let mut parser = Parser::new(&mut test_args);

        assert_eq!(
            parser.add_flag(&["-f", "-f"], |t| t.arg_flag = true),
            Err(Error::ArgAlreadyUsed(
                "Identifier '-f' already used.".to_string()
            ))
        );
    }

    #[test]
    fn parse_throws_error_for_unrecognized_arg() {
        let mut test_args = TestArgs::default();
        let mut parser = Parser::new(&mut test_args);

        assert_eq!(
            parser.parse(&["-f"]),
            Err(Error::ArgNotRecognized(
                "Argument '-f' not recognized.".to_string()
            ))
        );
    }
}
