#![no_std]

extern crate alloc;

use alloc::borrow::ToOwned;
use alloc::string::String;

use lazy_static::*;

use linux_kernel_module;
use linux_kernel_module::println;
use linux_kernel_module::sync;
use linux_kernel_module::sync::Spinlock;

struct SyncTest {
    message: String,
}

lazy_static! {
    static ref GLOBAL: Spinlock<i32> = Spinlock::new(0);
}

fn global_synchronization_example() {
    let mut global = GLOBAL.lock();
    *global = 1;
}

impl linux_kernel_module::KernelModule for SyncTest {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        global_synchronization_example();
        let spinlock_data = sync::Spinlock::new(100);
        println!("Data {} is locked by a spinlock", *spinlock_data.lock());
        let mutex_data = sync::Mutex::new(50);
        let mut data = mutex_data.lock();
        println!("Data {} is locked by a mutex", *data);
        *data = 100;
        println!("Now data is {}", *data);
        sync::drop(data);
        println!("Hello kernel module!");
        Ok(SyncTest {
            message: "on the heap!".to_owned(),
        })
    }
}

impl Drop for SyncTest {
    fn drop(&mut self) {
        println!("Global lock contains {}", *GLOBAL.lock());
        println!("My message is {}", self.message);
        println!("Goodbye kernel module!");
    }
}

linux_kernel_module::kernel_module!(
    SyncTest,
    author: "Fish in a Barrel + Lizhuohua + coms6118 Frankenstein",
    description: "A simple locking test",
    license: "GPL"
);
