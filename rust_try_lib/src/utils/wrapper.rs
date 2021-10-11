mod once {
    enum OnceInner<T> {
        Consumable(T),
        Consumed,
    }

    use OnceInner::*;

    pub struct Once<T>(OnceInner<T>);

    impl<T> Once<T> {
        pub fn new(item: T) -> Self {
            Self(Consumable(item))
        }

        pub fn consume_check(&mut self) -> Result<T, bool> {
            match std::mem::replace(&mut self.0, Consumed) {
                Consumable(item) => Ok(item),
                Consumed => Err(false),
            }
        }

        pub fn consume(&mut self) -> T {
            match std::mem::replace(&mut self.0, Consumed) {
                Consumable(item) => item,
                Consumed => unreachable!("It has already consumed!"),
            }
        }
    }
}
pub use once::*;
