// Payload Loader
// Load embedded payload DLLs from resources

use shared::*;

/// Load payload by ID
pub fn load_payload(payload_id: u32) -> Option<Vec<u8>> {
    match payload_id {
        constants::PAYLOAD_ID_NONE => None,
        constants::FUBUKI_ID => load_fubuki(),
        constants::FUBUKI32_ID => load_fubuki32(),
        constants::AKATSUKI_ID => load_akatsuki(),
        constants::KAMIKAZE_ID => load_kamikaze(),
        _ => {
            log::error!("Unknown payload ID: {}", payload_id);
            None
        }
    }
}

/// Load Fubuki payload (64-bit)
fn load_fubuki() -> Option<Vec<u8>> {
    // In a real implementation, this would load from embedded resources
    // For now, try to load from file system as fallback
    
    log::warn!("Fubuki payload not embedded, attempting to load from file");
    
    #[cfg(target_arch = "x86_64")]
    {
        std::fs::read("target/release/fubuki.dll").ok()
    }
    
    #[cfg(not(target_arch = "x86_64"))]
    {
        None
    }
}

/// Load Fubuki payload (32-bit)
fn load_fubuki32() -> Option<Vec<u8>> {
    log::warn!("Fubuki32 payload not embedded, attempting to load from file");
    
    #[cfg(target_arch = "x86")]
    {
        std::fs::read("target/release/fubuki.dll").ok()
    }
    
    #[cfg(not(target_arch = "x86"))]
    {
        None
    }
}

/// Load Akatsuki payload
fn load_akatsuki() -> Option<Vec<u8>> {
    log::warn!("Akatsuki payload not embedded, attempting to load from file");
    
    std::fs::read("target/release/akatsuki.dll").ok()
}

/// Load Kamikaze payload
fn load_kamikaze() -> Option<Vec<u8>> {
    log::warn!("Kamikaze payload not embedded");
    None
}

// For embedding payloads at compile time, you would use:
// const FUBUKI_DLL: &[u8] = include_bytes!("../../target/release/fubuki.dll");
// But this requires the DLLs to be built first

