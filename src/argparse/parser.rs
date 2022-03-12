use std::collections::HashMap;

struct Parser<'a, T> {
    output_args: &'a mut T,
    input_arg_to_funcs: HashMap<String, Box<dyn Fn(&mut T)>>,
}

#[derive(Debug, Eq, PartialEq)]
enum Error {
    ArgAlreadyUsed(String),
}

type Result<T> = std::result::Result<T, Error>;

impl<'a, T> Parser<'a, T> {
    pub fn new(output_args: &'a mut T) -> Self {
        Self {
            output_args,
            input_arg_to_funcs: HashMap::new(),
        }
    }

    pub fn add_flag<F>(&mut self, identifiers: &[String], func: F) -> Result<()>
    where
        F: 'static + Fn(&mut T) + Copy,
    {
        for identifier in identifiers.iter() {
            if self
                .input_arg_to_funcs
                .insert(identifier.to_string(), Box::new(func))
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

    pub fn parse(self, args: &[String]) {
        for arg in args.iter() {
            let func = self.input_arg_to_funcs.get(arg).unwrap();
            func(self.output_args);
        }
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
            .add_flag(&["--flag".to_owned()], |t| t.arg_flag = true)
            .unwrap();
        parser.parse(&["--flag".to_owned()]);

        assert!(test_args.arg_flag);
    }

    #[test]
    fn parse_flag_with_multiple_keys() {
        let keys = vec![
            "-f".to_owned(),
            "--flag".to_owned(),
            "--really-verbose-flag".to_owned(),
        ];

        for key in keys.iter() {
            let mut test_args = TestArgs::default();
            let mut parser = Parser::new(&mut test_args);

            parser.add_flag(&keys, |t| t.arg_flag = true).unwrap();
            parser.parse(&[key.to_string()]);

            assert!(test_args.arg_flag);
        }
    }

    #[test]
    fn cannot_register_flag_more_than_once() {
        let mut test_args = TestArgs::default();
        let mut parser = Parser::new(&mut test_args);

        assert_eq!(
            parser.add_flag(&["-f".to_owned(), "-f".to_owned()], |t| t.arg_flag = true),
            Err(Error::ArgAlreadyUsed(
                "Identifier '-f' already used.".to_string()
            ))
        );
    }
}
