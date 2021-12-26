//!Generalize closures

use std::ops::{Deref, DerefMut};

///Takes 1 mutable reference as argument and no return value
pub struct ConsMut<T: 'static> {
    cons: Box<dyn FnMut(&mut T)>,
}

impl<T> ConsMut<T> {
    pub fn new<F: 'static + FnMut(&mut T)>(cons: F) -> Self {
        Self {
            cons: Box::new(cons),
        }
    }
}

impl<T> Deref for ConsMut<T> {
    type Target = dyn FnMut(&mut T);

    fn deref(&self) -> &Self::Target {
        &self.cons
    }
}

impl<T> DerefMut for ConsMut<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cons
    }
}
