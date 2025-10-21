// Console output utilities
// Port of Source/Akagi/console.c

use shared::*;

/// Print status message
pub fn console_print_status(message: &str, status: i32) {
    let status_str = match status {
        s if s == constants::STATUS_SUCCESS => "SUCCESS",
        s if s == constants::STATUS_ACCESS_DENIED => "ACCESS_DENIED",
        s if s == constants::STATUS_INVALID_PARAMETER => "INVALID_PARAMETER",
        s if s == constants::STATUS_NOT_SUPPORTED => "NOT_SUPPORTED",
        s if s == constants::STATUS_ELEVATION_REQUIRED => "ELEVATION_REQUIRED",
        _ => "UNKNOWN",
    };
    
    println!("{} - Status: {} (0x{:08X})", message, status_str, status as u32);
}

/// Print value (unsigned long)
pub fn console_print_value_ulong(message: &str, value: u32, as_hex: bool) {
    if as_hex {
        println!("{}: 0x{:08X}", message, value);
    } else {
        println!("{}: {}", message, value);
    }
}

/// Print value (u64)
pub fn console_print_value_u64(message: &str, value: u64, as_hex: bool) {
    if as_hex {
        println!("{}: 0x{:016X}", message, value);
    } else {
        println!("{}: {}", message, value);
    }
}

/// Print text message
pub fn console_print_text(message: &str, new_line: bool) {
    if new_line {
        println!("{}", message);
    } else {
        print!("{}", message);
    }
}

/// Print error message
pub fn console_print_error(message: &str) {
    eprintln!("[ERROR] {}", message);
}

/// Print warning message
pub fn console_print_warning(message: &str) {
    println!("[WARNING] {}", message);
}

/// Print info message
pub fn console_print_info(message: &str) {
    println!("[INFO] {}", message);
}

/// Print debug message (only in debug builds)
#[cfg(debug_assertions)]
pub fn console_print_debug(message: &str) {
    println!("[DEBUG] {}", message);
}

#[cfg(not(debug_assertions))]
pub fn console_print_debug(_message: &str) {
    // No-op in release builds
}

