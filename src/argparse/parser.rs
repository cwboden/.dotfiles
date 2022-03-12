use std::collections::HashMap;

struct Parser<'a, T> {
    output_args: &'a mut T,
    input_arg_to_funcs: HashMap<String, Box<dyn Fn(&mut T)>>,
}

impl<'a, T> Parser<'a, T> {
    pub fn new(output_args: &'a mut T) -> Self {
        Self {
            output_args,
            input_arg_to_funcs: HashMap::new(),
        }
    }

    pub fn add_flag<F>(&mut self, flag: String, func: F) where F: 'static + Fn(&mut T) {
        assert!(self.input_arg_to_funcs.insert(flag, Box::new(func)).is_none());
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
    fn parse_single_flag() {
        let mut test_args = TestArgs::default();
        let mut parser = Parser::new(&mut test_args);

        parser.add_flag("--flag".to_owned(), |t| t.arg_flag = true);
        parser.parse(&["--flag".to_owned()]);

        assert!(test_args.arg_flag);
    }
}
