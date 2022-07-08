use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen};


#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Counter {
    val: i8,
}

#[near_bindgen]
impl Counter {
    /// Public method: Returns the counter value.
    pub fn get_num(&self) -> i8 {
        return self.val;
    }

    /// Public method: Increment the counter.
    pub fn increment(&mut self, count: i8) {
        self.val += count;
        log!("Increased number to {}", self.val);
    }

    /// Public method - Reset to zero.
    pub fn reset(&mut self) {
        self.val = 0;
        log!("Reset counter to zero");
    }
}

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increment() {
        // instantiate a contract variable with the counter at zero
        let mut contract = Counter { val: 0 };
        contract.increment(5);
        assert_eq!(5, contract.get_num());
    }

    #[test]
    fn increment_and_reset() {
        let mut contract = Counter { val: 0 };
        contract.increment(1);
        contract.reset();
        assert_eq!(0, contract.get_num());
    }

    #[test]
    #[should_panic]
    fn panics_on_overflow() {
        let mut contract = Counter { val: 127 };
        contract.increment(1);
    }
}
