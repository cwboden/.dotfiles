use std::collections::HashSet;

pub struct Argument<'a, T> {
    pub identifiers: HashSet<&'a str>,
    pub callbacks: Vec<Box<dyn Fn(&mut T)>>,
}

impl<'a, T> Argument<'a, T> {
    pub fn new() -> Self {
        Self {
            identifiers: HashSet::new(),
            callbacks: Vec::new(),
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
}
