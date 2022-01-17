//!Various wrappers for various purposes

use std::any::Any;
use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};

///This wrapper assumes that value has ever initialized before using it.
#[repr(transparent)]
pub struct LazyManual<T>(UnsafeCell<Option<T>>);

impl<T> LazyManual<T> {
    pub const fn new() -> Self {
        Self(UnsafeCell::new(None))
    }

    fn inited(&self) -> bool {
        unsafe { &*self.0.get() }.is_some()
    }

    ///initializes value. Does nothing when value has already initialized
    ///# SAFETY
    ///don't call twice or more. Read-on-write could occur.
    pub unsafe fn init(&self, item: T) {
        //not atomic
        if self.inited() {
            return;
        }

        *self.0.get() = Some(item);
    }

    ///Assumes that value has initialized before use.
    pub fn get(&self) -> &T {
        unsafe { &*self.0.get() }
            .as_ref()
            .expect("Initialize befor use LazyManual")
    }

    ///Same as get
    pub fn get_mut(&mut self) -> &mut T {
        self.0
            .get_mut()
            .as_mut()
            .expect("Initialize befor use LazyManual")
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

//

///name
///Wrapper for types that mutiple reading while writing is ok or user doesn't write while reading.
pub struct MutOnlyOnMainThread<T>(T);

impl<T> MutOnlyOnMainThread<T> {
    pub const fn new(value: T) -> Self {
        Self(value)
    }

    ///# SAFETY
    ///Call this only once per loop and in main thread
    pub unsafe fn get_mut(&self) -> &mut T {
        &mut *(&self.0 as *const T as *mut T)
    }
}

impl<T> Deref for MutOnlyOnMainThread<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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
