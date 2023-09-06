use std::ops::Deref;

pub struct MyRc<T> {
    value: *mut T,
    strong_count: *mut usize,
}

impl<T> MyRc<T> {
    pub fn new(value: T) -> Self {
        let value = Box::new(value);
        let strong_count = Box::new(1);

        Self {
            value: Box::into_raw(value),
            strong_count: Box::into_raw(strong_count),
        }
    }

    pub fn strong_count(&self) -> usize {
        unsafe { *self.strong_count }
    }
}

impl<T> Deref for MyRc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.value }
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        unsafe {
            *self.strong_count -= 1;
        }
    }
}

impl<T> Clone for MyRc<T> {
    fn clone(&self) -> Self {
        unsafe {
            *self.strong_count += 1;
        }
        Self { ..*self }
    }
}
