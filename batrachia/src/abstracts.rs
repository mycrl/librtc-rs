use std::cell::UnsafeCell;

#[derive(Debug)]
pub struct SafeCell<T> {
    data: UnsafeCell<T>
}

impl<T> SafeCell<T> {
    pub fn new(value: T) -> Self {
        Self { data: UnsafeCell::new(value) }
    }

    pub fn get_mut(&self) -> &mut T {
        unsafe { &mut *self.data.get() }
    }

    pub fn get(&self) -> &T {
        unsafe { &mut *self.data.get() }
    }
}

pub struct UintMemHeap<T> {
    data: SafeCell<Option<*mut T>>
}

impl<T> Default for UintMemHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> std::fmt::Debug for UintMemHeap<T> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl<T> UintMemHeap<T> {
    pub fn new() -> Self {
        Self { data: SafeCell::new(None) }
    }

    pub fn set(&self, value: T) -> *mut T {
        let value = Box::into_raw(Box::new(value));
        let _ = self.data.get_mut().insert(value);
        value
    }

    pub fn get(&self) -> &Option<*mut T> {
        self.data.get()
    }
}

impl<T> Drop for UintMemHeap<T> {
    fn drop(&mut self) {
        if let Some(ref_value) = self.data.get() {
            let _ = unsafe { Box::from_raw(*ref_value) };
        }
    }
}
