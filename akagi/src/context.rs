// UACME Context Management
// Port of Source/Akagi/sup.c (context-related functions)

use shared::*;
use windows::core::PCWSTR;
use windows::Win32::Foundation::*;
use windows::Win32::System::Threading::*;
use windows::Win32::UI::WindowsAndMessaging::SW_HIDE;
use std::ptr;

/// UAC Method enumeration
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UcmMethod {
    Test = 0,
    Sysprep1 = 1,
    Sysprep2 = 2,
    Oobe = 3,
    RedirectExe = 4,
    Simda = 5,
    Carberp1 = 6,
    Carberp2 = 7,
    Tilon = 8,
    AVrf = 9,
    Winsat = 10,
    ShimPatch = 11,
    Sysprep3 = 12,
    MMC1 = 13,
    Sirefef = 14,
    Generic = 15,
    GWX = 16,
    Sysprep4 = 17,
    Manifest = 18,
    InetMgr = 19,
    MMC2 = 20,
    SXS = 21,
    SXSConsent = 22,
    DISM = 23,
    Comet = 24,
    Enigma0x3 = 25,
    Enigma0x3_2 = 26,
    ExpLife = 27,
    Sandworm = 28,
    Enigma0x3_3 = 29,
    Wow64Logger = 30,
    Enigma0x3_4 = 31,
    UiAccess = 32,
    MsSettings = 33,
    DiskSilentCleanup = 34,
    TokenMod = 35,
    Junction = 36,
    SXSDccw = 37,
    Hakril = 38,
    CorProfiler = 39,
    COMHandlers = 40,
    CMLuaUtil = 41,
    FwCplLua = 42,
    DccwCOM = 43,
    VolatileEnv = 44,
    SluiHijack = 45,
    BitlockerRC = 46,
    COMHandlers2 = 47,
    SPPLUAObject = 48,
    CreateNewLink = 49,
    DateTimeWriter = 50,
    AcCplAdmin = 51,
    DirectoryMock = 52,
    ShellSdclt = 53,
    Egre55 = 54,
    TokenModUiAccess = 55,
    ShellWSReset = 56,
    Sysprep5 = 57,
    EditionUpgradeMgr = 58,
    DebugObject = 59,
    Glupteba = 60,
    ShellChangePk = 61,
    MsSettings2 = 62,
    NICPoison = 63,
    IeAddOnInstall = 64,
    WscActionProtocol = 65,
    FwCplLua2 = 66,
    MsSettingsProtocol = 67,
    MsStoreProtocol = 68,
    Pca = 69,
    CurVer = 70,
    NICPoison2 = 71,
    Msdt = 72,
    DotNetSerial = 73,
    VFServerTaskSched = 74,
    VFServerDiagProf = 75,
    IscsiCpl = 76,
    AtlHijack = 77,
    SspiDatagram = 78,
    TokenModUiAccess2 = 79,
    RequestTrace = 80,
    QuickAssist = 81,
    Max = 82,
}

impl UcmMethod {
    pub fn from_u32(value: u32) -> Option<Self> {
        if value < Self::Max as u32 {
            Some(unsafe { std::mem::transmute(value) })
        } else {
            None
        }
    }
}

/// Shared context for inter-process communication
pub struct SharedContext {
    pub h_isolated_namespace: HANDLE,
    pub h_shared_section: HANDLE,
    pub h_completion_event: HANDLE,
}

impl Default for SharedContext {
    fn default() -> Self {
        Self {
            h_isolated_namespace: HANDLE::default(),
            h_shared_section: HANDLE::default(),
            h_completion_event: HANDLE::default(),
        }
    }
}

/// Main UACME context
pub struct UacmeContext {
    pub is_wow64: bool,
    pub cookie: u32,
    pub build_number: u32,
    pub akagi_flag: u32,
    pub optional_parameter_length: u32,
    
    pub shared_context: SharedContext,
    
    pub system_root: Vec<u16>,
    pub system_directory: Vec<u16>,
    pub temp_directory: Vec<u16>,
    pub current_directory: Vec<u16>,
    pub optional_parameter: Vec<u16>,
    pub default_payload: Vec<u16>,
}

