use crate::types::*;

pub struct Gauge {
    amount: u8,
    limit: u8,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    NotEnoughResources,
}

pub type Result<T> = std::result::Result<T, Error>;

impl Gauge {
    pub fn get(&self) -> u8 {
        self.amount
    }

    pub fn set(&mut self, amount: u8) {
        self.amount = std::cmp::min(self.limit, amount);
    }

    pub fn add(&mut self, amount: u8) {
        self.amount = std::cmp::min(self.limit, self.amount + amount);
    }

    pub fn try_sub(&mut self, amount: u8) -> Result<()> {
        if self.amount < amount {
            Err(Error::NotEnoughResources)
        } else {
            self.amount -= amount;
            Ok(())
        }
    }

    pub fn sub(&mut self, amount: u8) {
        self.try_sub(amount).unwrap();
    }
}

impl Gauge {
    pub fn new(resource: Resource) -> Self {
        let limit = match resource {
            Resource::Ore | Resource::Knowledge => 15,
            Resource::Credit => 30,
            Resource::Qic => u8::MAX,
            _ => panic!("Cannot create a `Gauge` for resource type: {resource:?}"),
        };

        Self { amount: 0, limit }
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
        gauge.add(7);
        assert_eq!(gauge.get(), 7);
    }

    #[test]
    fn gauge_add_respects_limit() {
        let mut gauge = Gauge::new(Resource::Ore);
        gauge.add(16);
        assert_eq!(gauge.get(), 15);
    }

    #[test]
    fn gauge_sub() {
        let mut gauge = Gauge::new(Resource::Ore);
        gauge.set(5);
        gauge.sub(3);
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
        assert_eq!(gauge.try_sub(3), Ok(()));
        assert_eq!(gauge.get(), 2);
    }

    #[test]
    fn gauge_try_sub_throws_error_on_overflow() {
        let mut gauge = Gauge::new(Resource::Ore);
        assert_eq!(gauge.try_sub(1), Err(Error::NotEnoughResources));
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
    fn gauge_power_charge_is_not_valid() {
        let _ = Gauge::new(Resource::PowerCharge);
    }

    #[test]
    #[should_panic]
    fn gauge_power_tokens_is_not_valid() {
        let _ = Gauge::new(Resource::PowerTokens);
    }
}
