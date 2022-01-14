//!Various wrappers for various purposes

use std::any::Any;
use std::ops::{Deref, DerefMut};

#[repr(transparent)]
struct LazyInner<T>(Option<T>);

impl<T> LazyInner<T> {
    const fn new() -> Self {
        Self(None)
    }

    const fn get(&self) -> &Option<T> {
        &self.0
    }

    fn get_mut(&mut self) -> &mut Option<T> {
        &mut self.0
    }

    const fn get_ptr_mut(&self) -> *mut Option<T> {
        self as *const Self as *const Option<T> as *mut Option<T>
    }

    fn replace(&self, value: T) -> Option<T> {
        std::mem::replace(unsafe { &mut *self.get_ptr_mut() }, Some(value))
    }
}

///This wrapper assumes that value has ever initialized before using it.
#[repr(transparent)]
pub struct LazyManual<T>(LazyInner<T>);

impl<T> LazyManual<T> {
    pub const fn new() -> Self {
        Self(LazyInner::new())
    }

    const fn inited(&self) -> bool {
        self.0.get().is_some()
    }

    /**initializes value
    \n
    \ndoes nothing when value has already initialized*/
    pub fn init(&self, item: T) {
        if self.inited() {
            return;
        }

        self.0.replace(item);
    }

    ///Assumes that value has initialized before use.
    pub fn get(&self) -> &T {
        self.0.get().as_ref().unwrap()
    }

    ///Same as get
    pub fn get_mut(&mut self) -> &mut T {
        self.0.get_mut().as_mut().unwrap()
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
pub struct UnsafeRef<T>(*mut T);

impl<T> UnsafeRef<T> {
    pub fn new(item: &T) -> Self {
        Self(item as *const T as *mut T)
    }

    ///Assumes that value has initialized before use.
    pub fn get(&self) -> &T {
        unsafe { &*self.0 }
    }

    ///Same as get
    pub fn get_mut(&mut self) -> &mut T {
        unsafe { &mut *self.0 }
    }
}

impl<T> Deref for UnsafeRef<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T> DerefMut for UnsafeRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
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
