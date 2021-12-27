//!Generalize closures
/*!I can't understand why.
If reference become generics that for parameters of closure,
the closure wants argument live longer than closure itself.
When reference explicitly set as parameters of closure,
now the closure doesn't care how long argmuent lives.
*/

use std::any::Any;
use std::marker::PhantomData;

///Generalize Closure that takes 1 argmuent.
///Resposible of choosing appropriate method is on caller
pub trait Closure<A, R> {
    type Cons: FnOnce(A) -> R + ?Sized;

    fn call(&self, a: A) -> R;

    fn call_mut(&mut self, a: A) -> R;

    fn call_once(self, a: A) -> R;
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

    fn call(&self, _a: &mut A) {}

    fn call_mut(&mut self, a: &mut A) {
        (self.cons)(a)
    }

    fn call_once(mut self, a: &mut A) {
        (*self.cons)(a)
    }
}

pub struct Cons<A, F: 'static + FnOnce(A)> {
    cons: Box<dyn Any>,
    p1: PhantomData<A>,
    p2: PhantomData<F>,
}

impl<A, F: 'static + FnOnce(A)> Cons<A, F> {
    pub fn new(cons: F) -> Self {
        Self {
            cons: Box::new(cons),
            p1: PhantomData,
            p2: PhantomData,
        }
    }

    pub fn get(&self) -> &F {
        self.cons.downcast_ref::<F>().unwrap()
    }

    pub fn get_mut(&mut self) -> &mut F {
        self.cons.downcast_mut::<F>().unwrap()
    }

    pub fn get_once(self) -> F {
        *self.cons.downcast::<F>().unwrap()
    }
}

#[cfg(test)]
mod test {
    use std::any::Any;

    use super::*;

    struct Test;
    #[test]
    fn is_unbox_takes_ownership() {
        let a = Box::new(Test {});
        let b = &*a;
        a.as_ref();
        b.type_id();
    }

    #[test]
    fn closure_ref() {
        let x = Test {};
        let mut asdf = Test {};
        let a = move |_h: &mut Test| {
            let _y = &x;
        };
        let mut b = Cons::new(a);
        b.get()(&mut asdf);
        b.get()(&mut asdf);
        b.get_mut()(&mut asdf);
        b.get_once()(&mut asdf);
    }

    #[test]
    fn closure_mut() {
        let mut x = Test {};
        let mut asdf = Test {};
        let a = move |_h: &mut Test| {
            let _y = &mut x;
        };
        let mut b = Cons::new(a);
        b.get_mut()(&mut asdf);
        b.get_mut()(&mut asdf);
    }

    #[test]
    fn closure_once() {
        let x = Test {};
        let mut asdf = Test {};
        let a = move |_h: &mut Test| {
            let _y = x;
        };
        let b = Cons::new(a);
        b.get_once()(&mut asdf);
    }
}
