use winit::event::ElementState;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonState {
    Pressed,
    Released,
}

impl From<ElementState> for ButtonState {
    fn from(state: ElementState) -> ButtonState {
        match state {
            ElementState::Pressed => ButtonState::Pressed,
            ElementState::Released => ButtonState::Released,
        }
    }
}

pub struct Buttons {
    signal: Vec<bool>,
    current: Vec<ButtonState>,
    before: Vec<ButtonState>,
    number: usize,
}

impl Buttons {
    pub fn new(number: usize) -> Self {
        Self {
            signal: vec![false; number],
            current: vec![ButtonState::Released; number],
            before: vec![ButtonState::Released; number],
            number: number,
        }
    }

    pub(crate) fn handle_input(&mut self, index: impl Into<usize> + Copy, state: impl Into<ButtonState>) {
        let index = index.into();
        if index >= self.number {
            self.number = index + 1;
            self.signal.resize(self.number, false);
            self.current.resize(self.number, ButtonState::Released);
            self.before.resize(self.number, ButtonState::Released);
        }
        self.signal[index] = true;
        self.current[index] = state.into();
    }

    pub fn is_signaled(&self, index: impl Into<usize> + Copy) -> bool {
        let index = index.into();
        if index >= self.number {
            return false;
        }
        self.signal[index]
    }

    pub fn are_signaled(&self, indexes: &[impl Into<usize> + Copy]) -> bool {
        match indexes.last() {
            Some(index) if self.number > Into::<usize>::into(*index) => indexes
                .iter()
                .all(|index| self.signal[Into::<usize>::into(*index)]),
            _ => false,
        }
    }

    pub fn is_pressed(&self, index: impl Into<usize> + Copy) -> bool {
        let index = index.into();
        if index >= self.number {
            return false;
        }
        self.current[index] == ButtonState::Pressed
    }

    pub fn are_pressed(&self, indexes: &[impl Into<usize> + Copy]) -> bool {
        match indexes.last() {
            Some(index) if self.number > Into::<usize>::into(*index) => indexes
                .iter()
                .all(|index| self.current[Into::<usize>::into(*index)] == ButtonState::Pressed),
            _ => false,
        }
    }

    pub fn is_released(&self, index: impl Into<usize> + Copy) -> bool {
        let index = index.into();
        if index >= self.number {
            return false;
        }
        self.current[index] == ButtonState::Released
    }

    pub fn are_released(&self, indexes: &[impl Into<usize> + Copy]) -> bool {
        match indexes.last() {
            Some(index) if self.number > Into::<usize>::into(*index) => indexes
                .iter()
                .all(|index| self.current[Into::<usize>::into(*index)] == ButtonState::Released),
            _ => false,
        }
    }

    pub fn is_just_pressed(&self, index: impl Into<usize> + Copy) -> bool {
        let index = index.into();
        if index >= self.number {
            return false;
        }
        self.current[index] == ButtonState::Pressed && self.before[index] == ButtonState::Released
    }

    pub fn are_just_pressed(&self, indexes: &[impl Into<usize> + Copy]) -> bool {
        match indexes.last() {
            Some(index) if self.number > Into::<usize>::into(*index) => {
                indexes.iter().all(|index| {
                    let index = Into::<usize>::into(*index);
                    self.current[index] == ButtonState::Pressed
                        && self.before[index] == ButtonState::Released
                })
            }
            _ => false,
        }
    }

    pub fn is_just_released(&self, index: impl Into<usize> + Copy) -> bool {
        let index = index.into();
        if index >= self.number {
            return false;
        }
        self.current[index] == ButtonState::Released && self.before[index] == ButtonState::Pressed
    }

    pub fn are_just_released(&self, indexes: &[impl Into<usize> + Copy]) -> bool {
        match indexes.last() {
            Some(index) if self.number > Into::<usize>::into(*index) => {
                indexes.iter().all(|index| {
                    let index = Into::<usize>::into(*index);
                    self.current[index] == ButtonState::Released
                        && self.before[index] == ButtonState::Pressed
                })
            }
            _ => false,
        }
    }
}

impl Buttons {
    ///Updates buttons states before polling events
    pub(crate) fn pre_update(&mut self) {
        self.signal = vec![false; self.number];
        self.before.copy_from_slice(&self.current);
    }
}
