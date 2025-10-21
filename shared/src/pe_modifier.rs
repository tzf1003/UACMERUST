// PE File Modification
// Functions to modify PE files (DLL to EXE conversion, entry point patching, etc.)

use std::mem;

/// PE DOS header
#[repr(C)]
pub struct ImageDosHeader {
    pub e_magic: u16,
    pub e_cblp: u16,
    pub e_cp: u16,
    pub e_crlc: u16,
    pub e_cparhdr: u16,
    pub e_minalloc: u16,
    pub e_maxalloc: u16,
    pub e_ss: u16,
    pub e_sp: u16,
    pub e_csum: u16,
    pub e_ip: u16,
    pub e_cs: u16,
    pub e_lfarlc: u16,
    pub e_ovno: u16,
    pub e_res: [u16; 4],
    pub e_oemid: u16,
    pub e_oeminfo: u16,
    pub e_res2: [u16; 10],
    pub e_lfanew: i32,
}

pub const IMAGE_DOS_SIGNATURE: u16 = 0x5A4D; // MZ
pub const IMAGE_NT_SIGNATURE: u32 = 0x00004550; // PE\0\0

/// Replace DLL entry point with new entry point and convert to EXE
pub fn replace_dll_entry_point(
    dll_data: &mut [u8],
    entry_point_name: &str,
    convert_to_exe: bool,
) -> Result<(), String> {
    if dll_data.len() < mem::size_of::<ImageDosHeader>() {
        return Err("Invalid PE file: too small".to_string());
    }
    
    // Read DOS header
    let dos_header = unsafe {
        &*(dll_data.as_ptr() as *const ImageDosHeader)
    };
    
    if dos_header.e_magic != IMAGE_DOS_SIGNATURE {
        return Err("Invalid DOS signature".to_string());
    }
    
    let nt_headers_offset = dos_header.e_lfanew as usize;
    
    if nt_headers_offset + 4 > dll_data.len() {
        return Err("Invalid NT headers offset".to_string());
    }
    
    // Check NT signature
    let nt_signature = unsafe {
        *(dll_data.as_ptr().add(nt_headers_offset) as *const u32)
    };
    
    if nt_signature != IMAGE_NT_SIGNATURE {
        return Err("Invalid NT signature".to_string());
    }
    
    // Get file header offset
    let file_header_offset = nt_headers_offset + 4;
    
    // Get optional header offset (after file header which is 20 bytes)
    let optional_header_offset = file_header_offset + 20;
    
    if optional_header_offset + 28 > dll_data.len() {
        return Err("Invalid optional header offset".to_string());
    }
    
    // Read magic to determine if 32-bit or 64-bit
    let magic = unsafe {
        *(dll_data.as_ptr().add(optional_header_offset) as *const u16)
    };
    
    const IMAGE_NT_OPTIONAL_HDR32_MAGIC: u16 = 0x10b;
    const IMAGE_NT_OPTIONAL_HDR64_MAGIC: u16 = 0x20b;
    
    let is_64bit = match magic {
        IMAGE_NT_OPTIONAL_HDR32_MAGIC => false,
        IMAGE_NT_OPTIONAL_HDR64_MAGIC => true,
        _ => return Err("Invalid optional header magic".to_string()),
    };
    
    // Convert DLL to EXE if requested
    if convert_to_exe {
        // Characteristics offset in file header
        let characteristics_offset = file_header_offset + 18;
        
        if characteristics_offset + 2 > dll_data.len() {
            return Err("Invalid characteristics offset".to_string());
        }
        
        // Clear IMAGE_FILE_DLL flag (0x2000)
        let characteristics = unsafe {
            &mut *(dll_data.as_mut_ptr().add(characteristics_offset) as *mut u16)
        };
        
        *characteristics &= !0x2000;
        
        log::info!("Converted DLL to EXE");
    }
    
    // Find export directory to locate entry point
    // For simplicity, we'll just modify the AddressOfEntryPoint in optional header
    // In a real implementation, you'd parse the export directory
    
    // AddressOfEntryPoint is at offset 16 in optional header (after magic and linker version)
    let entry_point_rva_offset = optional_header_offset + 16;
    
    if entry_point_rva_offset + 4 > dll_data.len() {
        return Err("Invalid entry point RVA offset".to_string());
    }
    
    // For now, we'll keep the existing entry point
    // A full implementation would:
    // 1. Parse export directory
    // 2. Find the RVA of the named export
    // 3. Update AddressOfEntryPoint to point to it
    
    log::warn!("Entry point replacement not fully implemented");
    log::info!("Entry point name: {}", entry_point_name);
    
    Ok(())
}

/// Get export function RVA by name
pub fn get_export_rva(dll_data: &[u8], function_name: &str) -> Result<u32, String> {
    // This would parse the export directory and find the function RVA
    // For now, return error
    Err("Export parsing not implemented".to_string())
}

/// Patch PE checksum
pub fn update_pe_checksum(pe_data: &mut [u8]) -> Result<(), String> {
    // Calculate and update PE checksum
    // For now, just zero it out (Windows loader doesn't always check it)
    
    if pe_data.len() < mem::size_of::<ImageDosHeader>() {
        return Err("Invalid PE file".to_string());
    }
    
    let dos_header = unsafe {
        &*(pe_data.as_ptr() as *const ImageDosHeader)
    };
    
    if dos_header.e_magic != IMAGE_DOS_SIGNATURE {
        return Err("Invalid DOS signature".to_string());
    }
    
    let nt_headers_offset = dos_header.e_lfanew as usize;
    let optional_header_offset = nt_headers_offset + 24; // After signature and file header
    
    // CheckSum is at offset 64 in optional header
    let checksum_offset = optional_header_offset + 64;
    
    if checksum_offset + 4 > pe_data.len() {
        return Err("Invalid checksum offset".to_string());
    }
    
    // Zero out checksum
    let checksum = unsafe {
        &mut *(pe_data.as_mut_ptr().add(checksum_offset) as *mut u32)
    };
    
    *checksum = 0;
    
    Ok(())
}

