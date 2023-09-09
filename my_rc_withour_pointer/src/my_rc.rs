use std::{ops::Deref, ptr::NonNull};

static mut SHARED_COUNT: usize = 1;

pub struct MyRc<T: Clone> {
    value: T,
    strong_count: NonNull<usize>,
}

impl<T> MyRc<T>
where
    T: Clone,
{
    pub fn new(value: T) -> Self {
        Self {
            value,
            strong_count: unsafe { NonNull::new(&mut SHARED_COUNT as *mut usize).unwrap() },
        }
    }

    pub fn strong_count(&self) -> usize {
        unsafe { *self.strong_count.as_ptr() }
        // *self.strong_count.borrow()
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
        // *self.strong_count.borrow_mut() += 1;
        unsafe {
            *self.strong_count.as_ptr() += 1;
        }
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
        unsafe {
            *self.strong_count.as_ptr() -= 1;
        }
        // *self.strong_count.borrow_mut() -= 1;
    }
}
