//!Various wrappers for various purposes

use std::any::Any;
use std::ops::{Deref, DerefMut};

///This wrapper assumes that value won't be used more than twice except reference.
pub struct Once<T>(Option<T>);

impl<T> Once<T> {
    pub const fn new(item: T) -> Self {
        Self(Some(item))
    }

    /*pub fn consume_check(&mut self) -> Result<T, bool> {
        match self.0.take() {
            Some(item) => Ok(item),
            None => Err(false),
        }
    }*/

    ///Assumes value has never used, and takes ownership of value.
    pub fn consume(&mut self) -> T {
        match self.0.take() {
            Some(item) => item,
            None => unreachable!("It has already consumed!"),
        }
    }
}

//

// pub struct Lazy<T, F = fn() -> T> {
//     inner: Option<T>,
//     initializer: Option<F>,
// }

///This wrapper assumes that value has ever initialized before using it.
pub struct LazyManual<T>(Option<T>);

impl<T> LazyManual<T> {
    pub const fn new() -> Self {
        Self(None)
    }

    /**initializes value
    \n
    \ndoes nothing when value has already initialized*/
    pub fn init(&mut self, item: T) {
        if let None = self.0 {
            self.0 = Some(item);
        }
    }

    ///Assumes that value has initialized before use.
    pub fn get(&self) -> &T {
        match self.0 {
            None => unreachable!("It has never initialized"),
            Some(ref item) => item,
        }
    }

    ///Same as get
    pub fn get_mut(&mut self) -> &mut T {
        match self.0 {
            None => unreachable!("It has never initialized"),
            Some(ref mut item) => item,
        }
    }
}

impl<T> Deref for LazyManual<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T> DerefMut for LazyManual<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

//

///This wrapper provides kinda wildcard type.
pub struct BoxedAny {
    boxed: Box<dyn Any + Send>,
}

impl BoxedAny {
    pub fn new<T: Any + Send>(x: T) -> Self {
        Self { boxed: Box::new(x) }
    }

    ///consumes itself and release orginal object
    pub fn downcast<T: Any>(self) -> Option<T> {
        match self.boxed.downcast::<T>() {
            Ok(inner) => Some(*inner),
            _ => None,
        }
    }

    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        (&self.boxed).downcast_ref::<T>()
    }

    pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        (&mut self.boxed).downcast_mut::<T>()
    }
}