impl UacmeContext {
    /// Create new UACME context
    pub fn new(method: UcmMethod, optional_param: Option<String>) -> Result<Self, String> {
        let is_wow64 = is_process_32bit(current_process());
        let build_number = get_windows_build_number();
        
        // Get system directories
        let system_root = get_system_root();
        let system_directory = query_system_directory(true);
        let temp_directory = get_temp_directory();
        let current_directory = get_current_directory_wide();
        
        // Process optional parameter
        let (optional_parameter, optional_parameter_length) = if let Some(param) = optional_param {
            let wide_param = to_wide_string(&param);
            let len = wide_param.len() - 1; // Exclude null terminator
            (wide_param, len as u32)
        } else {
            (vec![0u16], 0)
        };
        
        // Build default payload path (system32\cmd.exe)
        let mut default_payload = system_directory.clone();
        let cmd_exe = to_wide_string("cmd.exe");
        wstrcat(default_payload.as_mut_ptr(), cmd_exe.as_ptr());
        
        Ok(Self {
            is_wow64,
            cookie: 0,
            build_number,
            akagi_flag: constants::AKAGI_FLAG_KILO,
            optional_parameter_length,
            shared_context: SharedContext::default(),
            system_root,
            system_directory,
            temp_directory,
            current_directory,
            optional_parameter,
            default_payload,
        })
    }
    
    /// Get payload to execute
    pub fn get_payload(&self) -> &[u16] {
        if self.optional_parameter_length > 0 {
            &self.optional_parameter
        } else {
            &self.default_payload
        }
    }
}

/// Get system root directory
fn get_system_root() -> Vec<u16> {
    let mut buffer = vec![0u16; MAX_PATH as usize];
    unsafe {
        let len = windows::Win32::System::SystemInformation::GetSystemWindowsDirectoryW(
            Some(&mut buffer)
        );
        if len > 0 {
            buffer.truncate((len + 1) as usize);
            // Ensure trailing backslash
            if buffer[len as usize - 1] != '\\' as u16 {
                buffer.insert(len as usize, '\\' as u16);
                buffer.push(0);
            }
        }
    }
    buffer
}

/// Get temp directory
fn get_temp_directory() -> Vec<u16> {
    let mut buffer = vec![0u16; MAX_PATH as usize];
    unsafe {
        let len = windows::Win32::Storage::FileSystem::GetTempPathW(Some(&mut buffer));
        if len > 0 {
            buffer.truncate((len + 1) as usize);
        }
    }
    buffer
}

/// Get current directory
fn get_current_directory_wide() -> Vec<u16> {
    let mut buffer = vec![0u16; MAX_PATH as usize];
    unsafe {
        use windows::Win32::System::Environment::GetCurrentDirectoryW;
        let len = GetCurrentDirectoryW(Some(&mut buffer));
        if len > 0 {
            buffer.truncate((len + 1) as usize);
            // Ensure trailing backslash
            if buffer[len as usize - 1] != '\\' as u16 {
                buffer.insert(len as usize, '\\' as u16);
                buffer.push(0);
            }
        }
    }
    buffer
}

/// Shared parameter block for payload communication
#[repr(C)]
pub struct UacmeParamBlock {
    pub crc32: u32,
    pub session_id: u32,
    pub akagi_flag: u32,
    pub parameter: [u16; MAX_PATH as usize + 1],
    pub desktop: [u16; MAX_PATH as usize + 1],
    pub winstation: [u16; MAX_PATH as usize + 1],
    pub signal_object: [u16; MAX_PATH as usize + 1],
}

impl Default for UacmeParamBlock {
    fn default() -> Self {
        Self {
            crc32: 0,
            session_id: 0,
            akagi_flag: 0,
            parameter: [0; MAX_PATH as usize + 1],
            desktop: [0; MAX_PATH as usize + 1],
            winstation: [0; MAX_PATH as usize + 1],
            signal_object: [0; MAX_PATH as usize + 1],
        }
    }
}

