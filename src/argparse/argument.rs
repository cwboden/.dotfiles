use std::collections::HashSet;
use std::fmt::{Display, Formatter};

pub struct Argument<'a, T> {
    pub identifiers: HashSet<&'a str>,
    pub callbacks: Vec<Box<dyn Fn(&mut T)>>,
    pub help_text: Option<&'a str>,
}

impl<'a, T> Argument<'a, T> {
    pub fn new() -> Self {
        Self {
            identifiers: HashSet::new(),
            callbacks: Vec::new(),
            help_text: None,
        }
    }

    pub fn with_identifiers(mut self, identifiers: &[&'a str]) -> Self {
        for identifier in identifiers.iter() {
            self.identifiers.insert(identifier);
        }

        self
    }

    pub fn with_callback<F: 'static + Copy + Fn(&mut T)>(mut self, callback: F) -> Self {
        self.callbacks.push(Box::new(callback));
        self
    }

    pub fn with_help_text(mut self, help_text: &'a str) -> Self {
        self.help_text = Some(help_text);
        self
    }
}

impl<T> Display for Argument<'_, T> {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        for identifier in self.identifiers.iter() {
            formatter
                .write_str(format!("{} ", identifier).as_str())
                .unwrap();
        }
        formatter
            .write_str(format!(": {}", self.help_text.unwrap_or("")).as_str())
            .unwrap();

        Ok(())
    }
}
