// UACME Rust - Shared Library
// Port of Source/Shared

pub mod strings;
pub mod cmdline;
pub mod pe_loader;
pub mod pe_modifier;
pub mod winapi_ext;
pub mod anti_emulator;
pub mod util;
pub mod constants;
pub mod com_helper;

// Re-export commonly used items
pub use strings::*;
pub use cmdline::*;
pub use util::*;
pub use constants::*;

use windows::Win32::Foundation::*;
use windows::Win32::System::Threading::*;
use std::ffi::c_void;

/// Get current process heap
#[inline]
pub fn get_process_heap() -> HANDLE {
    unsafe {
        windows::Win32::System::Memory::GetProcessHeap().unwrap_or(HANDLE::default())
    }
}

/// Heap allocation wrapper (equivalent to ucmxHeapAlloc)
pub fn heap_alloc(size: usize) -> Option<*mut c_void> {
    unsafe {
        let heap = get_process_heap();
        let ptr = windows::Win32::System::Memory::HeapAlloc(
            heap,
            windows::Win32::System::Memory::HEAP_ZERO_MEMORY,
            size,
        );
        if ptr.is_null() {
            None
        } else {
            Some(ptr)
        }
    }
}

/// Heap free wrapper (equivalent to ucmxHeapFree)
pub fn heap_free(ptr: *mut c_void) -> bool {
    unsafe {
        use windows::Win32::System::Memory::HEAP_FLAGS;
        let heap = get_process_heap();
        windows::Win32::System::Memory::HeapFree(heap, HEAP_FLAGS(0), Some(ptr)).is_ok()
    }
}

/// Check if process is 32-bit (WOW64)
pub fn is_process_32bit(process: HANDLE) -> bool {
    unsafe {
        let mut is_wow64 = false.into();
        if windows::Win32::System::Threading::IsWow64Process(process, &mut is_wow64).is_ok() {
            is_wow64.as_bool()
        } else {
            false
        }
    }
}

/// Get current process handle
#[inline]
pub fn current_process() -> HANDLE {
    unsafe { GetCurrentProcess() }
}

/// Exit process
#[inline]
pub fn exit_process(exit_code: u32) -> ! {
    unsafe {
        ExitProcess(exit_code);
    }
}

