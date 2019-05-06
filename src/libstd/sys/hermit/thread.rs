#![allow(dead_code)]

use crate::ffi::CStr;
use crate::io;
use crate::time::Duration;
use crate::mem;
use crate::fmt;
use core::u32;

use crate::sys_common::thread::*;

pub type Tid = u32;

/// Priority of a task
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub struct Priority(u8);

impl Priority {
    pub const fn into(self) -> u8 {
        self.0
    }

    pub const fn from(x: u8) -> Self {
        Priority(x)
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub const NORMAL_PRIO: Priority = Priority::from(2);

extern "C" {
    fn sys_usleep(usecs: u64);
    fn sys_spawn(id: *mut Tid, func: extern "C" fn(usize), arg: usize, prio: u8, core_id: usize) -> i32;
    fn sys_yield();
}

pub struct Thread {
    tid: Tid
}

pub const DEFAULT_MIN_STACK_SIZE: usize = 262144;

impl Thread {
    pub unsafe fn new(_stack: usize, p: Box<dyn FnOnce()>)
        -> io::Result<Thread>
    {
        let mut tid: Tid = u32::MAX;
        let ret = sys_spawn(&mut tid as *mut Tid, thread_start, &*p as *const _ as *const u8 as usize,
                            Priority::into(NORMAL_PRIO), 0);

        return if ret == 0 {
            Ok(Thread { tid: tid })
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Unable to create thread!"))
        };

        extern fn thread_start(main: usize) {
            unsafe {
                start_thread(main as *mut u8);
            }
        }
    }

    #[inline]
    pub fn yield_now() {
        unsafe {
            sys_yield();
        }
    }

    #[inline]
    pub fn set_name(_name: &CStr) {
        // nope
    }

    #[inline]
    pub fn sleep(dur: Duration) {
        unsafe {
            sys_usleep(dur.as_micros() as u64);
        }
    }

    pub fn join(self) {
        //match self.0 {}
    }

    #[inline]
    pub fn id(&self) -> Tid { self.tid }

    #[inline]
    pub fn into_id(self) -> Tid {
        let id = self.tid;
        mem::forget(self);
        id
    }
}

pub mod guard {
    pub type Guard = !;
    pub unsafe fn current() -> Option<Guard> { None }
    pub unsafe fn init() -> Option<Guard> { None }
}
