use crate::types::*;

pub struct Gauge<T> {
    resource: T,
    amount: u8,
    limit: u8,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    NotEnoughResources,
}

pub type Result<T> = std::result::Result<T, Error>;

impl<T> Gauge<T> {
    pub fn get(&self) -> u8 {
        self.amount
    }

    pub fn set(&mut self, amount: u8) {
        self.amount = std::cmp::min(self.limit, amount);
    }

    pub fn add(&mut self, amount: u8) -> u8 {
        self.amount = std::cmp::min(self.limit, self.amount + amount);
        self.amount
    }

    pub fn try_sub(&mut self, amount: u8) -> Result<u8> {
        if self.amount < amount {
            Err(Error::NotEnoughResources)
        } else {
            self.amount -= amount;
            Ok(self.amount)
        }
    }

    pub fn sub(&mut self, amount: u8) -> u8 {
        self.try_sub(amount).unwrap()
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

pub struct Gauges {
    ore: Gauge<Resource>,
    knowledge: Gauge<Resource>,
    credits: Gauge<Resource>,
    qic: Gauge<Resource>,
}

impl Gauges {
    pub fn new() -> Self {
        Self {
            ore: Gauge::new(Resource::Ore),
            knowledge: Gauge::new(Resource::Knowledge),
            credits: Gauge::new(Resource::Credit),
            qic: Gauge::new(Resource::Qic),
        }
    }

    pub fn get(&self, resource: Resource) -> &Gauge<Resource> {
        match resource {
            Resource::Credit => &self.credits,
            Resource::Ore => &self.ore,
            Resource::Knowledge => &self.knowledge,
            Resource::Qic => &self.qic,
            _ => panic!("No such `Gauge` for resource type: {resource:?}"),
        }
    }

    pub fn get_mut(&mut self, resource: Resource) -> &mut Gauge<Resource> {
        match resource {
            Resource::Credit => &mut self.credits,
            Resource::Ore => &mut self.ore,
            Resource::Knowledge => &mut self.knowledge,
            Resource::Qic => &mut self.qic,
            _ => panic!("No such `Gauge` for resource type: {resource:?}"),
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
    fn gauge_add() {
        let mut gauge = Gauge::new(Resource::Ore);
        assert_eq!(gauge.add(7), 7);
        assert_eq!(gauge.get(), 7);
    }

    #[test]
    fn gauge_add_respects_limit() {
        let mut gauge = Gauge::new(Resource::Ore);
        assert_eq!(gauge.add(16), 15);
        assert_eq!(gauge.get(), 15);
    }

    #[test]
    fn gauge_sub() {
        let mut gauge = Gauge::new(Resource::Ore);
        gauge.set(5);
        assert_eq!(gauge.sub(3), 2);
        assert_eq!(gauge.get(), 2);
    }

    #[test]
    #[should_panic]
    fn gauge_sub_panics_on_overflow() {
        let mut gauge = Gauge::new(Resource::Ore);
        gauge.sub(1);
    }

    #[test]
    fn gauge_try_sub() {
        let mut gauge = Gauge::new(Resource::Ore);
        gauge.set(5);
        assert_eq!(gauge.try_sub(3), Ok(2));
        assert_eq!(gauge.get(), 2);
    }

    #[test]
    fn gauge_try_sub_throws_error_on_overflow() {
        let mut gauge = Gauge::new(Resource::Ore);
        assert_eq!(gauge.try_sub(1), Err(Error::NotEnoughResources));
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

    #[test]
    fn gauges_get() {
        let gauges = Gauges::new();

        [
            Resource::Ore,
            Resource::Knowledge,
            Resource::Credit,
            Resource::Qic,
        ]
        .iter()
        .for_each(|&r| {
            assert_eq!(gauges.get(r).resource_type(), r);
        })
    }

    #[test]
    #[should_panic]
    fn gauges_get_invalid_for_power() {
        let gauges = Gauges::new();
        gauges.get(Resource::Power);
    }
}
