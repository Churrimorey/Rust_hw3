use std::{cell::RefCell, ops::Deref};

pub struct MyRc<T: Clone> {
    value: T,
    strong_count: RefCell<usize>,
}

impl<T> MyRc<T>
where
    T: Clone,
{
    pub fn new(value: T) -> Self {
        Self {
            value,
            strong_count: RefCell::new(1),
        }
    }

    pub fn strong_count(&self) -> usize {
        *self.strong_count.borrow()
    }
}

impl<T> Deref for MyRc<T>
where
    T: Clone,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> Clone for MyRc<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        *self.strong_count.borrow_mut() += 1;
        Self {
            value: self.value.clone(),
            strong_count: self.strong_count.clone(),
        }
    }
}

impl<T> Drop for MyRc<T>
where
    T: Clone,
{
    fn drop(&mut self) {
        *self.strong_count.borrow_mut() -= 1;
    }
}
