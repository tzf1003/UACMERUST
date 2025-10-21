// UACME Rust - Naka Compression Tool
// Port of Source/Naka

use shared::*;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use flate2::Compression;
use flate2::write::DeflateEncoder;
use flate2::read::DeflateDecoder;

fn main() {
    env_logger::init();
    
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        exit_process(1);
    }
    
    let command = &args[1];
    
    match command.as_str() {
        "--stable" => {
            println!("[*] Creating secret tables...");
            create_secret_tables();
        }
        _ => {
            let input_file = command;
            println!("[*] Compressing file: {}", input_file);
            
            match compress_file(input_file) {
                Ok(output) => {
                    println!("[+] Compressed file created: {}", output);
                    exit_process(0);
                }
                Err(e) => {
                    eprintln!("[ERROR] Compression failed: {}", e);
                    exit_process(1);
                }
            }
        }
    }
    
    exit_process(0);
}

/// Print usage information
fn print_usage() {
    println!("Naka - UACME Compression Tool");
    println!();
    println!("Usage:");
    println!("  naka <input_file>    - Compress file");
    println!("  naka --stable        - Create secret tables");
}

/// Compress file using DEFLATE
fn compress_file(input_path: &str) -> Result<String, String> {
    let path = Path::new(input_path);
    
    if !path.exists() {
        return Err(format!("Input file not found: {}", input_path));
    }
    
    // Read input file
    let mut input_file = File::open(path)
        .map_err(|e| format!("Failed to open input file: {}", e))?;
    
    let mut input_data = Vec::new();
    input_file.read_to_end(&mut input_data)
        .map_err(|e| format!("Failed to read input file: {}", e))?;
    
    println!("[*] Input size: {} bytes", input_data.len());
    
    // Compress data
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(&input_data)
        .map_err(|e| format!("Compression failed: {}", e))?;
    
    let compressed_data = encoder.finish()
        .map_err(|e| format!("Compression finalization failed: {}", e))?;
    
    println!("[*] Compressed size: {} bytes", compressed_data.len());
    println!("[*] Compression ratio: {:.2}%", 
        (compressed_data.len() as f64 / input_data.len() as f64) * 100.0);
    
    // Create output filename
    let output_path = format!("{}.cd", input_path);
    
    // Write compressed data
    let mut output_file = File::create(&output_path)
        .map_err(|e| format!("Failed to create output file: {}", e))?;
    
    output_file.write_all(&compressed_data)
        .map_err(|e| format!("Failed to write output file: {}", e))?;
    
    Ok(output_path)
}

/// Decompress data
pub fn decompress_data(compressed: &[u8]) -> Result<Vec<u8>, String> {
    let mut decoder = DeflateDecoder::new(compressed);
    let mut decompressed = Vec::new();
    
    decoder.read_to_end(&mut decompressed)
        .map_err(|e| format!("Decompression failed: {}", e))?;
    
    Ok(decompressed)
}

/// Create secret tables (for obfuscation)
fn create_secret_tables() {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    // Generate pseudo-random tables based on timestamp
    let mut table32 = Vec::new();
    let mut table64 = Vec::new();
    
    for i in 0..256 {
        let value32 = ((timestamp as u32).wrapping_mul(i).wrapping_add(0xDEADBEEF)) ^ 0x12345678;
        table32.push(value32);
        
        let value64 = ((timestamp as u64).wrapping_mul(i as u64).wrapping_add(0xDEADBEEFCAFEBABE)) 
            ^ 0x123456789ABCDEF0;
        table64.push(value64);
    }
    
    // Write tables to files
    match write_secret_table("secrets32.bin", &table32) {
        Ok(_) => println!("[+] Created secrets32.bin"),
        Err(e) => eprintln!("[ERROR] Failed to create secrets32.bin: {}", e),
    }
    
    match write_secret_table("secrets64.bin", &table64) {
        Ok(_) => println!("[+] Created secrets64.bin"),
        Err(e) => eprintln!("[ERROR] Failed to create secrets64.bin: {}", e),
    }
}

/// Write secret table to file
fn write_secret_table<T>(filename: &str, table: &[T]) -> Result<(), String> 
where
    T: Copy,
{
    let mut file = File::create(filename)
        .map_err(|e| format!("Failed to create file: {}", e))?;
    
    let bytes = unsafe {
        std::slice::from_raw_parts(
            table.as_ptr() as *const u8,
            table.len() * std::mem::size_of::<T>(),
        )
    };
    
    file.write_all(bytes)
        .map_err(|e| format!("Failed to write file: {}", e))?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_compression() {
        let test_data = b"Hello, World! This is a test string for compression.";
        
        let mut encoder = DeflateEncoder::new(Vec::new(), Compression::best());
        encoder.write_all(test_data).unwrap();
        let compressed = encoder.finish().unwrap();
        
        let decompressed = decompress_data(&compressed).unwrap();
        
        assert_eq!(test_data.to_vec(), decompressed);
    }
}

