use std::alloc::Layout;

pub struct MyVec<T> {
    ptr: *mut T, // you can actually convert between *const T and *mut T with the `as` keyword freely... they're just there to signal intent
    size: usize,
    capacity: usize,
}

impl<T> MyVec<T> {
    const INITIAL_CAPACITY: usize = 8;

    pub fn new() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
            size: 0,
            capacity: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        if self.size == self.capacity {
            self.expand_capacity();
        }

        // SAFETY: We just expanded the capacity if necessary so self.size is less than self.capacity
        let ptr = unsafe { self.ptr_to_elem(self.size) };

        self.size += 1;

        // dangerous, since we're writing to memory but it will drop old value but that might be uninitialised
        // unsafe { *ptr = value };

        // SAFETY: We just allocated the memory at `ptr` so it's safe to write to it
        unsafe { ptr.write(value) };
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            None
        } else {
            self.size -= 1;

            // SAFETY: We just decreased the size so it's safe to access the last element. And the size before was not zero,
            let ptr = unsafe { self.ptr_to_elem(self.size) };

            // SAFETY: same as above
            let value = unsafe { ptr.read() };

            Some(value)
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.size {
            None
        } else {
            // SAFETY: We just checked that `index` is less than `self.size`
            let ptr = unsafe { self.ptr_to_elem(index) };

            // SAFETY: Safe to borrow the value at `ptr` since it's valid for the lifetime of the reference
            let value = unsafe { &*ptr };

            Some(value)
        }
    }

    fn expand_capacity(&mut self) {
        if self.capacity == 0 {
            let layout = Layout::array::<T>(Self::INITIAL_CAPACITY).unwrap();
            // SAFETY: Layout has non-zero size
            let ptr = unsafe { std::alloc::alloc(layout) };

            self.ptr = ptr as _;
            self.capacity = Self::INITIAL_CAPACITY;
        } else {
            let new_capacity = self.capacity * 2;
            // SAFETY: We allocated `self.ptr` ourself with a layout equal to `old_layout`,
            //         and the new layout is greater than zero bytes in size,
            //         and we assume it doesn't overflow
            let new_ptr = unsafe {
                std::alloc::realloc(
                    self.ptr as *mut u8,
                    Self::layout_for(self.capacity),
                    new_capacity,
                )
            };

            self.ptr = new_ptr as _;
            self.capacity = new_capacity;
        }
    }

    fn layout_for(n_elems: usize) -> Layout {
        Layout::array::<T>(n_elems).unwrap()
    }

    /// # Safety
    ///
    /// The caller must ensure that `index` is less than `self.capacity`.
    unsafe fn ptr_to_elem(&self, index: usize) -> *mut T {
        unsafe { self.ptr.add(index) }
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        if self.capacity != 0 {
            for index in 0..self.size {
                unsafe {
                    // SAFETY: We just checked that `index` is less than `self.size`
                    let ptr = self.ptr_to_elem(index);

                    // SAFETY: We just checked that `index` is less than `self.size`
                    ptr.drop_in_place();
                }
            }

            // SAFETY: We allocated `self.ptr` ourself with a layout equal to `old_layout`
            unsafe {
                std::alloc::dealloc(self.ptr as _, Self::layout_for(self.capacity));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mut vec = MyVec::new();
        vec.push("Hello");
        vec.push("World");
        vec.push("this");
        vec.push("is");
        vec.push("comp6991!");

        for i in 0..10 {
            println!("{:?}", vec.get(i));
        }

        assert_eq!(vec.pop(), Some("comp6991!"));
        assert_eq!(vec.pop(), Some("is"));
        assert_eq!(vec.pop(), Some("this"));

        for i in 0..10 {
            println!("{:?}", vec.get(i));
        }
    }
}
