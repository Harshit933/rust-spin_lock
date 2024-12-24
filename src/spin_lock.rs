use std::{cell::UnsafeCell, sync::atomic::AtomicBool};

use crate::guard::Guard;

pub struct SpinLock<T> {
    pub locked: AtomicBool,
    pub value: UnsafeCell<T>,
}

// UnsafeCell does not implement Sync
// Here we are telling the compiler it is safe for our
// type to be shared between threads.
unsafe impl<T> Sync for SpinLock<T> where T: Send {}

impl<T> SpinLock<T> {
    pub fn new(value: T) -> Self {
        SpinLock {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value)
        }
    }

    pub fn lock<'a>(&'a self) -> Guard<T> {
        while self.locked.swap(true, std::sync::atomic::Ordering::Acquire) {
            std::hint::spin_loop();
        }
        Guard {lock: self}
    }

    // We mark it unsafe to let user know that
    // the content we get after .lock() should 
    // not have a reference at the time calling
    // for this unlock method, Fixed by using Guard<T>

    // This method should **NOT** be used as Guard takes care 
    // of unlocking by Drop
    pub unsafe fn unlock(&self) {
        self.locked.store(false, std::sync::atomic::Ordering::Release);
    }
}