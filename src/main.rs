use std::sync::atomic::{fence, AtomicBool, AtomicUsize, Ordering};

static A: AtomicBool = AtomicBool::new(false);
static DATA: AtomicUsize = AtomicUsize::new(0);

static X: AtomicUsize = AtomicUsize::new(0);
static Y: AtomicUsize = AtomicUsize::new(0);
static Z: AtomicUsize = AtomicUsize::new(0);

fn release_acquire() {
    let t = std::thread::spawn(|| {
        DATA.store(1, Ordering::Relaxed);
        A.store(true, Ordering::Release);
    });

    let t2 = std::thread::spawn(|| {
        while !A.load(Ordering::Acquire) {}

        println!("rls {}", DATA.load(Ordering::Relaxed));
    });

    t.join().unwrap();
    t2.join().unwrap();
}

fn fence_ordering() {
    let t1 = std::thread::spawn(|| {
        println!("{}", X.load(Ordering::Relaxed));
        println!("{}", Y.load(Ordering::Relaxed));
        println!("{}", Z.load(Ordering::Relaxed));
        fence(Ordering::Acquire);
    });
    let t2 = std::thread::spawn(|| {
        fence(Ordering::Release);
        X.store(1, Ordering::Relaxed);
        Y.store(2, Ordering::Relaxed);
        Z.store(3, Ordering::Relaxed);
    });
    t1.join().unwrap();
    t2.join().unwrap();
}

fn main() {
    release_acquire();
    fence_ordering();
}
