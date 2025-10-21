// PE Loader implementation
// Port of Source/Shared/ldr.c

use windows::Win32::System::Memory::*;
use windows::Win32::System::Diagnostics::Debug::*;
use std::ptr;
use crate::pe_modifier::{IMAGE_DOS_SIGNATURE, IMAGE_NT_SIGNATURE, ImageDosHeader};

// Type aliases for PE structures
type IMAGE_DOS_HEADER = ImageDosHeader;

// Relocation types
const IMAGE_REL_BASED_HIGHLOW: u16 = 3;
const IMAGE_REL_BASED_DIR64: u16 = 10;

// Define PE structures not available in windows-rs
#[repr(C)]
#[allow(non_snake_case)]
struct IMAGE_BASE_RELOCATION {
    VirtualAddress: u32,
    SizeOfBlock: u32,
}

#[repr(C)]
#[allow(non_snake_case)]
struct IMAGE_EXPORT_DIRECTORY {
    Characteristics: u32,
    TimeDateStamp: u32,
    MajorVersion: u16,
    MinorVersion: u16,
    Name: u32,
    Base: u32,
    NumberOfFunctions: u32,
    NumberOfNames: u32,
    AddressOfFunctions: u32,
    AddressOfNames: u32,
    AddressOfNameOrdinals: u32,
}

/// Align value greater than or equal to alignment
fn align_gt(p: u32, align: u32) -> u32 {
    if align == 0 {
        return p;
    }
    
    let remainder = p % align;
    if remainder == 0 {
        return p;
    }
    
    if p > u32::MAX - (align - remainder) {
        return p;
    }
    
    p + (align - remainder)
}

/// Align value less than or equal to alignment
fn align_le(p: u32, align: u32) -> u32 {
    if p % align == 0 {
        p
    } else {
        p - (p % align)
    }
}

/// Load PE image into memory
pub fn pe_loader_load_image(buffer: *const u8) -> Result<(*mut u8, u32), String> {
    if buffer.is_null() {
        return Err("Invalid buffer".to_string());
    }
    
    unsafe {
        // Check DOS header
        let dos_header = buffer as *const IMAGE_DOS_HEADER;
        if (*dos_header).e_magic != IMAGE_DOS_SIGNATURE {
            return Err("Invalid DOS signature".to_string());
        }
        
        if (*dos_header).e_lfanew < std::mem::size_of::<IMAGE_DOS_HEADER>() as i32 
            || (*dos_header).e_lfanew > 0xFFFFF {
            return Err("Invalid e_lfanew".to_string());
        }
        
        // Check NT headers
        let nt_headers = buffer.offset((*dos_header).e_lfanew as isize) as *const IMAGE_NT_HEADERS64;
        if (*nt_headers).Signature != IMAGE_NT_SIGNATURE {
            return Err("Invalid NT signature".to_string());
        }
        
        let file_header = &(*nt_headers).FileHeader;
        let opt_header_size = file_header.SizeOfOptionalHeader;
        
        if opt_header_size != std::mem::size_of::<IMAGE_OPTIONAL_HEADER32>() as u16
            && opt_header_size != std::mem::size_of::<IMAGE_OPTIONAL_HEADER64>() as u16 {
            return Err("Invalid optional header size".to_string());
        }
        
        let opt_header = &(*nt_headers).OptionalHeader;
        
        // Allocate memory for image
        let image_size = opt_header.SizeOfImage;
        let exe_buffer = VirtualAlloc(
            Some(ptr::null()),
            image_size as usize,
            MEM_RESERVE | MEM_COMMIT,
            PAGE_READWRITE,
        );
        
        if exe_buffer.is_null() {
            return Err("Failed to allocate memory".to_string());
        }
        
        // Copy headers
        let headers_size = std::cmp::min(
            opt_header.SizeOfHeaders,
            opt_header.SizeOfHeaders
        );
        ptr::copy_nonoverlapping(
            buffer,
            exe_buffer as *mut u8,
            headers_size as usize,
        );
        
        // Copy sections
        let sections = (nt_headers as *const u8)
            .offset(std::mem::size_of::<IMAGE_NT_HEADERS64>() as isize)
            as *const IMAGE_SECTION_HEADER;
        
        for i in 0..file_header.NumberOfSections {
            let section = &*sections.offset(i as isize);
            
            if section.SizeOfRawData > 0 && section.PointerToRawData > 0 {
                let dest = (exe_buffer as *mut u8).offset(section.VirtualAddress as isize);
                let src = buffer.offset(
                    align_le(section.PointerToRawData, opt_header.FileAlignment) as isize
                );
                let size = align_gt(section.SizeOfRawData, opt_header.FileAlignment);
                
                ptr::copy_nonoverlapping(src, dest, size as usize);
            }
        }
        
        // Process relocations
        let reloc_dir = &opt_header.DataDirectory[IMAGE_DIRECTORY_ENTRY_BASERELOC.0 as usize];
        if reloc_dir.Size > 0 {
            let delta = exe_buffer as isize - opt_header.ImageBase as isize;
            
            if delta != 0 {
                let mut reloc = (exe_buffer as *mut u8).offset(reloc_dir.VirtualAddress as isize)
                    as *mut IMAGE_BASE_RELOCATION;
                let reloc_end = (reloc as *const u8).offset(reloc_dir.Size as isize);
                
                while (reloc as *const u8) < reloc_end && (*reloc).SizeOfBlock > 0 {
                    let count = ((*reloc).SizeOfBlock as usize 
                        - std::mem::size_of::<IMAGE_BASE_RELOCATION>()) / 2;
                    let entries = (reloc as *const u8)
                        .offset(std::mem::size_of::<IMAGE_BASE_RELOCATION>() as isize)
                        as *const u16;
                    
                    for j in 0..count {
                        let entry = *entries.offset(j as isize);
                        let reloc_type = entry >> 12;
                        let offset = entry & 0x0FFF;
                        
                        let target = (exe_buffer as *mut u8)
                            .offset((*reloc).VirtualAddress as isize)
                            .offset(offset as isize);
                        
                        match reloc_type {
                            IMAGE_REL_BASED_HIGHLOW => {
                                let ptr = target as *mut u32;
                                *ptr = (*ptr as isize + delta) as u32;
                            }
                            IMAGE_REL_BASED_DIR64 => {
                                let ptr = target as *mut u64;
                                *ptr = (*ptr as isize + delta) as u64;
                            }
                            _ => {}
                        }
                    }
                    
                    reloc = (reloc as *const u8).offset((*reloc).SizeOfBlock as isize)
                        as *mut IMAGE_BASE_RELOCATION;
                }
            }
        }
        
        Ok((exe_buffer as *mut u8, image_size))
    }
}

