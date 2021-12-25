use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

///Implementor treated as Event. It will be distinguished only with its type.
pub trait IsEvent: Any {}

pub struct EventListener {
    raw_listener: *const fn(&dyn IsEvent),
}

impl EventListener {
    pub fn new<E: 'static + IsEvent>(listener: fn(&E)) -> Self {
        Self {
            raw_listener: &listener as *const fn(&E),
        }
    }

    pub fn downcast<E: 'static + IsEvent>(&self) -> fn(&E) {
        println!("a");
        let a = unsafe { *self.raw_listener };
        println!("b");
        a
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

    pub fn register<E: 'static + IsEvent>(&mut self, event: E, listener: fn(&E)) {
        let event = event.type_id();
        let listener = EventListener::new::<E>(listener);

        if self.events.contains_key(&event) {
            self.events.get_mut(&event).unwrap().push(listener);
        } else {
            self.events.insert(event, vec![listener]);
        }
    }

    pub fn fire<E: 'static + IsEvent>(&self, event: E) {
        if let Some(listeners) = self.events.get(&event.type_id()) {
            listeners.iter().for_each(|f| f.downcast::<E>()(&event));
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
