// Anti-emulator detection
// Port of Source/Shared/windefend.c

use windows::Win32::Foundation::*;
use windows::Win32::System::LibraryLoader::*;
use std::ffi::c_void;

// Windows Defender emulator API hash table
const WD_EMULATOR_API_HASH_TABLE: [u32; 3] = [
    0x70CE7692,
    0xD4CE4554,
    0x7A99CFAE,
];

/// Simple hash function for API names
fn hash_string(s: &str) -> u32 {
    let mut hash = 0u32;
    for byte in s.bytes() {
        hash = hash.wrapping_mul(31).wrapping_add(byte as u32);
    }
    hash
}

/// Check if running in Windows Defender emulator
pub fn is_emulator_present() -> i32 {
    unsafe {
        // Try to load kernel32.dll
        let kernel32_name = "kernel32.dll\0";
        let h_kernel32 = GetModuleHandleA(
            windows::core::PCSTR(kernel32_name.as_ptr())
        );
        
        if h_kernel32.is_err() {
            return crate::constants::STATUS_NOT_SUPPORTED;
        }
        
        let h_kernel32 = h_kernel32.unwrap();
        
        // Check for specific emulator APIs
        let test_apis = [
            "wine_get_version",
            "wine_get_build_id",
            "__wine_syscall_dispatcher",
        ];
        
        for api_name in &test_apis {
            let api_cstr = format!("{}\0", api_name);
            let proc_addr = GetProcAddress(
                h_kernel32,
                windows::core::PCSTR(api_cstr.as_ptr()),
            );
            
            if proc_addr.is_some() {
                return crate::constants::STATUS_NEEDS_REMEDIATION;
            }
        }
        
        // Check hash table
        for &hash in &WD_EMULATOR_API_HASH_TABLE {
            // This is a simplified check
            // Original code does more sophisticated hash checking
            if hash == 0x70CE7692 {
                // Additional checks could be added here
            }
        }
        
        crate::constants::STATUS_NOT_SUPPORTED
    }
}

/// Check if running in emulated VFS (Virtual File System)
pub fn check_emulated_vfs() -> bool {
    unsafe {
        // Check for specific registry keys or file paths that indicate emulation
        // This is a simplified version
        
        use windows::Win32::System::Registry::*;
        
        let key_path = "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\0";
        let mut h_key = HKEY::default();
        
        let result = RegOpenKeyExA(
            HKEY_LOCAL_MACHINE,
            windows::core::PCSTR(key_path.as_ptr()),
            0,
            KEY_READ,
            &mut h_key,
        );
        
        if result.is_ok() {
            let _ = RegCloseKey(h_key);
            false
        } else {
            // If we can't open a basic registry key, might be in emulated environment
            true
        }
    }
}

/// Check for Wine environment
pub fn is_wine_present() -> bool {
    unsafe {
        let ntdll_name = "ntdll.dll\0";
        let h_ntdll = GetModuleHandleA(
            windows::core::PCSTR(ntdll_name.as_ptr())
        );
        
        if let Ok(h_ntdll) = h_ntdll {
            let wine_api = "wine_get_version\0";
            let proc_addr = GetProcAddress(
                h_ntdll,
                windows::core::PCSTR(wine_api.as_ptr()),
            );
            
            proc_addr.is_some()
        } else {
            false
        }
    }
}

/// Comprehensive emulator detection
pub fn detect_emulation() -> EmulatorType {
    if is_wine_present() {
        return EmulatorType::Wine;
    }
    
    let status = is_emulator_present();
    if status == crate::constants::STATUS_NEEDS_REMEDIATION {
        return EmulatorType::WindowsDefender;
    }
    
    if check_emulated_vfs() {
        return EmulatorType::Unknown;
    }
    
    EmulatorType::None
}

#[derive(Debug, PartialEq, Eq)]
pub enum EmulatorType {
    None,
    Wine,
    WindowsDefender,
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hash_string() {
        let hash = hash_string("test");
        assert!(hash > 0);
    }
    
    #[test]
    fn test_emulator_detection() {
        // This test will vary based on environment
        let emulator = detect_emulation();
        // Just ensure it doesn't crash
        println!("Emulator detection result: {:?}", emulator);
    }
}

