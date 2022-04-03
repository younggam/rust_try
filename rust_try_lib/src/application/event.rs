use std::any::{Any, TypeId};
use std::collections::HashMap;

use crate::{utils::BoxedAny, *};

///Literally registers events and correspond listeners.
///events can be any type but 'static.
pub struct EventRegistry {
    pub events: HashMap<TypeId, Vec<BoxedAny>>,
}

impl EventRegistry {
    #[allow(dead_code)]
    pub(crate) fn new() -> Self {
        Self {
            events: HashMap::new(),
        }
    }

    pub fn register<E, F>(&mut self, event: TypeId, listener: F)
    where
        E: 'static,
        F: 'static + FnMut(&mut E) + Send,
    {
        let listener = BoxedAny::new(box_as!(listener, dyn FnMut(&mut E) + Send));

        if let Some(vec) = self.events.get_mut(&event) {
            vec.push(listener);
        } else {
            self.events.insert(event, vec![listener]);
        }
    }

    pub fn fire<E: 'static>(&mut self, mut event: E) {
        if let Some(listeners) = self.events.get_mut(&event.type_id()) {
            listeners.iter_mut().for_each(|f| {
                f.downcast_mut::<Box<dyn FnMut(&mut E) + Send>>().unwrap()(&mut event)
            });
        }
    }

    pub fn clear_all(&mut self) {
        self.events.clear();
    }

    pub fn clear_list(&mut self, list: &[TypeId]) {
        for item in list {
            self.events.remove(item);
        }
    }

    pub fn clear_exclude(&mut self, list: &[TypeId]) {
        let mut temp = Vec::new();
        for item in list {
            if let Some(entry) = self.events.remove_entry(item) {
                temp.push(entry);
            }
        }

        self.events.clear();

        for entry in temp {
            self.events.insert(entry.0, entry.1);
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
