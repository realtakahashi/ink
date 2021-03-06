use ink_lang as ink;

#[ink::contract(version = "0.1.0")]
mod noop {
    #[ink(storage)]
    #[ink(event)] // We cannot have #[ink(event)] if we already have #[ink(storage)]
    struct Noop {}

    impl Noop {
        #[ink(constructor)]
        fn new(&mut self) {}

        #[ink(message)]
        fn noop(&self) {}
    }
}

fn main() {}
