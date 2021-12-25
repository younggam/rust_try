use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};

///Implementor treated as Event. It will be distinguished only with its type.
pub trait IsEvent: Any {}

pub struct Listener<E: 'static + IsEvent> {
    listener: Box<dyn FnMut(&E)>,
}

impl<E: 'static + IsEvent> Listener<E> {
    pub fn new<F: 'static + FnMut(&E)>(listener: F) -> Self {
        Self {
            listener: Box::new(listener),
        }
    }
}

impl<E: 'static + IsEvent> Deref for Listener<E> {
    type Target = dyn FnMut(&E);

    fn deref(&self) -> &Self::Target {
        &self.listener
    }
}

impl<E: 'static + IsEvent> DerefMut for Listener<E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.listener
    }
}

pub struct EventListener {
    boxed_listener: Box<dyn Any>,
}

impl EventListener {
    pub fn new<E: 'static + IsEvent, F: 'static + FnMut(&E)>(listener: F) -> Self {
        Self {
            boxed_listener: Box::new(Listener::new(listener)),
        }
    }

    pub fn downcast<E: 'static + IsEvent>(&mut self) -> &mut Listener<E> {
        (&mut self.boxed_listener)
            .downcast_mut::<Listener<E>>()
            .unwrap()
    }
}

///Literally registers events and behaviors
///Events should be impelement IsEvent
pub struct EventRegistry {
    pub events: HashMap<TypeId, Vec<EventListener>>,
}

impl EventRegistry {
    pub fn new() -> Self {
        Self {
            events: HashMap::new(),
        }
    }

    pub fn register<E: 'static + IsEvent, F: 'static + FnMut(&E)>(
        &mut self,
        event: E,
        listener: F,
    ) {
        let event = event.type_id();
        let listener = EventListener::new::<E, F>(listener);

        if self.events.contains_key(&event) {
            self.events.get_mut(&event).unwrap().push(listener);
        } else {
            self.events.insert(event, vec![listener]);
        }
    }

    pub fn fire<E: 'static + IsEvent>(&mut self, event: E) {
        if let Some(listeners) = self.events.get_mut(&event.type_id()) {
            listeners.iter_mut().for_each(|f| f.downcast::<E>()(&event));
        }
    }
}

#[cfg(test)]
mod test_event {
    use super::*;

    struct TestEvent {
        a: isize,
    }

    impl TestEvent {
        fn a(&mut self) {
            self.a += 1;
        }
    }

    impl IsEvent for TestEvent {}

    #[test]
    fn it_works() {
        let mut event_reg = EventRegistry::new();
        event_reg.register(TestEvent { a: 0 }, |_| {});
        event_reg.fire(TestEvent { a: 1 });
    }

    #[test]
    fn is_custom_works() {
        let mut event_reg = EventRegistry::new();
        assert!(event_reg
            .events
            .insert(TestEvent { a: 0 }.type_id(), Vec::<EventListener>::new())
            .is_none());
        assert!(event_reg
            .events
            .insert(TestEvent { a: 1 }.type_id(), Vec::<EventListener>::new())
            .is_some());
    }
}
