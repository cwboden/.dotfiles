struct Gauge<T> {
    resource: T,
    amount: u8,
    limit: u8,
}

impl<T> Gauge<T> {
    pub fn get(&self) -> u8 {
        self.amount
    }

    pub fn set(&mut self, amount: u8) {
        self.amount = std::cmp::min(self.limit, amount);
    }
}

impl<T: Copy> Gauge<T> {
    pub fn resource_type(&self) -> T {
        self.resource
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    struct TestResource;

    impl Gauge<TestResource> {
        pub fn new() -> Self {
            Self {
                resource: TestResource,
                amount: 0,
                limit: 15,
            }
        }
    }

    #[test]
    fn gauge_get() {
        let gauge = Gauge::<TestResource>::new();
        assert_eq!(gauge.get(), 0);
    }

    #[test]
    fn gauge_set() {
        let mut gauge = Gauge::<TestResource>::new();
        gauge.set(4);
        assert_eq!(gauge.get(), 4);
    }

    #[test]
    fn gauge_set_respects_limit() {
        let mut gauge = Gauge::<TestResource>::new();
        gauge.set(16);
        assert_eq!(gauge.get(), 15);
    }

    #[test]
    fn gauge_resource_type() {
        let gauge = Gauge::<TestResource>::new();
        assert_eq!(gauge.resource_type(), TestResource);
    }
}
