pub mod structs;
pub mod processor;
pub mod scheduler;
pub mod thread_pool;

use structs::Thread;
use processor::Processor;
use scheduler::RRScheduler;
use thread_pool::ThreadPool;
use crate::alloc::{
    vec::Vec,
    boxed::Box,
};
use lazy_static::lazy_static;
use core::cell::UnsafeCell;


pub fn init() {
    
    /*
    let mut boot_thread = Thread::get_boot_thread();
    let mut temp_thread = Thread::new_kernel(temp_thread as usize);
    
    unsafe {
        temp_thread.append_initial_arguments([&*boot_thread as *const Thread as usize, &*temp_thread as *const Thread as usize, 0]);
    }
    boot_thread.switch_to(&mut temp_thread);
    
    println!("switched back from temp_thread!");
    loop {}
    */

    let scheduler = RRScheduler::new(1);
    let thread_pool = ThreadPool::new(100, Box::new(scheduler));

    let idle = Thread::new_kernel(Processor::idle_main as usize);
    idle.append_initial_arguments([&CPU as *const Processor as usize, 0, 0]);
    //CPU.init(Thread::new_idle(), Box::new(thread_pool));
    CPU.init(idle, Box::new(thread_pool));
    println!("CPU init successfully!");

    println!("hello_thread is at {:#x}", hello_thread as usize);
    for i in 0..5 {
        CPU.add_thread({
            let thread = Thread::new_kernel(hello_thread as usize);
            thread.append_initial_arguments([i, 0, 0]);
            thread
        });
    }

    extern "C" {
        fn _user_img_start();
        fn _user_img_end();
    }
    let data = unsafe {
        core::slice::from_raw_parts(
            _user_img_start as *const u8,
            _user_img_end as usize - _user_img_start as usize,
        )
    };
    let user_thread = unsafe { Thread::new_user(data) };
    CPU.add_thread(user_thread);
    println!("++++ setup process!   ++++");
}

pub fn run() {
    CPU.run();
}

#[no_mangle]
pub extern "C" fn temp_thread(from_thread: &mut Thread, current_thread: &mut Thread) {
    println!("I'm leaving soon, but I still want to say: Hello world!");
    current_thread.switch_to(from_thread);
}

#[no_mangle]
pub extern "C" fn hello_thread(arg: usize) -> ! {
    println!("begin of thread {}", arg);
    /*
    let a = 1000000;
    let b = 10000;
    for i in 0..a {
        if (i + 1) % b == 0 {
            println!("arg = {}, i = {}/{}", arg, i + 1, a);
        }
    }
    */
    for i in 0..800 {
        print!("{}", arg);
    }
    println!("\nend  of thread {}", arg);
    CPU.exit(0);
    loop {}
}

pub type Tid = usize;
pub type ExitCode = usize;


static CPU: Processor = Processor::new();

pub fn tick() {
    //println!("ready CPU.tick()");
    CPU.tick();
}

pub fn exit(code: usize) {
    CPU.exit(code);
}
