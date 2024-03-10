#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod stator {
    use ink::prelude::string::String;

    #[ink(event)]
    pub struct stated {
        from: Option<AccountId>,
        message: String,
    }

    #[ink(storage)]
    pub struct Stator {
        message: String,
    }

    impl Stator {
        /// Creates a new greeter contract initialized with the given value.
        #[ink(constructor)]
        pub fn new(init_value: String) -> Self {
            Self {
                message: init_value,
            }
        }

        /// Creates a new greeter contract initialized to 'Hello ink!'.
        #[ink(constructor)]
        pub fn default() -> Self {
            let default_message = String::from("Hello ink!");
            Self::new(default_message)
        }

        /// Returns the current value of `message`.
        #[ink(message)]
        pub fn stator(&self) -> String {
            self.message.clone()
        }

        /// Sets `message` to the given value.
        #[ink(message)]
        pub fn set_message(&mut self, new_value: String) {
            self.message = new_value.clone();

            let from = self.env().caller();
            self.env().emit_event(Greeted {
                from: Some(from),
                message: new_value,
            });
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn new_works() {
            let message = "Hello ink! v4".to_string();
            let greeter = Greeter::new(message.clone());
            assert_eq!(stator.status(), message);
        }

        #[ink::test]
        fn default_new_works() {
            let greeter = Stator::default();
            let default_message = String::from("Hello ink!");
            assert_eq!(stator.status(), default_message);
        }

        #[ink::test]
        fn set_message_works() {
            let message_1 = String::from("gm ink!");
            let mut greeter = Stator::new(message_1.clone());
            assert_eq!(stator.status(), message_1);
            let message_2 = String::from("gn");
            greeter.set_message(message_2.clone());
            assert_eq!(stator.status(), message_2);
        }
    }
}
