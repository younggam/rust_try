//!Generalize closures

use std::any::Any;
use std::cell::Cell;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

///Cl
pub trait Consumer {}

///Takes 1 mutable reference as argument and no return value
pub struct MutCapCons<A> {
    cons: Box<dyn FnMut(A)>,
}

impl<A> MutCapCons<A> {
    pub fn new<F: 'static + FnMut(A)>(cons: F) -> Self {
        Self {
            cons: Box::new(cons),
        }
    }

    pub fn call_mut(&mut self, a: A) {
        (&mut self.cons)(a)
    }
}

#[cfg(test)]
mod test {
    use std::any::Any;

    use super::*;

    struct Test;
    #[test]
    fn is_unbox_takes_ownership() {
        let h = Test {};
        let a = Box::new(Test {});
        let b = &*a;
        a.as_ref();
        b.type_id();
    }
}
