use crate::types::*;

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

impl Gauge<Resource> {
    pub fn new(resource: Resource) -> Self {
        let limit = match resource {
            Resource::Ore | Resource::Knowledge => 15,
            Resource::Credit => 30,
            Resource::Qic => u8::MAX,
            _ => panic!("Cannot create a `Gauge` for resource type: {resource:?}"),
        };

        Self {
            resource,
            amount: 0,
            limit,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gauge_get() {
        let gauge = Gauge::new(Resource::Ore);
        assert_eq!(gauge.get(), 0);
    }

    #[test]
    fn gauge_set() {
        let mut gauge = Gauge::new(Resource::Ore);
        gauge.set(4);
        assert_eq!(gauge.get(), 4);
    }

    #[test]
    fn gauge_set_respects_limit() {
        let mut gauge = Gauge::new(Resource::Ore);
        gauge.set(16);
        assert_eq!(gauge.get(), 15);
    }

    #[test]
    fn gauge_resource_type() {
        let gauge = Gauge::new(Resource::Ore);
        assert_eq!(gauge.resource_type(), Resource::Ore);
    }

    #[test]
    fn gauge_ore_limit_is_15() {
        let mut gauge = Gauge::new(Resource::Ore);
        gauge.set(16);
        assert_eq!(gauge.get(), 15);
    }

    #[test]
    fn gauge_credit_limit_is_30() {
        let mut gauge = Gauge::new(Resource::Credit);
        gauge.set(31);
        assert_eq!(gauge.get(), 30);
    }

    #[test]
    fn gauge_knowledge_limit_is_15() {
        let mut gauge = Gauge::new(Resource::Knowledge);
        gauge.set(16);
        assert_eq!(gauge.get(), 15);
    }

    #[test]
    fn gauge_qic_limit_is_unlimited() {
        let mut gauge = Gauge::new(Resource::Qic);
        gauge.set(u8::MAX);
        assert_eq!(gauge.get(), u8::MAX);
    }

    #[test]
    #[should_panic]
    fn gauge_power_is_not_valid() {
        let _ = Gauge::new(Resource::Power);
    }
}
