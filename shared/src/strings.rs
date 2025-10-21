// String manipulation functions
// Port of Source/Shared/_str*.c

use std::ptr;

/// Wide string length (equivalent to _strlen_w)
#[inline]
pub fn wstrlen(s: *const u16) -> usize {
    if s.is_null() {
        return 0;
    }
    
    unsafe {
        let mut len = 0;
        let mut ptr = s;
        while *ptr != 0 {
            len += 1;
            ptr = ptr.add(1);
        }
        len
    }
}

/// ANSI string length (equivalent to _strlen_a)
#[inline]
pub fn strlen(s: *const u8) -> usize {
    if s.is_null() {
        return 0;
    }
    
    unsafe {
        let mut len = 0;
        let mut ptr = s;
        while *ptr != 0 {
            len += 1;
            ptr = ptr.add(1);
        }
        len
    }
}

/// Wide string copy (equivalent to _strcpy_w)
pub fn wstrcpy(dest: *mut u16, src: *const u16) -> *mut u16 {
    if dest.is_null() || src.is_null() {
        return dest;
    }
    
    if dest == src as *mut u16 {
        return dest;
    }
    
    unsafe {
        let mut d = dest;
        let mut s = src;
        while *s != 0 {
            *d = *s;
            d = d.add(1);
            s = s.add(1);
        }
        *d = 0;
    }
    
    dest
}

/// ANSI string copy (equivalent to _strcpy_a)
pub fn strcpy(dest: *mut u8, src: *const u8) -> *mut u8 {
    if dest.is_null() || src.is_null() {
        return dest;
    }
    
    if dest == src as *mut u8 {
        return dest;
    }
    
    unsafe {
        let mut d = dest;
        let mut s = src;
        while *s != 0 {
            *d = *s;
            d = d.add(1);
            s = s.add(1);
        }
        *d = 0;
    }
    
    dest
}

/// Wide string concatenate (equivalent to _strcat_w)
pub fn wstrcat(dest: *mut u16, src: *const u16) -> *mut u16 {
    if dest.is_null() || src.is_null() {
        return dest;
    }
    
    unsafe {
        let mut d = dest;
        // Find end of dest
        while *d != 0 {
            d = d.add(1);
        }
        
        // Copy src to end of dest
        let mut s = src;
        while *s != 0 {
            *d = *s;
            d = d.add(1);
            s = s.add(1);
        }
        *d = 0;
    }
    
    dest
}

/// ANSI string concatenate (equivalent to _strcat_a)
pub fn strcat(dest: *mut u8, src: *const u8) -> *mut u8 {
    if dest.is_null() || src.is_null() {
        return dest;
    }
    
    unsafe {
        let mut d = dest;
        // Find end of dest
        while *d != 0 {
            d = d.add(1);
        }
        
        // Copy src to end of dest
        let mut s = src;
        while *s != 0 {
            *d = *s;
            d = d.add(1);
            s = s.add(1);
        }
        *d = 0;
    }
    
    dest
}

/// Wide string compare (case-sensitive)
pub fn wstrcmp(s1: *const u16, s2: *const u16) -> i32 {
    if s1.is_null() || s2.is_null() {
        return if s1 == s2 { 0 } else { -1 };
    }
    
    unsafe {
        let mut p1 = s1;
        let mut p2 = s2;
        
        loop {
            let c1 = *p1;
            let c2 = *p2;
            
            if c1 != c2 {
                return (c1 as i32) - (c2 as i32);
            }
            
            if c1 == 0 {
                return 0;
            }
            
            p1 = p1.add(1);
            p2 = p2.add(1);
        }
    }
}

/// Wide string compare (case-insensitive)
pub fn wstrcmpi(s1: *const u16, s2: *const u16) -> i32 {
    if s1.is_null() || s2.is_null() {
        return if s1 == s2 { 0 } else { -1 };
    }
    
    unsafe {
        let mut p1 = s1;
        let mut p2 = s2;
        
        loop {
            let c1 = to_lower_wide(*p1);
            let c2 = to_lower_wide(*p2);
            
            if c1 != c2 {
                return (c1 as i32) - (c2 as i32);
            }
            
            if c1 == 0 {
                return 0;
            }
            
            p1 = p1.add(1);
            p2 = p2.add(1);
        }
    }
}

/// Convert wide char to lowercase
#[inline]
fn to_lower_wide(c: u16) -> u16 {
    if c >= 'A' as u16 && c <= 'Z' as u16 {
        c + ('a' as u16 - 'A' as u16)
    } else {
        c
    }
}

/// Wide string n-compare
pub fn wstrncmp(s1: *const u16, s2: *const u16, n: usize) -> i32 {
    if s1.is_null() || s2.is_null() || n == 0 {
        return 0;
    }
    
    unsafe {
        let mut p1 = s1;
        let mut p2 = s2;
        let mut count = 0;
        
        while count < n {
            let c1 = *p1;
            let c2 = *p2;
            
            if c1 != c2 {
                return (c1 as i32) - (c2 as i32);
            }
            
            if c1 == 0 {
                return 0;
            }
            
            p1 = p1.add(1);
            p2 = p2.add(1);
            count += 1;
        }
        
        0
    }
}

/// Wide string n-copy
pub fn wstrncpy(dest: *mut u16, src: *const u16, n: usize) -> *mut u16 {
    if dest.is_null() || src.is_null() || n == 0 {
        return dest;
    }
    
    unsafe {
        let mut d = dest;
        let mut s = src;
        let mut count = 0;
        
        while count < n && *s != 0 {
            *d = *s;
            d = d.add(1);
            s = s.add(1);
            count += 1;
        }
        
        // Null-terminate
        if count < n {
            *d = 0;
        }
    }
    
    dest
}

