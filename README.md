## Spin Lock

A minimal implementation of a thread safe spin-lock using rust

### Usage

1. Create a new lock by using
```rust
let sl = SpinLock::new(Vec::new())
```

2. You can lock this object by using `.lock()` method
```rust
sl.lock()
```

3. Unlocking happens when the object gets destroyed so we don't need
   to explicitly call the `.unlock()` method.

***NOTE:***
SpinLock is usually used on the threads which does not wait for long time 
on some stuff to happen. If your threads happens to wait for longer times
while processing something, you should not consider using this. Instead you 
can use `Mutex` which could be found inside the standard library.
