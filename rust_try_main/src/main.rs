use rust_try_lib::application::*;

use scenes::initial::InitialScene;

fn main() {
    env_logger::init();
    let app = Application::new("Rust Try");
    let initial_scene = InitialScene::new(&app);
    app.run(initial_scene);
}

pub mod scenes {
    pub mod initial;
}

pub mod objects {
    pub mod camera;
    pub mod transform;
}

#[cfg(test)]
mod test {
    struct Test {
        bet: Bet,
    }
    struct Bet(i32);
    impl Test {
        fn a(&self) -> i32 {
            10
        }
    }

    #[test]
    fn pointer_move() {
        let a = Test { bet: Bet(1) };
        let b = &a as *const Test as *mut Test;
        let c = unsafe { &mut *b };
        b;
    }

    #[test]
    fn addr_of_move() {
        let mut a = Test { bet: Bet(1) };
        let b = std::ptr::addr_of_mut!(a);
        drop(a);
        let c = unsafe { &mut *b };
        assert_eq!(c.a(), 10);
        assert_eq!(c.bet.0, 1);
        drop(c);
    }
}
