use std::any::Any;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

///Implementor treated as Event. It will be distinguished only with its type.
pub trait IsEvent {}

impl PartialEq for dyn IsEvent {
    fn eq(&self, other: &Self) -> bool {
        self.type_id() == other.type_id()
    }
}

impl Hash for dyn IsEvent {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.type_id().hash(state);
    }
}

impl Eq for dyn IsEvent {}

///Literally registers events and behaviors
///Events should be impelement IsEvent
pub struct EventRegistry {
    events: HashMap<Box<dyn IsEvent>, Vec<fn(&dyn IsEvent)>>,
}

impl EventRegistry {
    pub fn new() -> Self {
        Self {
            events: HashMap::<Box<dyn IsEvent>, Vec<fn(&dyn IsEvent)>>::new(),
        }
    }

    ///put event
    pub fn on<E: 'static + IsEvent>(&mut self, key: E, value: fn(&dyn IsEvent)) {
        let key = Box::new(key) as Box<dyn IsEvent>;
        if self.events.contains_key(&key) {
            self.events.get_mut(&key).unwrap().push(value);
        } else {
            self.events.insert(key, vec![value]);
        }
    }

    pub fn fire<E: 'static + IsEvent>(&self, key: E) {
        let key = Box::new(key) as Box<dyn IsEvent>;
        if let Some(val) = self.events.get(&key) {
            val.iter().for_each(|f| f(key.as_ref()));
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
        event_reg.on(TestEvent { a: 0 }, |_| {});
        let (key, val) = event_reg
            .events
            .get_key_value(&(Box::new(TestEvent { a: 1 }) as Box<dyn IsEvent>))
            .unwrap();
        val.iter().for_each(|f| f(key.as_ref()));
    }

    #[test]
    fn is_custom_works() {
        let mut event_reg = EventRegistry::new();
        assert!(event_reg
            .events
            .insert(
                Box::new(TestEvent { a: 0 }) as Box<dyn IsEvent>,
                Vec::<fn(&dyn IsEvent)>::new()
            )
            .is_none());
        assert!(event_reg
            .events
            .insert(
                Box::new(TestEvent { a: 1 }) as Box<dyn IsEvent>,
                Vec::<fn(&dyn IsEvent)>::new()
            )
            .is_some());
    }
}
