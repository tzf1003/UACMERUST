// Command line parsing
// Port of Source/Shared/cmdline.c

use crate::strings::wstrlen;

/// Get command line parameter (wide string version)
/// Returns (success, param_length)
pub fn get_command_line_param_w(
    cmdline: *const u16,
    param_index: u32,
    buffer: *mut u16,
    buffer_size: u32,
) -> (bool, u32) {
    if cmdline.is_null() {
        if !buffer.is_null() && buffer_size > 0 {
            unsafe { *buffer = 0; }
        }
        return (false, 0);
    }
    
    unsafe {
        let mut cmd = cmdline;
        let mut plen = 0u32;
        
        for c in 0..=param_index {
            plen = 0;
            
            // Skip leading spaces
            while *cmd == ' ' as u16 {
                cmd = cmd.add(1);
            }
            
            // Check for end of string
            if *cmd == 0 {
                if !buffer.is_null() && buffer_size > 0 {
                    *buffer = 0;
                }
                return (plen < buffer_size, plen);
            }
            
            // Determine divider
            let divider = if *cmd == '"' as u16 {
                cmd = cmd.add(1);
                '"' as u16
            } else {
                ' ' as u16
            };
            
            // Extract parameter
            while *cmd != '"' as u16 && *cmd != divider && *cmd != 0 {
                plen += 1;
                if c == param_index {
                    if plen <= buffer_size && !buffer.is_null() {
                        *buffer.add((plen - 1) as usize) = *cmd;
                    }
                }
                cmd = cmd.add(1);
            }
            
            if *cmd != 0 {
                cmd = cmd.add(1);
            }
        }
        
        // Null-terminate
        if !buffer.is_null() && buffer_size > 0 {
            let term_pos = std::cmp::min(plen as usize, (buffer_size - 1) as usize);
            *buffer.add(term_pos) = 0;
        }
        
        (plen < buffer_size, plen)
    }
}

/// Get command line parameter (ANSI version)
pub fn get_command_line_param_a(
    cmdline: *const u8,
    param_index: u32,
    buffer: *mut u8,
    buffer_size: u32,
) -> (bool, u32) {
    if cmdline.is_null() {
        if !buffer.is_null() && buffer_size > 0 {
            unsafe { *buffer = 0; }
        }
        return (false, 0);
    }
    
    unsafe {
        let mut cmd = cmdline;
        let mut plen = 0u32;
        
        for c in 0..=param_index {
            plen = 0;
            
            // Skip leading spaces
            while *cmd == b' ' {
                cmd = cmd.add(1);
            }
            
            // Check for end of string
            if *cmd == 0 {
                if !buffer.is_null() && buffer_size > 0 {
                    *buffer = 0;
                }
                return (plen < buffer_size, plen);
            }
            
            // Determine divider
            let divider = if *cmd == b'"' {
                cmd = cmd.add(1);
                b'"'
            } else {
                b' '
            };
            
            // Extract parameter
            while *cmd != b'"' && *cmd != divider && *cmd != 0 {
                plen += 1;
                if c == param_index {
                    if plen <= buffer_size && !buffer.is_null() {
                        *buffer.add((plen - 1) as usize) = *cmd;
                    }
                }
                cmd = cmd.add(1);
            }
            
            if *cmd != 0 {
                cmd = cmd.add(1);
            }
        }
        
        // Null-terminate
        if !buffer.is_null() && buffer_size > 0 {
            let term_pos = std::cmp::min(plen as usize, (buffer_size - 1) as usize);
            *buffer.add(term_pos) = 0;
        }
        
        (plen < buffer_size, plen)
    }
}

/// Extract file path from full filename (wide string)
pub fn extract_file_path_w(filename: *const u16, filepath: *mut u16) -> *mut u16 {
    if filename.is_null() || filepath.is_null() {
        return std::ptr::null_mut();
    }
    
    unsafe {
        let mut p = filename;
        let mut last_slash = filename;
        
        // Find last backslash
        while *p != 0 {
            if *p == '\\' as u16 {
                last_slash = p.add(1);
            }
            p = p.add(1);
        }
        
        // Copy path portion
        let mut src = filename;
        let mut dst = filepath;
        while src < last_slash {
            *dst = *src;
            dst = dst.add(1);
            src = src.add(1);
        }
        *dst = 0;
        
        filepath
    }
}

/// Extract file path from full filename (ANSI)
pub fn extract_file_path_a(filename: *const u8, filepath: *mut u8) -> *mut u8 {
    if filename.is_null() || filepath.is_null() {
        return std::ptr::null_mut();
    }
    
    unsafe {
        let mut p = filename;
        let mut last_slash = filename;
        
        // Find last backslash
        while *p != 0 {
            if *p == b'\\' {
                last_slash = p.add(1);
            }
            p = p.add(1);
        }
        
        // Copy path portion
        let mut src = filename;
        let mut dst = filepath;
        while src < last_slash {
            *dst = *src;
            dst = dst.add(1);
            src = src.add(1);
        }
        *dst = 0;
        
        filepath
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_command_line_param() {
        let cmdline = "program.exe arg1 \"arg 2\" arg3\0".encode_utf16().collect::<Vec<u16>>();
        let mut buffer = vec![0u16; 256];
        
        let (success, len) = get_command_line_param_w(
            cmdline.as_ptr(),
            1,
            buffer.as_mut_ptr(),
            256,
        );
        
        assert!(success);
        assert_eq!(len, 4); // "arg1"
    }
}

