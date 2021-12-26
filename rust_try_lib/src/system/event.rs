use std::any::{Any, TypeId};
use std::collections::HashMap;

use crate::utils::{BoxedAny, ConsMut};

///Literally registers events and listeners
///Events should be impelement IsEvent
pub struct EventRegistry {
    pub events: HashMap<TypeId, Vec<BoxedAny>>,
}

impl EventRegistry {
    pub fn new() -> Self {
        Self {
            events: HashMap::new(),
        }
    }

    pub fn register<E: Any, F: 'static + FnMut(&mut E)>(&mut self, event: TypeId, listener: F) {
        let listener = BoxedAny::new(ConsMut::new(listener));

        if let Some(vec) = self.events.get_mut(&event) {
            vec.push(listener);
        } else {
            self.events.insert(event, vec![listener]);
        }
    }

    pub fn fire<E: Any>(&mut self, mut event: E) {
        if let Some(listeners) = self.events.get_mut(&event.type_id()) {
            listeners
                .iter_mut()
                .for_each(|f| f.downcast_mut::<ConsMut<E>>().unwrap()(&mut event));
        }
    }
}

#[cfg(test)]
mod test_event {
    use super::*;

    struct TestEvent {
        a: isize,
    }

    struct HiEvent;

    impl TestEvent {
        fn a(&mut self) {
            self.a += 1;
        }
    }

    #[test]
    fn it_works() {
        let mut event_reg = EventRegistry::new();
        event_reg.register(TypeId::of::<TestEvent>(), TestEvent::a);
        event_reg.fire(TestEvent { a: 1 });
    }

    #[test]
    fn is_others_works() {
        let mut event_reg = EventRegistry::new();
        assert!(event_reg
            .events
            .insert(TypeId::of::<TestEvent>(), Vec::<BoxedAny>::new())
            .is_none());
        assert!(event_reg
            .events
            .insert(TypeId::of::<TestEvent>(), Vec::<BoxedAny>::new())
            .is_some());
        assert!(event_reg
            .events
            .insert(TypeId::of::<HiEvent>(), Vec::<BoxedAny>::new())
            .is_none());
    }
}
