mod once {
    ///This class assumes that value won't be used more than twice except reference.
    ///I know this is bad practice, so I left it as only for crate.
    pub(crate) struct Once<T>(Option<T>);

    impl<T> Once<T> {
        pub fn new(item: T) -> Self {
            Self(Some(item))
        }

        pub fn consume_check(&mut self) -> Result<T, bool> {
            match self.0.take() {
                Some(item) => Ok(item),
                None => Err(false),
            }
        }

        pub fn consume(&mut self) -> T {
            match self.0.take() {
                Some(item) => item,
                None => unreachable!("It has already consumed!"),
            }
        }
    }
}
pub use once::*;

mod lazy {
    ///This class assumes that value has ever initialized before using it.
    ///I know this is bad practice, so I left it as only for crate.
    pub(crate) struct LazyManual<T> {
        item: Option<T>,
        initialized: bool,
    }

    impl<T> LazyManual<T> {
        pub fn new() -> Self {
            Self {
                item: None,
                initialized: false,
            }
        }

        ///initializes value
        ///does nothing when value has already initialized
        pub fn init(&mut self, item: T) {
            if !self.initialized {
                self.item = Some(item);
            }
        }

        pub fn get(&self) -> &T {
            match self.item {
                None => unreachable!("It has never initialized"),
                Some(ref item) => item,
            }
        }

        pub fn get_mut(&mut self) -> &mut T {
            match self.item {
                None => unreachable!("It has never initialized"),
                Some(ref mut item) => item,
            }
        }
    }

    impl<T> std::ops::Deref for LazyManual<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            self.get()
        }
    }

    impl<T> std::ops::DerefMut for LazyManual<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            self.get_mut()
        }
    }
}
pub use lazy::*;
