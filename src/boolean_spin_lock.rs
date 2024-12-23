use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Release;
use std::sync::atomic::Ordering::Acquire;

// This is a tiny spin lock implementation,
// should only be used between threads if the
// threads halt for a tiny bit of time while
// waiting for some stuff to happen as we loop
// until we get the state of the lock to be 
// unlocked
#[derive(Debug)]
pub struct SpinLockAtomic {
    // True- -> locked, false -> unlocked
    pub locked: AtomicBool
}


impl SpinLockAtomic {
    pub fn new() -> Self {
        SpinLockAtomic {locked: AtomicBool::new(false)}
    }

    pub fn unlock(&self) {
        self.locked.store(false, Release);
    }

    pub fn lock(&self) {
        // Returs the value that is already present and stores true in it afterwards
        while self.locked.swap(true, Acquire) {
            std::hint::spin_loop();
        }
    }
}

mod tests {
    use std::sync::atomic::Ordering::Relaxed;
    use crate::boolean_spin_lock::SpinLockAtomic;

    #[test]
    fn try_simple_spin_lock() {
        let lock = SpinLockAtomic::new();
        lock.lock();
        assert_eq!(true, lock.locked.load(Relaxed));
        lock.unlock();
        assert_eq!(false, lock.locked.into_inner());
    }
}