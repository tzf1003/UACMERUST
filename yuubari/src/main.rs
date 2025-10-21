// UACME Rust - Yuubari UAC Information Dumper
// Port of Source/Yuubari

use shared::*;
use windows::Win32::Foundation::*;
use windows::Win32::System::Registry::*;
use serde::{Serialize, Deserialize};
use serde_json;

const YUUBARI_MIN_SUPPORTED_BUILD: u32 = constants::NT_WIN7_RTM;
const YUUBARI_MAX_SUPPORTED_BUILD: u32 = constants::NT_WIN11_23H2;

#[derive(Debug, Serialize, Deserialize)]
struct UacInfo {
    build_number: u32,
    enable_lua: u32,
    consent_prompt_behavior_admin: u32,
    consent_prompt_behavior_user: u32,
    enable_installer_detection: u32,
    validate_admin_code_signatures: u32,
    enable_secure_uiaccess: u32,
    enable_virtualization: u32,
    filter_administrator_token: u32,
}

fn main() {
    env_logger::init();
    
    println!("Yuubari - UAC Information Dumper");
    println!("Version: 3.6.4");
    println!();
    
    let build_number = get_windows_build_number();
    
    println!("[*] Windows Build Number: {}", build_number);
    
    if build_number < YUUBARI_MIN_SUPPORTED_BUILD {
        eprintln!("[ERROR] Unsupported Windows version (build {} < {})", 
            build_number, YUUBARI_MIN_SUPPORTED_BUILD);
        exit_process(1);
    }
    
    if build_number > YUUBARI_MAX_SUPPORTED_BUILD {
        println!("[WARNING] Not all features may be available for this build");
    }
    
    println!();
    
    match dump_uac_info() {
        Ok(info) => {
            print_uac_info(&info);
            
            // Optionally save to JSON
            if let Ok(json) = serde_json::to_string_pretty(&info) {
                println!("\n[*] JSON Output:");
                println!("{}", json);
            }
            
            exit_process(0);
        }
        Err(e) => {
            eprintln!("[ERROR] Failed to dump UAC info: {}", e);
            exit_process(1);
        }
    }
}

/// Dump UAC information from registry
fn dump_uac_info() -> Result<UacInfo, String> {
    let key_path = to_wide_string(
        "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\System"
    );
    
    let key = winapi_ext::RegistryKey::open(
        HKEY_LOCAL_MACHINE,
        &key_path,
        KEY_READ.0,
    )?;
    
    Ok(UacInfo {
        build_number: get_windows_build_number(),
        enable_lua: read_reg_dword(&key, "EnableLUA").unwrap_or(0),
        consent_prompt_behavior_admin: read_reg_dword(&key, "ConsentPromptBehaviorAdmin").unwrap_or(0),
        consent_prompt_behavior_user: read_reg_dword(&key, "ConsentPromptBehaviorUser").unwrap_or(0),
        enable_installer_detection: read_reg_dword(&key, "EnableInstallerDetection").unwrap_or(0),
        validate_admin_code_signatures: read_reg_dword(&key, "ValidateAdminCodeSignatures").unwrap_or(0),
        enable_secure_uiaccess: read_reg_dword(&key, "EnableSecureUIAPaths").unwrap_or(0),
        enable_virtualization: read_reg_dword(&key, "EnableVirtualization").unwrap_or(0),
        filter_administrator_token: read_reg_dword(&key, "FilterAdministratorToken").unwrap_or(0),
    })
}

/// Read DWORD value from registry key
fn read_reg_dword(key: &winapi_ext::RegistryKey, value_name: &str) -> Result<u32, String> {
    let value_name_wide = to_wide_string(value_name);
    key.get_dword_value(&value_name_wide)
}

/// Print UAC information
fn print_uac_info(info: &UacInfo) {
    println!("=== UAC Configuration ===");
    println!();
    println!("Build Number: {}", info.build_number);
    println!();
    
    println!("EnableLUA: {} ({})", 
        info.enable_lua,
        if info.enable_lua != 0 { "Enabled" } else { "Disabled" }
    );
    
    println!("ConsentPromptBehaviorAdmin: {} ({})",
        info.consent_prompt_behavior_admin,
        get_consent_prompt_behavior_desc(info.consent_prompt_behavior_admin, true)
    );
    
    println!("ConsentPromptBehaviorUser: {} ({})",
        info.consent_prompt_behavior_user,
        get_consent_prompt_behavior_desc(info.consent_prompt_behavior_user, false)
    );
    
    println!("EnableInstallerDetection: {} ({})",
        info.enable_installer_detection,
        if info.enable_installer_detection != 0 { "Enabled" } else { "Disabled" }
    );
    
    println!("ValidateAdminCodeSignatures: {} ({})",
        info.validate_admin_code_signatures,
        if info.validate_admin_code_signatures != 0 { "Enabled" } else { "Disabled" }
    );
    
    println!("EnableSecureUIAPaths: {} ({})",
        info.enable_secure_uiaccess,
        if info.enable_secure_uiaccess != 0 { "Enabled" } else { "Disabled" }
    );
    
    println!("EnableVirtualization: {} ({})",
        info.enable_virtualization,
        if info.enable_virtualization != 0 { "Enabled" } else { "Disabled" }
    );
    
    println!("FilterAdministratorToken: {} ({})",
        info.filter_administrator_token,
        if info.filter_administrator_token != 0 { "Enabled" } else { "Disabled" }
    );
}

/// Get consent prompt behavior description
fn get_consent_prompt_behavior_desc(value: u32, is_admin: bool) -> &'static str {
    if is_admin {
        match value {
            0 => "Elevate without prompting",
            1 => "Prompt for credentials on the secure desktop",
            2 => "Prompt for consent on the secure desktop",
            3 => "Prompt for credentials",
            4 => "Prompt for consent",
            5 => "Prompt for consent for non-Windows binaries",
            _ => "Unknown",
        }
    } else {
        match value {
            0 => "Automatically deny elevation requests",
            1 => "Prompt for credentials on the secure desktop",
            3 => "Prompt for credentials",
            _ => "Unknown",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_consent_prompt_desc() {
        assert_eq!(
            get_consent_prompt_behavior_desc(0, true),
            "Elevate without prompting"
        );
        assert_eq!(
            get_consent_prompt_behavior_desc(5, true),
            "Prompt for consent for non-Windows binaries"
        );
    }
}