/// Get procedure address from loaded PE image
pub fn pe_loader_get_proc_address(
    image_base: *const u8,
    routine_name: &str,
) -> Option<*const ()> {
    if image_base.is_null() {
        return None;
    }
    
    unsafe {
        let dos_header = image_base as *const IMAGE_DOS_HEADER;
        let nt_headers = image_base.offset((*dos_header).e_lfanew as isize)
            as *const IMAGE_NT_HEADERS64;
        
        let export_dir_rva = (*nt_headers).OptionalHeader
            .DataDirectory[IMAGE_DIRECTORY_ENTRY_EXPORT.0 as usize]
            .VirtualAddress;
        
        if export_dir_rva == 0 {
            return None;
        }
        
        let export_dir = image_base.offset(export_dir_rva as isize)
            as *const IMAGE_EXPORT_DIRECTORY;
        
        let name_table = image_base.offset((*export_dir).AddressOfNames as isize)
            as *const u32;
        let function_table = image_base.offset((*export_dir).AddressOfFunctions as isize)
            as *const u32;
        let ordinal_table = image_base.offset((*export_dir).AddressOfNameOrdinals as isize)
            as *const u16;
        
        // Binary search for function name
        let mut low = 0usize;
        let mut high = (*export_dir).NumberOfNames as usize;
        
        while low < high {
            let middle = (low + high) / 2;
            let name_rva = *name_table.offset(middle as isize);
            let current_name = image_base.offset(name_rva as isize) as *const i8;
            
            let cmp = compare_strings(current_name, routine_name);
            
            if cmp == 0 {
                let ordinal = *ordinal_table.offset(middle as isize);
                let function_rva = *function_table.offset(ordinal as isize);
                let function_addr = image_base.offset(function_rva as isize);
                return Some(function_addr as *const ());
            } else if cmp < 0 {
                low = middle + 1;
            } else {
                high = middle;
            }
        }
        
        None
    }
}

/// Compare C string with Rust string
unsafe fn compare_strings(c_str: *const i8, rust_str: &str) -> i32 {
    let mut i = 0;
    let rust_bytes = rust_str.as_bytes();
    
    loop {
        let c_char = *c_str.offset(i);
        let r_char = if i < rust_bytes.len() as isize {
            rust_bytes[i as usize] as i8
        } else {
            0
        };
        
        if c_char != r_char {
            return c_char as i32 - r_char as i32;
        }
        
        if c_char == 0 {
            return 0;
        }
        
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_align_functions() {
        assert_eq!(align_gt(100, 16), 112);
        assert_eq!(align_le(100, 16), 96);
        assert_eq!(align_gt(96, 16), 96);
        assert_eq!(align_le(96, 16), 96);
    }
}

