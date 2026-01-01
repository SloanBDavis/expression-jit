//! Runtime support for executing JIT-compiled code.
//!
//! Handles memory allocation with mmap/mprotect for executable code.

use std::ptr;

pub struct CompiledCode {
    ptr: *mut u8,
    len: usize,
}

impl CompiledCode {
    /// # Safety
    /// The caller must ensure the code bytes are valid x86-64 machine code.
    pub fn new(code: &[u8]) -> Result<Self, String> {
        let len = code.len();
        let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) } as usize;
        let alloc_size = ((len + page_size - 1) / page_size) * page_size;

        let ptr = unsafe {
            libc::mmap(
                ptr::null_mut(),
                alloc_size,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
                -1,
                0,
            )
        };

        if ptr == libc::MAP_FAILED {
            return Err("mmap failed".to_string());
        }

        let ptr = ptr as *mut u8;

        unsafe {
            ptr::copy_nonoverlapping(code.as_ptr(), ptr, len);
        }

        // Make executable (W^X: switch from writable to executable)
        let result = unsafe {
            libc::mprotect(
                ptr as *mut libc::c_void,
                alloc_size,
                libc::PROT_READ | libc::PROT_EXEC,
            )
        };

        if result != 0 {
            unsafe {
                libc::munmap(ptr as *mut libc::c_void, alloc_size);
            }
            return Err("mprotect failed".to_string());
        }

        Ok(Self {
            ptr,
            len: alloc_size,
        })
    }

    /// # Safety
    /// The compiled code must be valid and return an i64 in rax.
    pub unsafe fn execute(&self) -> i64 {
        let func: extern "C" fn() -> i64 = std::mem::transmute(self.ptr);
        func()
    }
}

impl Drop for CompiledCode {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.ptr as *mut libc::c_void, self.len);
        }
    }
}

unsafe impl Send for CompiledCode {}
unsafe impl Sync for CompiledCode {}
