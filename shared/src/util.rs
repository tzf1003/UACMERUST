// Utility functions
// Port of Source/Shared/util.c

use windows::Win32::Foundation::*;
use windows::Win32::System::Threading::*;
use windows::Win32::Security::*;
use crate::strings::*;
use crate::{to_wide_string, from_wide_string};

/// Query system directory with WOW64 support
pub fn query_system_directory(check_wow64: bool) -> Vec<u16> {
    let mut buffer = vec![0u16; windows::Win32::Foundation::MAX_PATH as usize * 2];
    
    unsafe {
        // Get system root from shared user data
        let system_root = get_system_root();
        wstrcpy(buffer.as_mut_ptr(), system_root.as_ptr());
        
        // Append \sys
        let sys_prep: Vec<u16> = "\\sys".encode_utf16().chain(std::iter::once(0)).collect();
        wstrcat(buffer.as_mut_ptr(), sys_prep.as_ptr());
        
        // Append tem32\ or wow64\ based on process type
        if check_wow64 && crate::is_process_32bit(crate::current_process()) {
            let wow64_final: Vec<u16> = "wow64\\".encode_utf16().chain(std::iter::once(0)).collect();
            wstrcat(buffer.as_mut_ptr(), wow64_final.as_ptr());
        } else {
            let system32_final: Vec<u16> = "tem32\\".encode_utf16().chain(std::iter::once(0)).collect();
            wstrcat(buffer.as_mut_ptr(), system32_final.as_ptr());
        }
    }
    
    buffer
}

/// Get system root directory (e.g., C:\Windows)
fn get_system_root() -> Vec<u16> {
    let mut buffer = vec![0u16; windows::Win32::Foundation::MAX_PATH as usize];
    unsafe {
        let len = windows::Win32::System::SystemInformation::GetSystemWindowsDirectoryW(
            Some(&mut buffer)
        );
        if len > 0 {
            buffer.truncate((len + 1) as usize);
        }
    }
    buffer
}

/// Binary text encode - create pseudo random string from u64 value
pub fn bin_text_encode(x: u64) -> Vec<u16> {
    let mut tbl = [0u8; 64];
    
    // Initialize encoding table
    tbl[62] = b'-';
    tbl[63] = b'_';
    
    for c in 0..26u8 {
        tbl[c as usize] = b'A' + c;
        tbl[(26 + c) as usize] = b'a' + c;
        if c < 10 {
            tbl[(52 + c) as usize] = b'0' + c;
        }
    }
    
    let mut result = Vec::with_capacity(14);
    let mut value = x;
    
    for _ in 0..13 {
        let c = (value & 0x3f) as usize;
        value >>= 5;
        result.push(tbl[c] as u16);
    }
    
    result.push(0); // Null terminator
    result
}

/// Create unique object name
pub fn create_unique_name(prefix: &str, suffix: &str) -> Vec<u16> {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let encoded = bin_text_encode(timestamp);
    let encoded_str = from_wide_string(&encoded);
    
    let name = format!("{}{}{}", prefix, encoded_str, suffix);
    to_wide_string(&name)
}

/// Sleep wrapper
#[inline]
pub fn sleep(milliseconds: u32) {
    unsafe {
        windows::Win32::System::Threading::Sleep(milliseconds);
    }
}

/// Get current thread ID
#[inline]
pub fn get_current_thread_id() -> u32 {
    unsafe {
        windows::Win32::System::Threading::GetCurrentThreadId()
    }
}

/// Get current process ID
#[inline]
pub fn get_current_process_id() -> u32 {
    unsafe {
        windows::Win32::System::Threading::GetCurrentProcessId()
    }
}

/// Secure zero memory
pub fn secure_zero_memory(ptr: *mut u8, size: usize) {
    unsafe {
        std::ptr::write_bytes(ptr, 0, size);
        // Prevent compiler optimization
        std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
    }
}

/// Convert unsigned long to hex string (wide)
pub fn ulong_to_hex(value: u32) -> Vec<u16> {
    let hex_str = format!("{:08X}", value);
    to_wide_string(&hex_str)
}

/// Convert unsigned long to decimal string (wide)
pub fn ulong_to_str(value: u32) -> Vec<u16> {
    let dec_str = format!("{}", value);
    to_wide_string(&dec_str)
}

/// Convert u64 to hex string (wide)
pub fn u64_to_hex(value: u64) -> Vec<u16> {
    let hex_str = format!("{:016X}", value);
    to_wide_string(&hex_str)
}

/// Convert u64 to decimal string (wide)
pub fn u64_to_str(value: u64) -> Vec<u16> {
    let dec_str = format!("{}", value);
    to_wide_string(&dec_str)
}

/// String to unsigned long
pub fn str_to_ulong(s: &str) -> Option<u32> {
    s.parse().ok()
}

/// String to i32
pub fn str_to_i32(s: &str) -> Option<i32> {
    s.parse().ok()
}

/// Wide string to unsigned long
pub fn wstr_to_ulong(s: *const u16) -> Option<u32> {
    if s.is_null() {
        return None;
    }
    
    let rust_str = from_wide_string(unsafe {
        std::slice::from_raw_parts(s, wstrlen(s))
    });
    
    str_to_ulong(&rust_str)
}

/// Check if running with elevated privileges
pub fn is_elevated() -> bool {
    unsafe {
        let mut elevated = false;
        let mut token_handle = HANDLE::default();

        if OpenProcessToken(
            GetCurrentProcess(),
            TOKEN_ACCESS_MASK(TOKEN_QUERY.0),
            &mut token_handle,
        ).is_ok() {
            let mut elevation = TOKEN_ELEVATION { TokenIsElevated: 0 };
            let mut return_length = 0u32;

            if GetTokenInformation(
                token_handle,
                TokenElevation,
                Some(&mut elevation as *mut _ as *mut std::ffi::c_void),
                std::mem::size_of::<TOKEN_ELEVATION>() as u32,
                &mut return_length,
            ).is_ok() {
                elevated = elevation.TokenIsElevated != 0;
            }

            let _ = CloseHandle(token_handle);
        }

        elevated
    }
}

/// Get Windows build number
pub fn get_windows_build_number() -> u32 {
    use ntapi::winapi_local::um::winnt::NtCurrentTeb;
    unsafe {
        let peb = NtCurrentTeb().as_ref().unwrap().ProcessEnvironmentBlock;
        let peb_ref = peb.as_ref().unwrap();
        peb_ref.OSBuildNumber as u32
    }
}

/// Check if Windows version is in range
pub fn is_windows_version_in_range(min_build: u32, max_build: u32) -> bool {
    let current_build = get_windows_build_number();
    current_build >= min_build && current_build <= max_build
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bin_text_encode() {
        let encoded = bin_text_encode(12345);
        assert!(encoded.len() > 0);
        assert_eq!(encoded[encoded.len() - 1], 0); // Null terminated
    }
    
    #[test]
    fn test_ulong_to_hex() {
        let hex = ulong_to_hex(0xDEADBEEF);
        let hex_str = from_wide_string(&hex);
        assert_eq!(hex_str, "DEADBEEF");
    }
}

