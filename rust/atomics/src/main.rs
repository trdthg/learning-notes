use std::{sync::atomic::{AtomicBool, Ordering, AtomicUsize}, thread::{self, spawn}, cell::UnsafeCell};

const UNLOCKED: bool = true;
const LOCKED: bool = false;

struct Mutex<T> {
    locked: AtomicBool,
    v: UnsafeCell<T>
}

unsafe impl<T> Sync for Mutex<T> where T: Send {}

impl<T> Mutex<T> {

    pub fn new(v: T) -> Self {
        Self {
            locked: AtomicBool::new(UNLOCKED),
            v: UnsafeCell::new(v)
        }
    }

    /// 暴露一个可变引用出去
    ///
    /// 当线程A和线程B同时执行时，A，B可能同时拿到🔓，并同时上锁，这两个线程并没有`看到`对方页拿到了锁
    /// 所以就出先了， 线程A和B同时从寄存器拿到值1，改为了2, 然后复制到寄存器内，后修改的会覆盖前一次修改
    /// 为了解决上述问题，下面是一些解决方法
    ///
    /// 方案1, 使用`compare_exchange`合并加锁上锁过程
    /// while self.locked.compare_exchange(UNLOCKED, LOCKED, Ordering::Relaxed, Ordering::Relaxed).is_err() {}
    ///
    /// #1 compare_exchange是低效的，如果是多核都在同时争夺锁，8个核中有一个和先拿到了锁
    /// 那么剩下的7个核依然会互相竞争，这个变量的内存就会在多个核中不断拷贝
    /// #2 相比于mutex，mutex拿不到锁就会阻塞线程，而compare_exchange拿不到就会返回一个Err
    /// #3 rust中还提供了compare_and_exchange_weak, 最大的区别是
    ///     compare_and_exchange只允许在判断Current v alue和传入的值不一样时返回Err
    ///     compare_and_exchange_weak即使在一样的时候也可能会返回Err，这种细小的差别能够用于某些场景，让性能更好
    ///     原因是由于在不同平台上的实现不同
    ///         x86: compare_and_swap
    ///         ARM: LDREX STREX
    ///     在x86上weak与普通的相同，
    ///     在ARM上：
    ///         compare_and_exchange: impl using a loop of LDREX and STREX
    ///         compare_and_exchange_weak: LDREX and STREX with no loop, it may be fake
    pub fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        // 拿🔓 上🔓
        // while self.locked.load(Ordering::Relaxed) != UNLOCKED {}
        // self.locked.store(LOCKED, Ordering::Relaxed);

        // 方案1
        // while self.locked.compare_exchange(UNLOCKED, LOCKED, Ordering::Relaxed, Ordering::Relaxed).is_err() {}

        // 方案2 在arm下有更好的性能
        while self.locked.compare_exchange_weak(UNLOCKED, LOCKED, Ordering::Relaxed, Ordering::Relaxed).is_err() {
            // 假如现在current value就是UNLOCKED状态，已经修改为LOCKED状态，那么就已经拿到了所有权
            // 加入现在还没有修改完成，current value依然是UNLOCKED状态，当前线程就会卡住，直到有别的线程成功拿到了所有权
            while self.locked.load(Ordering::Relaxed) == LOCKED {
                thread::yield_now();
            }
            thread::yield_now();
        }


        // 暴露数据
        let ret = f(unsafe { &mut *self.v.get() });
        // 解🔓
        self.locked.store(UNLOCKED, Ordering::Relaxed);
        ret
    }

    /// 但是即使用了上面的东西也有可能会出问题，虽然在X86 64位系统上没有出现问题，这是因为他不支持！，他只支持Seq的，不过还是要说明问题
    /// 因为下面对变量v的修改和上锁释放锁的过程毫不相关，
    /// 所以下一行可能被重新排列在加锁之前，或者解锁之后，这两种都是不允许的，但是cpu和编译器就可能这样做
    ///
    /// 对此需要使用Acquire 和 Release
    pub fn with_lock2<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        // 任何之后的读写操作不会被重排到Acquire之前
        // 别的线程中的写操作，对于这里的Acquire都是可见的
        while self.locked.compare_exchange_weak(UNLOCKED, LOCKED, Ordering::Acquire, Ordering::Relaxed).is_err() {
            while self.locked.load(Ordering::Relaxed) == LOCKED {
                thread::yield_now();
            }
            thread::yield_now();
        }
        let ret = f(unsafe { &mut *self.v.get() });

        // 任何之前的读写操作不会被重排到Release之后
        // 这个线程里的所有写操作对别的线程中的Acquire都是可见的
        self.locked.store(UNLOCKED, Ordering::Release);
        ret
    }

}

#[test]
fn seq_test() {
    // fetch_add always succeed

    let x: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let y: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let z: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
    let tx = thread::spawn(move || {
        x.store(true, Ordering::Release);                  // ！！！
    });
    let ty = thread::spawn(move || {
        y.store(true, Ordering::Release);
    });
    let t1 = spawn(move || {
        while !x.load(Ordering::Acquire) {}
        if y.load(Ordering::Acquire) {
            z.fetch_add(1, Ordering::Release);
        }
    });
    let t2 = spawn(move || {
        while !y.load(Ordering::Acquire) {}                    // 这行和下面一行可能重排(或者说不是重排，单纯时可见性的问题，下面的x就是看到了x是false)
        if x.load(Ordering::Acquire) {                         // ！！！ x为false，当x被修改为true后，也不会发生改变
            z.fetch_add(1, Ordering::Release);
        }
    });
    println!("{}", z.load(Ordering::SeqCst));

    // What are the possibles for z?
    // - is 0 possibly ?
    //   经过判断，至少有下面的条件
    //     t1 must run after tx
    //     t2 must run after ty
    //   几种排列组合应该是1或2,没有0
    //   但是0还是可能的，
    //           t2    t1,t2
    //   MO(x)  false  true
    //           t1    t1,t2
    //   MO(y)  false  true
    // - is 1 possibly ?
    //   Yes: tx -> t1 -> ty -> t2
    // - is 2 possibly ?
    //   Yes: tx -> ty -> t1 -> t2

}

fn main() {
    let l: &'static _ = Box::leak(Box::new(Mutex::new(0)));

    let handles: Vec<_> = (0..1000).map(|_| {
        thread::spawn(move || {
            for _ in 0..1000 {
                l.with_lock(|v| {
                    *v += 1;
                })
            }
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }

    // 这里依然会报错
    assert_eq!(l.with_lock(|v| *v), 1000 * 1000);

}

#[test]
fn to_relaxed() {
    let x: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
    let y: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
    let t1 = thread::spawn(move || {
        // 读取x存到y里
        let r1 = y.load(Ordering::Relaxed);
        x.store(r1, Ordering::Relaxed);
        r1
    });
    let t2 = thread::spawn(move || {
        // 读取y存到x里
        let r2 = x.load(Ordering::Relaxed);
        y.store(42, Ordering::Relaxed);
        r2
    });

    let r1 = t1.join().unwrap();
    let r2 = t2.join().unwrap();
}