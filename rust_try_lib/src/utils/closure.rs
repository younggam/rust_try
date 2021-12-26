//!Generalize closures
/*!I can't understand why.
If reference become generics that for parameters of closure,
the closure wants argument live longer than closure itself.
When reference explicitly set as parameters of closure,
now the closure doesn't care how long argmuent lives.
*/

///Generalize Closure that takes 1 argmuent.
///Resposible of choosing appropriate method is on caller
pub trait Closure<A, R> {
    type Cons: FnOnce(A) -> R + ?Sized;

    fn get(&self) -> &Self::Cons;

    fn get_mut(&mut self) -> &mut Self::Cons;

    fn take(self) -> *mut Self::Cons;
}

pub struct MutCapProv<R> {
    cons: Box<dyn FnMut() -> R>,
}

impl<T> MutCapProv<T> {}

///Takes 1 mutable reference as argument and no return value
pub struct MutCapMutCons<A: 'static> {
    cons: Box<dyn FnMut(&mut A)>,
}

impl<A> MutCapMutCons<A> {
    pub fn new<F: 'static + FnMut(&mut A)>(cons: F) -> Self {
        Self {
            cons: Box::new(cons),
        }
    }
}

impl<A> Closure<&mut A, ()> for MutCapMutCons<A> {
    type Cons = dyn FnMut(&mut A);

    fn get(&self) -> &Self::Cons {
        &self.cons
    }

    fn get_mut(&mut self) -> &mut Self::Cons {
        &mut self.cons
    }

    fn take(self) -> *mut Self::Cons {
        Box::into_raw(self.cons)
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
