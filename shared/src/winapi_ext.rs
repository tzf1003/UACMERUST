// Windows API extensions and wrappers
// Additional Windows API functionality not directly available in windows-rs

use windows::Win32::Foundation::*;
use windows::Win32::System::Threading::*;
use windows::Win32::System::Registry::*;
use windows::Win32::Storage::FileSystem::*;
use windows::core::*;

// Use std::result::Result to avoid conflict with windows::core::Result
type StdResult<T> = std::result::Result<T, String>;

/// Create process with specific parameters
pub fn create_process_simple(
    application_name: &[u16],
    command_line: Option<&[u16]>,
    show_window: i32,
) -> StdResult<PROCESS_INFORMATION> {
    unsafe {
        let si = STARTUPINFOW {
            cb: std::mem::size_of::<STARTUPINFOW>() as u32,
            dwFlags: STARTF_USESHOWWINDOW,
            wShowWindow: show_window as u16,
            ..Default::default()
        };
        
        let mut pi = PROCESS_INFORMATION::default();
        
        let mut cmd_line_buf = if let Some(cmd) = command_line {
            cmd.to_vec()
        } else {
            vec![0u16]
        };
        
        let result = CreateProcessW(
            PCWSTR(application_name.as_ptr()),
            PWSTR(if cmd_line_buf.len() > 1 { cmd_line_buf.as_mut_ptr() } else { std::ptr::null_mut() }),
            None,
            None,
            false,
            PROCESS_CREATION_FLAGS(0),
            None,
            None,
            &si,
            &mut pi,
        );
        
        if result.is_ok() {
            Ok(pi)
        } else {
            Err(format!("CreateProcess failed: {:?}", GetLastError()))
        }
    }
}

/// Run process and wait for completion
pub fn run_process_and_wait(
    application_name: &[u16],
    timeout_ms: u32,
) -> StdResult<u32> {
    let pi = create_process_simple(application_name, None, 0)?;
    
    unsafe {
        let wait_result = WaitForSingleObject(pi.hProcess, timeout_ms);
        
        let mut exit_code = 0u32;
        let _ = GetExitCodeProcess(pi.hProcess, &mut exit_code);
        
        let _ = CloseHandle(pi.hProcess);
        let _ = CloseHandle(pi.hThread);
        
        if wait_result == WAIT_OBJECT_0 {
            Ok(exit_code)
        } else {
            Err("Process wait timeout or failed".to_string())
        }
    }
}

/// Registry key operations
pub struct RegistryKey {
    hkey: HKEY,
}

impl RegistryKey {
    /// Open registry key
    pub fn open(root: HKEY, subkey: &[u16], access: u32) -> StdResult<Self> {
        unsafe {
            let mut hkey = HKEY::default();
            let result = RegOpenKeyExW(
                root,
                PCWSTR(subkey.as_ptr()),
                0,
                REG_SAM_FLAGS(access),
                &mut hkey,
            );

            if result.is_ok() {
                Ok(RegistryKey { hkey })
            } else {
                Err(format!("Failed to open registry key: {:?}", result))
            }
        }
    }

    /// Get internal HKEY handle
    pub fn hkey(&self) -> HKEY {
        self.hkey
    }
    
    /// Create registry key
    pub fn create(root: HKEY, subkey: &[u16], access: u32) -> StdResult<Self> {
        use windows::Win32::System::Registry::REG_CREATE_KEY_DISPOSITION;
        unsafe {
            let mut hkey = HKEY::default();
            let mut disposition = REG_CREATE_KEY_DISPOSITION::default();

            let result = RegCreateKeyExW(
                root,
                PCWSTR(subkey.as_ptr()),
                0,
                PCWSTR::null(),
                REG_OPTION_NON_VOLATILE,
                REG_SAM_FLAGS(access),
                None,
                &mut hkey,
                Some(&mut disposition),
            );
            
            if result.is_ok() {
                Ok(RegistryKey { hkey })
            } else {
                Err(format!("Failed to create registry key: {:?}", result))
            }
        }
    }
    
    /// Set string value
    pub fn set_string_value(&self, value_name: &[u16], data: &[u16]) -> StdResult<()> {
        unsafe {
            let data_bytes = std::slice::from_raw_parts(
                data.as_ptr() as *const u8,
                data.len() * 2,
            );
            let result = RegSetValueExW(
                self.hkey,
                PCWSTR(value_name.as_ptr()),
                0,
                REG_SZ,
                Some(data_bytes),
            );

            if result.is_ok() {
                Ok(())
            } else {
                Err(format!("Failed to set registry value: {:?}", result))
            }
        }
    }

