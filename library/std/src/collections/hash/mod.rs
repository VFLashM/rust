//! Unordered containers, implemented as hash-tables


#[derive(Clone)]
pub struct Dropper {
    kind: &'static str,
    fname: &'static str,
    line: u32,
    size: usize,
}

impl Default for Dropper {
    fn default() -> Self {
        Dropper { kind: "", fname: "", line: 0, size: 0 }
    }
}

impl Dropper {
    fn new(kind: &'static str, fname: &'static str, line: u32) -> Self {
        Self { kind, fname, line, size: 0 }
    }
    fn set(&mut self, size: usize) {
        self.size = size;
    }
    fn inc(&mut self) {
        self.size += 1;
    }
}

use crate::sync::atomic::{AtomicBool, Ordering};
use crate::fs::OpenOptions;
use crate::io::Write;

static mut USED: AtomicBool = AtomicBool::new(false);

impl Drop for Dropper {
    fn drop(&mut self) {
        if !self.fname.is_empty() {
            let logpath = format!("/tmp/rust/{}.{}", self.kind, crate::process::id());

            unsafe {
                while USED.compare_and_swap(false, true, Ordering::Relaxed) {}
            }

            {
                let mut file = OpenOptions::new().create(true).append(true).open(logpath).unwrap();
                writeln!(file, "{}:{} {}", self.fname, self.line, self.size).unwrap();
            }

            unsafe {
                USED.store(false, Ordering::Relaxed);
            }
        }
    }
}

pub mod map;
pub mod set;
