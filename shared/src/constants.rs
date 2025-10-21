// Constants and definitions
// Port of Source/Shared/consts.h

pub const MAX_PATH: usize = 260;

// Akagi flags
pub const AKAGI_FLAG_KILO: u32 = 1;  // Default execution flow
pub const AKAGI_FLAG_TANGO: u32 = 2; // Suppress all additional output

// Payload IDs
pub const PAYLOAD_ID_NONE: u32 = 0;
pub const FUBUKI_ID: u32 = 1;
pub const FUBUKI32_ID: u32 = 2;
pub const AKATSUKI_ID: u32 = 3;
pub const KAMIKAZE_ID: u32 = 4;

// NT Build numbers
pub const NT_WIN7_RTM: u32 = 7600;
pub const NT_WIN7_SP1: u32 = 7601;
pub const NT_WIN8_RTM: u32 = 9200;
pub const NT_WIN81_RTM: u32 = 9600;
pub const NT_WIN10_THRESHOLD1: u32 = 10240;
pub const NT_WIN10_THRESHOLD2: u32 = 10586;
pub const NT_WIN10_REDSTONE1: u32 = 14393;
pub const NT_WIN10_REDSTONE2: u32 = 15063;
pub const NT_WIN10_REDSTONE3: u32 = 16299;
pub const NT_WIN10_REDSTONE4: u32 = 17134;
pub const NT_WIN10_REDSTONE5: u32 = 17763;
pub const NT_WIN10_19H1: u32 = 18362;
pub const NT_WIN10_19H2: u32 = 18363;
pub const NT_WIN10_20H1: u32 = 19041;
pub const NT_WIN10_20H2: u32 = 19042;
pub const NT_WIN10_21H1: u32 = 19043;
pub const NT_WIN10_21H2: u32 = 19044;
pub const NT_WIN10_22H2: u32 = 19045;
pub const NT_WIN11_21H2: u32 = 22000;
pub const NT_WIN11_22H2: u32 = 22621;
pub const NT_WIN11_23H2: u32 = 22631;
pub const NT_WIN11_24H2: u32 = 26100;

// Additional NT versions
pub const NT_WIN8_BLUE: u32 = 9600; // Windows 8.1

// Common paths
pub const SYSTEM32_DIR: &str = "\\system32\\";
pub const SYSWOW64_DIR: &str = "\\SysWOW64\\";
pub const SYSNATIVE_DIR: &str = "\\sysnative\\";

// Common executables
pub const CMD_EXE: &str = "cmd.exe";
pub const EVENTVWR_EXE: &str = "eventvwr.exe";
pub const COMPUTERDEFAULTS_EXE: &str = "ComputerDefaults.exe";
pub const SDCLT_EXE: &str = "sdclt.exe";
pub const FODHELPER_EXE: &str = "fodhelper.exe";
pub const WSRESET_EXE: &str = "WSReset.exe";
pub const OSK_EXE: &str = "osk.exe";

// Common DLLs
pub const CRYPTBASE_DLL: &str = "cryptbase.dll";
pub const SHCORE_DLL: &str = "shcore.dll";
pub const WDSCORE_DLL: &str = "wdscore.dll";

// Registry paths
pub const REG_UAC_PATH: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Policies\\System";
pub const REG_WINLOGON_PATH: &str = "Software\\Microsoft\\Windows NT\\CurrentVersion\\Winlogon";

// Shared section names
pub const AKAGI_SHARED_SECTION: &str = "AkagiSharedSection";
pub const AKAGI_COMPLETION_EVENT: &str = "AkagiCompletionEvent";

// Command line
pub const RUN_CMD_COMMAND: &str = " /c ";

// Status codes
pub const STATUS_SUCCESS: i32 = 0;
pub const STATUS_ACCESS_DENIED: i32 = -1073741790; // 0xC0000022
pub const STATUS_INVALID_PARAMETER: i32 = -1073741811; // 0xC000000D
pub const STATUS_NOT_SUPPORTED: i32 = -1073741822; // 0xC00000BB
pub const STATUS_NOT_IMPLEMENTED: i32 = -1073741822; // 0xC00000BB (same as NOT_SUPPORTED)
pub const STATUS_ELEVATION_REQUIRED: i32 = -1073740756; // 0xC000042C
pub const STATUS_INTERNAL_ERROR: i32 = -1073741595; // 0xC00000E5
pub const STATUS_FATAL_APP_EXIT: i32 = 0x40000015;
pub const STATUS_NEEDS_REMEDIATION: i32 = -1073740763; // 0xC0000425

// Wide string constants as UTF-16 arrays
pub fn w_system32() -> Vec<u16> {
    "\\system32\\".encode_utf16().chain(std::iter::once(0)).collect()
}

pub fn w_syswow64() -> Vec<u16> {
    "\\SysWOW64\\".encode_utf16().chain(std::iter::once(0)).collect()
}

pub fn w_cmd_exe() -> Vec<u16> {
    "cmd.exe".encode_utf16().chain(std::iter::once(0)).collect()
}

// Helper to create null-terminated wide string
pub fn to_wide_string(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

// Helper to convert wide string to Rust String
pub fn from_wide_string(wide: &[u16]) -> String {
    let len = wide.iter().position(|&c| c == 0).unwrap_or(wide.len());
    String::from_utf16_lossy(&wide[..len])
}