    /// Get string value
    pub fn get_string_value(&self, value_name: &[u16]) -> StdResult<Vec<u16>> {
        unsafe {
            let mut size = 0u32;
            let mut value_type = REG_NONE;
            
            // Get size
            let _ = RegQueryValueExW(
                self.hkey,
                PCWSTR(value_name.as_ptr()),
                None,
                Some(&mut value_type),
                None,
                Some(&mut size),
            );
            
            if size == 0 {
                return Err("Value not found or empty".to_string());
            }
            
            let mut buffer = vec![0u16; (size / 2) as usize];
            
            let result = RegQueryValueExW(
                self.hkey,
                PCWSTR(value_name.as_ptr()),
                None,
                Some(&mut value_type),
                Some(buffer.as_mut_ptr() as *mut u8),
                Some(&mut size),
            );
            
            if result.is_ok() {
                Ok(buffer)
            } else {
                Err(format!("Failed to get registry value: {:?}", result))
            }
        }
    }
    
    /// Delete value
    pub fn delete_value(&self, value_name: &[u16]) -> StdResult<()> {
        unsafe {
            let result = RegDeleteValueW(
                self.hkey,
                PCWSTR(value_name.as_ptr()),
            );

            if result.is_ok() {
                Ok(())
            } else {
                Err(format!("Failed to delete registry value: {:?}", result))
            }
        }
    }

    /// Get DWORD value
    pub fn get_dword_value(&self, value_name: &[u16]) -> StdResult<u32> {
        unsafe {
            let mut value = 0u32;
            let mut size = std::mem::size_of::<u32>() as u32;
            let mut value_type = REG_NONE;

            let result = RegQueryValueExW(
                self.hkey,
                PCWSTR(value_name.as_ptr()),
                None,
                Some(&mut value_type),
                Some(&mut value as *mut _ as *mut u8),
                Some(&mut size),
            );

            if result.is_ok() && value_type == REG_DWORD {
                Ok(value)
            } else {
                Err(format!("Failed to get DWORD value: {:?}", result))
            }
        }
    }

    /// Set DWORD value
    pub fn set_dword_value(&self, value_name: &[u16], value: u32) -> StdResult<()> {
        unsafe {
            let value_bytes = value.to_le_bytes();
            let result = RegSetValueExW(
                self.hkey,
                PCWSTR(value_name.as_ptr()),
                0,
                REG_DWORD,
                Some(&value_bytes),
            );

            if result.is_ok() {
                Ok(())
            } else {
                Err(format!("Failed to set DWORD value: {:?}", result))
            }
        }
    }
}

impl Drop for RegistryKey {
    fn drop(&mut self) {
        unsafe {
            let _ = RegCloseKey(self.hkey);
        }
    }
}

/// File operations helper
pub struct FileHandle {
    handle: HANDLE,
}

impl FileHandle {
    /// Create or open file
    pub fn create(
        filename: &[u16],
        desired_access: u32,
        share_mode: u32,
        creation_disposition: u32,
    ) -> StdResult<Self> {
        unsafe {
            let handle = CreateFileW(
                PCWSTR(filename.as_ptr()),
                desired_access,
                FILE_SHARE_MODE(share_mode),
                None,
                FILE_CREATION_DISPOSITION(creation_disposition),
                FILE_FLAGS_AND_ATTRIBUTES(0),
                None,
            );
            
            if let Ok(h) = handle {
                if h.is_invalid() {
                    Err("Invalid file handle".to_string())
                } else {
                    Ok(FileHandle { handle: h })
                }
            } else {
                Err(format!("Failed to create file: {:?}", GetLastError()))
            }
        }
    }
    
    /// Write data to file
    pub fn write(&self, data: &[u8]) -> StdResult<u32> {
        use windows::Win32::Storage::FileSystem::WriteFile;
        unsafe {
            let mut bytes_written = 0u32;
            let result = WriteFile(
                self.handle,
                Some(data),
                Some(&mut bytes_written),
                None,
            );
            
            if result.is_ok() {
                Ok(bytes_written)
            } else {
                Err(format!("Failed to write file: {:?}", GetLastError()))
            }
        }
    }
    
    /// Read data from file
    pub fn read(&self, buffer: &mut [u8]) -> StdResult<u32> {
        use windows::Win32::Storage::FileSystem::ReadFile;
        unsafe {
            let mut bytes_read = 0u32;
            let result = ReadFile(
                self.handle,
                Some(buffer),
                Some(&mut bytes_read),
                None,
            );
            
            if result.is_ok() {
                Ok(bytes_read)
            } else {
                Err(format!("Failed to read file: {:?}", GetLastError()))
            }
        }
    }
}

