use std::{
    mem::ManuallyDrop,
    cell::UnsafeCell,
};

/// The type wrapper for interior mutability in rust.
#[derive(Debug)]
pub struct PointerCell<T> {
    data: UnsafeCell<T>,
}

impl<T> PointerCell<T> {
    /// Constructs a new instance of UnsafeCell which will wrap the
    /// specified value.
    pub fn new(value: T) -> Self {
        Self {
            data: UnsafeCell::new(value),
        }
    }

    /// safe get mutability ref for inner value.
    pub fn get_mut(&self) -> &mut T {
        unsafe { &mut *self.data.get() }
    }

    /// safe get ref for inner value.
    pub fn get(&self) -> &T {
        unsafe { &mut *self.data.get() }
    }
}

/// A wrapper type to construct uninitialized instances of T.
/// inner manage auto drop.
pub struct HeapPointer<T> {
    data: PointerCell<Option<*mut T>>,
}

impl<T> Default for HeapPointer<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> std::fmt::Debug for HeapPointer<T> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl<T> HeapPointer<T> {
    /// Creates a new MaybeUninit<T> initialized with the given value.
    pub fn new() -> Self {
        Self {
            data: PointerCell::new(None),
        }
    }

    /// Sets the value of the HeapPointer<T>.
    ///
    /// This overwrites any previous value without dropping it,
    /// so be careful not to use this twice unless you want to skip
    /// running the destructor. For your convenience, this also
    /// returns a mutable reference to the (now safely initialized)
    /// contents of self.
    pub fn set(&self, value: T) -> *mut T {
        let value = Box::into_raw(Box::new(value));
        let _ = self.data.get_mut().insert(value);
        value
    }

    /// get inner type T the raw ptr.
    pub fn get(&self) -> &Option<*mut T> {
        self.data.get()
    }
}

impl<T> Drop for HeapPointer<T> {
    fn drop(&mut self) {
        if let Some(ref_value) = self.data.get() {
            let _ = unsafe { Box::from_raw(*ref_value) };
        }
    }
}

pub(crate) trait ArrayExt<T> {
    fn into_c_layout(self) -> (*mut T, usize, usize);
}

impl<T> ArrayExt<T> for Vec<T> {
    /// ```no_run
    /// let vec = vec![0u8; 10];
    /// let (ptr, size, capacity) = vec.into_c_layout();
    /// assert!(ptr.is_null());
    /// assert_eq!(capacity, 10);
    /// assert_eq!(size, 10);
    /// ```
    fn into_c_layout(self) -> (*mut T, usize, usize) {
        let mut me = ManuallyDrop::new(self);
        (me.as_mut_ptr(), me.len(), me.capacity())
    }
}
