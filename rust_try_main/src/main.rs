use rust_try_lib::application::*;

use modules::core::CoreModule;

fn main() {
    let app = ApplicationWinit::new(CoreModule {});
    app.run();
}

mod modules {
    pub mod core;
    // pub mod renderer;
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