impl Drop for FileHandle {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self.handle);
        }
    }
}

/// Token operations helper
pub struct TokenHandle {
    handle: HANDLE,
}

impl TokenHandle {
    /// Open process token
    pub fn open_process_token(process: HANDLE, desired_access: u32) -> StdResult<Self> {
        use windows::Win32::Security::*;

        unsafe {
            let mut token = HANDLE::default();
            let result = OpenProcessToken(
                process,
                TOKEN_ACCESS_MASK(desired_access),
                &mut token,
            );

            if result.is_ok() {
                Ok(TokenHandle { handle: token })
            } else {
                Err(format!("Failed to open process token: {:?}", GetLastError()))
            }
        }
    }

    /// Duplicate token
    pub fn duplicate_token(&self, impersonation_level: u32) -> StdResult<Self> {
        use windows::Win32::Security::*;

        unsafe {
            let mut new_token = HANDLE::default();
            let result = DuplicateTokenEx(
                self.handle,
                TOKEN_ALL_ACCESS,
                None,
                SECURITY_IMPERSONATION_LEVEL(impersonation_level as i32),
                TokenPrimary,
                &mut new_token,
            );

            if result.is_ok() {
                Ok(TokenHandle { handle: new_token })
            } else {
                Err(format!("Failed to duplicate token: {:?}", GetLastError()))
            }
        }
    }

    /// Get token handle
    pub fn handle(&self) -> HANDLE {
        self.handle
    }
}

impl Drop for TokenHandle {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self.handle);
        }
    }
}

/// Create process as user with token
pub fn create_process_as_user(
    token: HANDLE,
    application_name: Option<&[u16]>,
    command_line: Option<&mut [u16]>,
    current_directory: Option<&[u16]>,
    session_id: u32,
) -> StdResult<PROCESS_INFORMATION> {
    use windows::Win32::Security::*;

    unsafe {
        // Set token session ID
        let session = session_id;
        let result = SetTokenInformation(
            token,
            TokenSessionId,
            &session as *const _ as *const _,
            std::mem::size_of::<u32>() as u32,
        );

        if result.is_err() {
            return Err(format!("Failed to set token session ID: {:?}", GetLastError()));
        }

        let si = STARTUPINFOW {
            cb: std::mem::size_of::<STARTUPINFOW>() as u32,
            lpDesktop: PWSTR(std::ptr::null_mut()),
            ..Default::default()
        };

        let mut pi = PROCESS_INFORMATION::default();

        let app_name_ptr = application_name.map(|s| PCWSTR(s.as_ptr())).unwrap_or(PCWSTR(std::ptr::null()));
        let cmd_line_ptr = command_line.map(|s| PWSTR(s.as_mut_ptr())).unwrap_or(PWSTR(std::ptr::null_mut()));
        let cur_dir_ptr = current_directory.map(|s| PCWSTR(s.as_ptr())).unwrap_or(PCWSTR(std::ptr::null()));

        let result = CreateProcessAsUserW(
            token,
            app_name_ptr,
            cmd_line_ptr,
            None,
            None,
            false,
            CREATE_NEW_CONSOLE,
            None,
            cur_dir_ptr,
            &si,
            &mut pi,
        );

        if result.is_ok() {
            Ok(pi)
        } else {
            Err(format!("CreateProcessAsUser failed: {:?}", GetLastError()))
        }
    }
}

/// Get active console session ID
pub fn get_active_console_session_id() -> u32 {
    use windows::Win32::System::RemoteDesktop::*;

    unsafe {
        WTSGetActiveConsoleSessionId()
    }
}

/// Enable privilege in token
pub fn enable_privilege(token: HANDLE, privilege_name: &str) -> StdResult<()> {
    use windows::Win32::Security::*;

    unsafe {
        let mut luid = LUID::default();
        let privilege_name_wide = crate::to_wide_string(privilege_name);

        if LookupPrivilegeValueW(
            None,
            PCWSTR(privilege_name_wide.as_ptr()),
            &mut luid,
        ).is_err() {
            return Err(format!("Failed to lookup privilege: {:?}", GetLastError()));
        }

        let tp = TOKEN_PRIVILEGES {
            PrivilegeCount: 1,
            Privileges: [LUID_AND_ATTRIBUTES {
                Luid: luid,
                Attributes: SE_PRIVILEGE_ENABLED,
            }],
        };

        if AdjustTokenPrivileges(
            token,
            false,
            Some(&tp),
            0,
            None,
            None,
        ).is_err() {
            return Err(format!("Failed to adjust token privileges: {:?}", GetLastError()));
        }

        Ok(())
    }
}

