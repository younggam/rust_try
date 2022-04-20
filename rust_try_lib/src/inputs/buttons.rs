use std::marker::PhantomData;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElementState {
    Pressed,
    Released,
}

pub struct Buttons<T: Into<usize> + Copy, const N: usize> {
    signal: Vec<bool>,
    current: Vec<ElementState>,
    before: Vec<ElementState>,
    number: usize,

    phantom: PhantomData<T>,
}

impl<T: Into<usize> + Copy, const N: usize> Buttons<T, N> {
    pub fn new() -> Self {
        Self {
            signal: Vec::with_capacity(N),
            current: Vec::with_capacity(N),
            before: Vec::with_capacity(N),
            number: N,

            phantom: PhantomData,
        }
    }

    pub fn is_signaled(&self, index: T) -> bool {
        let index = index.into();
        if index > self.number {
            return false;
        }
        self.signal[index]
    }

    pub fn are_signaled(&self, indexes: &[T]) -> bool {
        match indexes.last() {
            Some(index) if self.number >= Into::<usize>::into(*index) => indexes
                .iter()
                .all(|index| self.signal[Into::<usize>::into(*index)]),
            _ => false,
        }
    }

    pub fn is_pressed(&self, index: T) -> bool {
        let index = index.into();
        if index > self.number {
            return false;
        }
        self.current[index] == ElementState::Pressed
    }

    pub fn are_pressed(&self, indexes: &[T]) -> bool {
        match indexes.last() {
            Some(index) if self.number >= Into::<usize>::into(*index) => indexes
                .iter()
                .all(|index| self.current[Into::<usize>::into(*index)] == ElementState::Pressed),
            _ => false,
        }
    }

    pub fn is_released(&self, index: T) -> bool {
        let index = index.into();
        if index > self.number {
            return false;
        }
        self.current[index] == ElementState::Released
    }

    pub fn are_released(&self, indexes: &[T]) -> bool {
        match indexes.last() {
            Some(index) if self.number >= Into::<usize>::into(*index) => indexes
                .iter()
                .all(|index| self.current[Into::<usize>::into(*index)] == ElementState::Released),
            _ => false,
        }
    }

    pub fn is_just_pressed(&self, index: T) -> bool {
        let index = index.into();
        if index > self.number {
            return false;
        }
        self.current[index] == ElementState::Pressed && self.before[index] == ElementState::Released
    }

    pub fn are_just_pressed(&self, indexes: &[T]) -> bool {
        match indexes.last() {
            Some(index) if self.number >= Into::<usize>::into(*index) => {
                indexes.iter().all(|index| {
                    let index = Into::<usize>::into(*index);
                    self.current[index] == ElementState::Pressed
                        && self.before[index] == ElementState::Released
                })
            }
            _ => false,
        }
    }

    pub fn is_just_released(&self, index: T) -> bool {
        let index = index.into();
        if index > self.number {
            return false;
        }
        self.current[index] == ElementState::Released && self.before[index] == ElementState::Pressed
    }

    pub fn are_just_released(&self, indexes: &[T]) -> bool {
        match indexes.last() {
            Some(index) if self.number >= Into::<usize>::into(*index) => {
                indexes.iter().all(|index| {
                    let index = Into::<usize>::into(*index);
                    self.current[index] == ElementState::Released
                        && self.before[index] == ElementState::Pressed
                })
            }
            _ => false,
        }
    }
}
