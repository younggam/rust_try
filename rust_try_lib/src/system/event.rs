use std::collections::HashMap;
use std::hash::{Hash, Hasher};

///Implementor should hash its type_id if you want distinguish event based on type not field values.
pub trait IsEvent: Eq + Hash {}

pub struct EventRegistry<T: IsEvent> {
    events: HashMap<T, Vec<fn(&T)>>,
}

impl<T: IsEvent> EventRegistry<T> {
    pub fn new() -> Self {
        Self {
            events: HashMap::<T, Vec<fn(&T)>>::new(),
        }
    }

    pub fn on(&mut self, key: T, value: fn(&T)) {
        if self.events.contains_key(&key) {
            self.events.get_mut(&key).unwrap().push(value);
        } else {
            self.events.insert(key, vec![value]);
        }
    }

    pub fn fire(&self, key: T) {}
}

#[derive(Eq)]
pub enum TestEvent {
    OnStart,
    Cont(i8),
    OnQuit,
}

impl PartialEq for TestEvent {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Cont(_), Self::Cont(_)) => true,
            (Self::OnStart, Self::OnStart) => true,
            (Self::OnQuit, Self::OnQuit) => true,
            _ => false,
        }
    }
}

impl Hash for TestEvent {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::OnStart => state.write_i32(Self::OnStart),
            _ => {}
        }
    }
}

impl IsEvent for TestEvent {}

#[test]
fn it_works() {
    let mut event_reg = EventRegistry::<TestEvent>::new();
    assert!(event_reg
        .events
        .insert(TestEvent::OnStart, Vec::<fn(&TestEvent)>::new())
        .is_none());
    event_reg
        .events
        .get_mut(&TestEvent::OnStart)
        .unwrap()
        .push(|e| {});
    let (key, val) = event_reg.events.get_key_value(&TestEvent::OnStart).unwrap();
    val.iter().for_each(|f| f(key));
}

#[test]
fn is_custom_works() {
    let mut event_reg = EventRegistry::<TestEvent>::new();
    assert!(TestEvent::Cont(1) == TestEvent::Cont(2));
    assert!(event_reg
        .events
        .insert(TestEvent::Cont(1), Vec::<fn(&TestEvent)>::new())
        .is_none());
    assert!(event_reg
        .events
        .insert(TestEvent::Cont(2), Vec::<fn(&TestEvent)>::new())
        .is_none());
}
