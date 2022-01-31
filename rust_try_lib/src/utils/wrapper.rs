//!Various wrappers for various purposes

use std::any::Any;
use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::Once;

///This wrapper assumes that value has ever initialized before using it.
pub struct LazyManual<T>(UnsafeCell<Option<T>>, Once, Once);

impl<T> LazyManual<T> {
    pub const fn new() -> Self {
        Self(UnsafeCell::new(None), Once::new(), Once::new())
    }

    ///initializes value. Does nothing when value has already initialized
    pub fn init(&self, item: T) {
        self.1.call_once(|| {
            unsafe { *self.0.get() = Some(item) };
        })
    }

    pub fn fin(&self) {
        self.2.call_once(|| {
            unsafe { *self.0.get() = None };
        })
    }

    ///Assumes that value has initialized before use.
    pub fn get(&self) -> &T {
        unsafe {
            match &*self.0.get() {
                Some(ref item) => item,
                None => {
                    debug_assert!(false, "Attempted to use unitialized or dropped lazy value.");
                    std::hint::unreachable_unchecked()
                }
            }
        }
    }

    ///Same as get
    pub fn get_mut(&mut self) -> &mut T {
        unsafe {
            match &mut *self.0.get() {
                Some(ref mut item) => item,
                None => {
                    debug_assert!(false, "Attempted to use unitialized or dropped lazy value.");
                    std::hint::unreachable_unchecked()
                }
            }
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

///Doesn't leak pointer access when sent to other thread
unsafe impl<T: Send> Send for LazyManual<T> {}
///Has inner mutability, so rigidly this is not Sync. Use init only in main thread.
unsafe impl<T: Sync> Sync for LazyManual<T> {}

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

///ignores lifetime of reference
#[repr(transparent)]
pub struct UnsafeRef<T>(*const T);

impl<T> UnsafeRef<T> {
    pub fn new(item: &T) -> Self {
        Self(item as *const T)
    }

    ///Assumes that value has initialized before use.
    pub fn get(&self) -> &T {
        unsafe { &*self.0 }
    }
}

impl<T> Deref for UnsafeRef<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

///Doesn't leak pointer access when sent to other thread
unsafe impl<T: Sync> Sync for UnsafeRef<T> {}
///No inner mutabiliy.
unsafe impl<T: Send> Send for UnsafeRef<T> {}

#[cfg(test)]
mod test {
    use super::*;
    struct Test;
    impl Test {
        fn a(&self) {
            10;
        }
    }
    #[test]
    fn what() {
        let a = Test {};
        let b = UnsafeRef::new(&a);
        let c = b.get();
        c.a();
    }
}
