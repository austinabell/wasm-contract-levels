use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Hello {
    greeting: String,
}

impl Default for Hello {
    fn default() -> Self {
        Self {
            greeting: "Hello".to_string(),
        }
    }
}

#[near_bindgen]
impl Hello {
    /// Sets new greeting and returns old one
    pub fn set_greeting(&mut self, greeting: String) -> String {
        std::mem::replace(&mut self.greeting, greeting)
    }

    /// Log greeting with name.
    pub fn hello(&self, name: String) {
        log!("{} {}", self.greeting, name);
    }
}
