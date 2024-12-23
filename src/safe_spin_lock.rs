use std::{cell::UnsafeCell, env::var, ops::{Deref, DerefMut}};

use crate::spin_lock::SpinLock;

// To safely unlock the UnsafeSpinLock, we will
// guard it with this Guard
pub struct Guard<'a, T> {
    pub lock: &'a SpinLock<T>,
}

impl<T> Deref for Guard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        // Guarantees we've locked this lock
        unsafe {& *self.lock.value.get()}
    }
}

impl<T> DerefMut for Guard<'_, T> {
    // Guarantees we've locked this lock
    fn deref_mut(&mut self) -> &mut T {
        unsafe {&mut *self.lock.value.get()}
    }
}

// When object gets out of scope we automatically 
// unlock this lock
impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, std::sync::atomic::Ordering::Release);
    }
}

mod tests {
    use std::thread;

    use crate::spin_lock::SpinLock;

    #[test]
    fn test_simple_spin_lock() {
        let sl = SpinLock::new(Vec::<u8>::new());

        thread::scope(|s| {
            s.spawn(|| {
                let mut res = sl.lock();
                res.push(1);
                res.push(2);
            });

            s.spawn(|| {
                let mut res = sl.lock();
                res.push(15);
            });
        });

        let g = sl.lock();
        print!("{:?}", g.as_slice());
        assert!(g.as_slice() == [1, 2, 15] || g.as_slice() == [15, 1, 2]);
    }
}