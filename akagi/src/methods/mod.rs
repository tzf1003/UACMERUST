// UAC Bypass Methods - Module Organization
// Port of Source/Akagi/methods/

pub mod hybrids;      // Hybrid methods (SXS, DISM, Junction, etc.)
pub mod tyranid;      // Token manipulation methods
pub mod hakril;       // Hakril's methods
pub mod api0cradle;   // API0Cradle's methods
pub mod azagarampur;  // Azagarampur's methods
pub mod dwells;       // Dwells' methods
pub mod rinn;         // Rinn's methods
pub mod zcgonvh;      // Zcgonvh's methods
pub mod antonio_coco; // Antonio Coco's methods
pub mod wusa;         // WUSA-based methods
pub mod shellsup;     // Shell-based methods
pub mod comsup;       // COM-based methods

use crate::context::*;
use shared::*;

/// Method parameters block
pub struct MethodParams {
    pub method: UcmMethod,
    pub payload_code: Option<Vec<u8>>,
}

/// Method result type
pub type MethodResult = i32;

/// Method function signature
pub type MethodFn = fn(&mut UacmeContext, &MethodParams) -> MethodResult;

/// Method availability range
pub struct MethodAvailability {
    pub min_build: u32,
    pub max_build: u32,
}

/// Method dispatch entry
pub struct MethodDispatchEntry {
    pub routine: MethodFn,
    pub availability: MethodAvailability,
    pub payload_id: u32,
    pub win32_or_wow64_required: bool,
    pub disallow_wow64: bool,
    pub set_parameters: bool,
}

/// Get dispatch entry for method
pub fn get_dispatch_entry(method: UcmMethod) -> Option<MethodDispatchEntry> {
    use constants::*;
    
    match method {
        // Method 0: Test
        UcmMethod::Test => Some(MethodDispatchEntry {
            routine: method_test,
            availability: MethodAvailability { min_build: NT_WIN7_RTM, max_build: u32::MAX },
            payload_id: FUBUKI_ID,
            win32_or_wow64_required: false,
            disallow_wow64: true,
            set_parameters: true,
        }),
        
        // Methods 1-21: Deprecated (removed in v3.5.0)
        UcmMethod::Sysprep1 | UcmMethod::Sysprep2 | UcmMethod::Oobe | UcmMethod::RedirectExe |
        UcmMethod::Simda | UcmMethod::Carberp1 | UcmMethod::Carberp2 | UcmMethod::Tilon |
        UcmMethod::AVrf | UcmMethod::Winsat | UcmMethod::ShimPatch | UcmMethod::Sysprep3 |
        UcmMethod::MMC1 | UcmMethod::Sirefef | UcmMethod::Generic | UcmMethod::GWX |
        UcmMethod::Sysprep4 | UcmMethod::Manifest | UcmMethod::InetMgr | UcmMethod::MMC2 |
        UcmMethod::SXS => Some(MethodDispatchEntry {
            routine: method_deprecated,
            availability: MethodAvailability { min_build: NT_WIN7_RTM, max_build: u32::MAX },
            payload_id: PAYLOAD_ID_NONE,
            win32_or_wow64_required: false,
            disallow_wow64: false,
            set_parameters: false,
        }),
        
        // Method 22: SXS Consent
        UcmMethod::SXSConsent => Some(MethodDispatchEntry {
            routine: hybrids::method_sxs_consent,
            availability: MethodAvailability { min_build: NT_WIN7_RTM, max_build: NT_WIN10_REDSTONE1 },
            payload_id: FUBUKI_ID,
            win32_or_wow64_required: false,
            disallow_wow64: true,
            set_parameters: true,
        }),
        
        // Method 23: DISM
        UcmMethod::DISM => Some(MethodDispatchEntry {
            routine: hybrids::method_dism,
            availability: MethodAvailability { min_build: NT_WIN7_RTM, max_build: NT_WIN10_REDSTONE1 },
            payload_id: FUBUKI_ID,
            win32_or_wow64_required: false,
            disallow_wow64: true,
            set_parameters: true,
        }),
        
        // Methods 24-29: Deprecated
        UcmMethod::Comet | UcmMethod::Enigma0x3 | UcmMethod::Enigma0x3_2 | 
        UcmMethod::ExpLife | UcmMethod::Sandworm | UcmMethod::Enigma0x3_3 => Some(MethodDispatchEntry {
            routine: method_deprecated,
            availability: MethodAvailability { min_build: NT_WIN7_RTM, max_build: u32::MAX },
            payload_id: PAYLOAD_ID_NONE,
            win32_or_wow64_required: false,
            disallow_wow64: false,
            set_parameters: false,
        }),
        
        // Method 30: WOW64 Logger
        UcmMethod::Wow64Logger => Some(MethodDispatchEntry {
            routine: tyranid::method_wow64_logger,
            availability: MethodAvailability { min_build: NT_WIN7_RTM, max_build: u32::MAX },
            payload_id: constants::AKATSUKI_ID,
            win32_or_wow64_required: false,
            disallow_wow64: true,
            set_parameters: true,
        }),
        
        // Method 31: Deprecated
        UcmMethod::Enigma0x3_4 => Some(MethodDispatchEntry {
            routine: method_deprecated,
            availability: MethodAvailability { min_build: NT_WIN7_RTM, max_build: u32::MAX },
            payload_id: PAYLOAD_ID_NONE,
            win32_or_wow64_required: false,
            disallow_wow64: false,
            set_parameters: false,
        }),
        
        // Method 32: UI Access
        UcmMethod::UiAccess => Some(MethodDispatchEntry {
            routine: hybrids::method_ui_access,
            availability: MethodAvailability { min_build: NT_WIN7_RTM, max_build: u32::MAX },
            payload_id: FUBUKI_ID,
            win32_or_wow64_required: false,
            disallow_wow64: false,
            set_parameters: true,
        }),
        
        // Method 33: MS Settings
        UcmMethod::MsSettings => Some(MethodDispatchEntry {
            routine: api0cradle::method_ms_settings,
            availability: MethodAvailability { min_build: NT_WIN10_THRESHOLD1, max_build: u32::MAX },
            payload_id: PAYLOAD_ID_NONE,
            win32_or_wow64_required: false,
            disallow_wow64: false,
            set_parameters: false,
        }),
        
        // Method 34: Disk Silent Cleanup
        UcmMethod::DiskSilentCleanup => Some(MethodDispatchEntry {
            routine: api0cradle::method_disk_cleanup,
            availability: MethodAvailability { min_build: NT_WIN7_RTM, max_build: u32::MAX },
            payload_id: PAYLOAD_ID_NONE,
            win32_or_wow64_required: false,
            disallow_wow64: false,
            set_parameters: false,
        }),
        
        // Method 35: Token Modification
        UcmMethod::TokenMod => Some(MethodDispatchEntry {
            routine: method_deprecated,
            availability: MethodAvailability { min_build: NT_WIN7_RTM, max_build: u32::MAX },
            payload_id: PAYLOAD_ID_NONE,
            win32_or_wow64_required: false,
            disallow_wow64: false,
            set_parameters: false,
        }),
        
        // Method 36: Junction
        UcmMethod::Junction => Some(MethodDispatchEntry {
            routine: hybrids::method_junction,
            availability: MethodAvailability { min_build: NT_WIN7_RTM, max_build: u32::MAX },
            payload_id: FUBUKI_ID,
            win32_or_wow64_required: false,
            disallow_wow64: true,
            set_parameters: true,
        }),
        
        // Method 37: SXS Dccw
        UcmMethod::SXSDccw => Some(MethodDispatchEntry {
            routine: hybrids::method_sxs_dccw,
            availability: MethodAvailability { min_build: NT_WIN7_RTM, max_build: u32::MAX },
            payload_id: FUBUKI_ID,
            win32_or_wow64_required: false,
            disallow_wow64: true,
            set_parameters: true,
        }),
        
        // Method 38: Hakril
        UcmMethod::Hakril => Some(MethodDispatchEntry {
            routine: hakril::method_hakril,
            availability: MethodAvailability { min_build: NT_WIN7_RTM, max_build: u32::MAX },
            payload_id: FUBUKI_ID,
            win32_or_wow64_required: false,
            disallow_wow64: false,
            set_parameters: true,
        }),
        
        // Method 39: Cor Profiler
        UcmMethod::CorProfiler => Some(MethodDispatchEntry {
            routine: api0cradle::method_cor_profiler,
            availability: MethodAvailability { min_build: NT_WIN7_RTM, max_build: u32::MAX },
            payload_id: FUBUKI_ID,
            win32_or_wow64_required: false,
            disallow_wow64: true,
            set_parameters: true,
        }),
        
        // Method 40: COM Handlers (deprecated)
        UcmMethod::COMHandlers => Some(MethodDispatchEntry {
            routine: method_deprecated,
            availability: MethodAvailability { min_build: NT_WIN7_RTM, max_build: u32::MAX },
            payload_id: PAYLOAD_ID_NONE,
            win32_or_wow64_required: false,
            disallow_wow64: false,
            set_parameters: false,
        }),

        // Method 41: CMLuaUtil
        UcmMethod::CMLuaUtil => Some(MethodDispatchEntry {
            routine: api0cradle::method_cmluautil,
            availability: MethodAvailability { min_build: NT_WIN7_RTM, max_build: u32::MAX },
            payload_id: PAYLOAD_ID_NONE,
            win32_or_wow64_required: false,
            disallow_wow64: true,
            set_parameters: false,
        }),

        // Method 42: FwCplLua (deprecated)
        UcmMethod::FwCplLua => Some(MethodDispatchEntry {
            routine: method_deprecated,
            availability: MethodAvailability { min_build: NT_WIN7_RTM, max_build: u32::MAX },
            payload_id: PAYLOAD_ID_NONE,
            win32_or_wow64_required: false,
            disallow_wow64: false,
            set_parameters: false,
        }),

        // Method 43: Dccw COM
        UcmMethod::DccwCOM => Some(MethodDispatchEntry {
            routine: api0cradle::method_dccw_com,
            availability: MethodAvailability { min_build: NT_WIN7_RTM, max_build: u32::MAX },
            payload_id: PAYLOAD_ID_NONE,
            win32_or_wow64_required: false,
            disallow_wow64: false,
            set_parameters: false,
        }),

        // Methods 44-51: Various deprecated methods
        UcmMethod::VolatileEnv | UcmMethod::SluiHijack | UcmMethod::BitlockerRC |
        UcmMethod::COMHandlers2 | UcmMethod::SPPLUAObject | UcmMethod::CreateNewLink |
        UcmMethod::DateTimeWriter | UcmMethod::AcCplAdmin => Some(MethodDispatchEntry {
            routine: method_deprecated,
            availability: MethodAvailability { min_build: NT_WIN7_RTM, max_build: u32::MAX },
            payload_id: PAYLOAD_ID_NONE,
            win32_or_wow64_required: false,
            disallow_wow64: false,
            set_parameters: false,
        }),

        // Method 52: Directory Mock
        UcmMethod::DirectoryMock => Some(MethodDispatchEntry {
            routine: azagarampur::method_directory_mock,
            availability: MethodAvailability { min_build: NT_WIN7_RTM, max_build: u32::MAX },
            payload_id: FUBUKI_ID,
            win32_or_wow64_required: false,
            disallow_wow64: true,
            set_parameters: true,
        }),

        // Method 53: Shell Sdclt
        UcmMethod::ShellSdclt => Some(MethodDispatchEntry {
            routine: azagarampur::method_shell_sdclt,
            availability: MethodAvailability { min_build: NT_WIN10_THRESHOLD1, max_build: u32::MAX },
            payload_id: PAYLOAD_ID_NONE,
            win32_or_wow64_required: false,
            disallow_wow64: false,
            set_parameters: false,
        }),

        // Method 54: Egre55 (deprecated)
        UcmMethod::Egre55 => Some(MethodDispatchEntry {
            routine: method_deprecated,
            availability: MethodAvailability { min_build: NT_WIN7_RTM, max_build: u32::MAX },
            payload_id: PAYLOAD_ID_NONE,
            win32_or_wow64_required: false,
            disallow_wow64: false,
            set_parameters: false,
        }),

        // Method 55: Token Mod UI Access
        UcmMethod::TokenModUiAccess => Some(MethodDispatchEntry {
            routine: tyranid::method_token_mod_ui_access,
            availability: MethodAvailability { min_build: NT_WIN10_19H1, max_build: u32::MAX },
            payload_id: FUBUKI_ID,
            win32_or_wow64_required: false,
            disallow_wow64: true,
            set_parameters: true,
        }),

        // Method 56: Shell WSReset
        UcmMethod::ShellWSReset => Some(MethodDispatchEntry {
            routine: azagarampur::method_shell_wsreset,
            availability: MethodAvailability { min_build: NT_WIN10_THRESHOLD1, max_build: u32::MAX },
            payload_id: PAYLOAD_ID_NONE,
            win32_or_wow64_required: false,
            disallow_wow64: false,
            set_parameters: false,
        }),

        // Method 57: Sysprep5 (deprecated)
        UcmMethod::Sysprep5 => Some(MethodDispatchEntry {
            routine: method_deprecated,
            availability: MethodAvailability { min_build: NT_WIN7_RTM, max_build: u32::MAX },
            payload_id: PAYLOAD_ID_NONE,
            win32_or_wow64_required: false,
            disallow_wow64: false,
            set_parameters: false,
        }),

        // Method 58: Edition Upgrade Manager
        UcmMethod::EditionUpgradeMgr => Some(MethodDispatchEntry {
            routine: dwells::method_edition_upgrade_mgr,
            availability: MethodAvailability { min_build: NT_WIN10_THRESHOLD1, max_build: u32::MAX },
            payload_id: FUBUKI_ID,
            win32_or_wow64_required: false,
            disallow_wow64: true,
            set_parameters: true,
        }),

        // Method 59: Debug Object
        UcmMethod::DebugObject => Some(MethodDispatchEntry {
            routine: dwells::method_debug_object,
            availability: MethodAvailability { min_build: NT_WIN7_RTM, max_build: u32::MAX },
            payload_id: FUBUKI_ID,
            win32_or_wow64_required: false,
            disallow_wow64: true,
            set_parameters: true,
        }),

        // Method 60: Glupteba (deprecated)
        UcmMethod::Glupteba => Some(MethodDispatchEntry {
            routine: method_deprecated,
            availability: MethodAvailability { min_build: NT_WIN7_RTM, max_build: u32::MAX },
            payload_id: PAYLOAD_ID_NONE,
            win32_or_wow64_required: false,
            disallow_wow64: false,
            set_parameters: false,
        }),

        // Method 61: Shell ChangePk
        UcmMethod::ShellChangePk => Some(MethodDispatchEntry {
            routine: rinn::method_shell_change_pk,
            availability: MethodAvailability { min_build: NT_WIN10_THRESHOLD1, max_build: u32::MAX },
            payload_id: PAYLOAD_ID_NONE,
            win32_or_wow64_required: false,
            disallow_wow64: false,
            set_parameters: false,
        }),

        // Method 62: MS Settings 2
        UcmMethod::MsSettings2 => Some(MethodDispatchEntry {
            routine: antonio_coco::method_ms_settings2,
            availability: MethodAvailability { min_build: NT_WIN10_THRESHOLD1, max_build: u32::MAX },
            payload_id: PAYLOAD_ID_NONE,
            win32_or_wow64_required: false,
            disallow_wow64: false,
            set_parameters: false,
        }),

        // Method 63: NIC Poison
        UcmMethod::NICPoison => Some(MethodDispatchEntry {
            routine: antonio_coco::method_nic_poison,
            availability: MethodAvailability { min_build: NT_WIN7_RTM, max_build: u32::MAX },
            payload_id: FUBUKI_ID,
            win32_or_wow64_required: false,
            disallow_wow64: true,
            set_parameters: true,
        }),

        // Method 64: IE Add-On Install
        UcmMethod::IeAddOnInstall => Some(MethodDispatchEntry {
            routine: shellsup::method_ie_addon_install,
            availability: MethodAvailability { min_build: NT_WIN7_RTM, max_build: u32::MAX },
            payload_id: PAYLOAD_ID_NONE,
            win32_or_wow64_required: false,
            disallow_wow64: false,
            set_parameters: false,
        }),

        // Method 65: WSC Action Protocol
        UcmMethod::WscActionProtocol => Some(MethodDispatchEntry {
            routine: shellsup::method_wsc_action_protocol,
            availability: MethodAvailability { min_build: NT_WIN10_THRESHOLD1, max_build: u32::MAX },
            payload_id: PAYLOAD_ID_NONE,
            win32_or_wow64_required: false,
            disallow_wow64: false,
            set_parameters: false,
        }),

        // Method 66: FwCplLua2
        UcmMethod::FwCplLua2 => Some(MethodDispatchEntry {
            routine: shellsup::method_fwcpllua2,
            availability: MethodAvailability { min_build: NT_WIN7_RTM, max_build: constants::NT_WIN11_24H2 },
            payload_id: PAYLOAD_ID_NONE,
            win32_or_wow64_required: false,
            disallow_wow64: true,
            set_parameters: false,
        }),

        // Method 67: MS Settings Protocol
        UcmMethod::MsSettingsProtocol => Some(MethodDispatchEntry {
            routine: shellsup::method_ms_settings_protocol,
            availability: MethodAvailability { min_build: NT_WIN10_THRESHOLD1, max_build: u32::MAX },
            payload_id: PAYLOAD_ID_NONE,
            win32_or_wow64_required: false,
            disallow_wow64: true,
            set_parameters: false,
        }),

        // Method 68: MS Store Protocol
        UcmMethod::MsStoreProtocol => Some(MethodDispatchEntry {
            routine: shellsup::method_ms_store_protocol,
            availability: MethodAvailability { min_build: NT_WIN10_REDSTONE5, max_build: u32::MAX },
            payload_id: PAYLOAD_ID_NONE,
            win32_or_wow64_required: false,
            disallow_wow64: true,
            set_parameters: false,
        }),

        // Method 69: PCA
        UcmMethod::Pca => Some(MethodDispatchEntry {
            routine: comsup::method_pca,
            availability: MethodAvailability { min_build: NT_WIN7_RTM, max_build: u32::MAX },
            payload_id: FUBUKI_ID,
            win32_or_wow64_required: false,
            disallow_wow64: true,
            set_parameters: true,
        }),

        // Method 70: CurVer
        UcmMethod::CurVer => Some(MethodDispatchEntry {
            routine: comsup::method_curver,
            availability: MethodAvailability { min_build: NT_WIN10_THRESHOLD1, max_build: u32::MAX },
            payload_id: PAYLOAD_ID_NONE,
            win32_or_wow64_required: false,
            disallow_wow64: false,
            set_parameters: false,
        }),

        // Method 71: NIC Poison 2
        UcmMethod::NICPoison2 => Some(MethodDispatchEntry {
            routine: antonio_coco::method_nic_poison2,
            availability: MethodAvailability { min_build: NT_WIN7_RTM, max_build: u32::MAX },
            payload_id: FUBUKI_ID,
            win32_or_wow64_required: false,
            disallow_wow64: true,
            set_parameters: true,
        }),

        // Method 72: Msdt
        UcmMethod::Msdt => Some(MethodDispatchEntry {
            routine: method_not_implemented,
            availability: MethodAvailability { min_build: NT_WIN10_THRESHOLD1, max_build: u32::MAX },
            payload_id: constants::FUBUKI32_ID,
            win32_or_wow64_required: false,
            disallow_wow64: false,
            set_parameters: true,
        }),

        // Method 73: Dot Net Serial
        UcmMethod::DotNetSerial => Some(MethodDispatchEntry {
            routine: method_not_implemented,
            availability: MethodAvailability { min_build: NT_WIN7_RTM, max_build: u32::MAX },
            payload_id: PAYLOAD_ID_NONE,
            win32_or_wow64_required: false,
            disallow_wow64: true,
            set_parameters: false,
        }),

        // Method 74: VFServer Task Scheduler
        UcmMethod::VFServerTaskSched => Some(MethodDispatchEntry {
            routine: zcgonvh::method_vfserver_task_sched,
            availability: MethodAvailability { min_build: constants::NT_WIN8_BLUE, max_build: u32::MAX },
            payload_id: constants::AKATSUKI_ID,
            win32_or_wow64_required: false,
            disallow_wow64: true,
            set_parameters: true,
        }),

        // Method 75: VFServer Diagnostic Profiler
        UcmMethod::VFServerDiagProf => Some(MethodDispatchEntry {
            routine: zcgonvh::method_vfserver_diag_prof,
            availability: MethodAvailability { min_build: NT_WIN7_RTM, max_build: u32::MAX },
            payload_id: constants::AKATSUKI_ID,
            win32_or_wow64_required: false,
            disallow_wow64: true,
            set_parameters: true,
        }),

        // Method 76: IscsiCpl
        UcmMethod::IscsiCpl => Some(MethodDispatchEntry {
            routine: method_not_implemented,
            availability: MethodAvailability { min_build: NT_WIN7_RTM, max_build: u32::MAX },
            payload_id: constants::FUBUKI32_ID,
            win32_or_wow64_required: false,
            disallow_wow64: false,
            set_parameters: true,
        }),

        // Method 77: Atl Hijack
        UcmMethod::AtlHijack => Some(MethodDispatchEntry {
            routine: method_not_implemented,
            availability: MethodAvailability { min_build: NT_WIN7_RTM, max_build: u32::MAX },
            payload_id: FUBUKI_ID,
            win32_or_wow64_required: false,
            disallow_wow64: true,
            set_parameters: true,
        }),

        // Method 78: SSPI Datagram
        UcmMethod::SspiDatagram => Some(MethodDispatchEntry {
            routine: method_not_implemented,
            availability: MethodAvailability { min_build: NT_WIN7_RTM, max_build: u32::MAX },
            payload_id: constants::AKATSUKI_ID,
            win32_or_wow64_required: false,
            disallow_wow64: true,
            set_parameters: true,
        }),

        // Method 79: TokenModUiAccess2 (was Reserved)
        UcmMethod::TokenModUiAccess2 => Some(MethodDispatchEntry {
            routine: method_deprecated,
            availability: MethodAvailability { min_build: NT_WIN7_RTM, max_build: u32::MAX },
            payload_id: PAYLOAD_ID_NONE,
            win32_or_wow64_required: false,
            disallow_wow64: false,
            set_parameters: false,
        }),

        // Method 80: Token Mod UI Access 2
        UcmMethod::TokenModUiAccess2 => Some(MethodDispatchEntry {
            routine: tyranid::method_token_mod_ui_access2,
            availability: MethodAvailability { min_build: NT_WIN10_19H1, max_build: u32::MAX },
            payload_id: FUBUKI_ID,
            win32_or_wow64_required: false,
            disallow_wow64: true,
            set_parameters: true,
        }),

        // Method 81: Request Trace
        UcmMethod::RequestTrace => Some(MethodDispatchEntry {
            routine: method_not_implemented,
            availability: MethodAvailability { min_build: constants::NT_WIN11_24H2, max_build: u32::MAX },
            payload_id: FUBUKI_ID,
            win32_or_wow64_required: false,
            disallow_wow64: true,
            set_parameters: true,
        }),

        // Method 82: Quick Assist
        UcmMethod::QuickAssist => Some(MethodDispatchEntry {
            routine: method_not_implemented,
            availability: MethodAvailability { min_build: NT_WIN10_REDSTONE5, max_build: u32::MAX },
            payload_id: FUBUKI_ID,
            win32_or_wow64_required: false,
            disallow_wow64: true,
            set_parameters: true,
        }),

        _ => None,
    }
}

/// Not implemented method stub
fn method_not_implemented(_ctx: &mut UacmeContext, _params: &MethodParams) -> MethodResult {
    log::warn!("This method is not yet implemented");
    constants::STATUS_NOT_IMPLEMENTED
}

/// Methods manager - dispatch method execution
pub fn methods_manager_call(ctx: &mut UacmeContext, method: UcmMethod) -> MethodResult {
    use crate::console;

    console::console_print_status("[*] Calling method", method as i32);

    // Check for emulator
    if anti_emulator::is_emulator_present() == constants::STATUS_NEEDS_REMEDIATION {
        console::console_print_error("Emulator detected");
        return constants::STATUS_NOT_SUPPORTED;
    }

    // Get dispatch entry
    let entry = match get_dispatch_entry(method) {
        Some(e) => e,
        None => {
            console::console_print_error("Invalid method");
            return constants::STATUS_INVALID_PARAMETER;
        }
    };

    // Check method requirements
    if !check_method_requirements(ctx, &entry) {
        console::console_print_error("Method requirements not met");
        return constants::STATUS_NOT_SUPPORTED;
    }

    // Prepare parameters
    let params = MethodParams {
        method,
        payload_code: load_payload_code(entry.payload_id),
    };

    // Execute method
    (entry.routine)(ctx, &params)
}

/// Check if method requirements are met
fn check_method_requirements(ctx: &UacmeContext, entry: &MethodDispatchEntry) -> bool {
    // Check build number range
    if ctx.build_number < entry.availability.min_build ||
       ctx.build_number > entry.availability.max_build {
        log::error!("Build number {} not in range [{}, {}]",
            ctx.build_number,
            entry.availability.min_build,
            entry.availability.max_build);
        return false;
    }

    // Check WOW64 requirements
    if entry.disallow_wow64 && ctx.is_wow64 {
        log::error!("Method does not support WOW64");
        return false;
    }

    if entry.win32_or_wow64_required && !ctx.is_wow64 {
        #[cfg(target_arch = "x86_64")]
        {
            log::error!("Method requires WOW64 or x86");
            return false;
        }
    }

    true
}

/// Load payload code by ID
fn load_payload_code(payload_id: u32) -> Option<Vec<u8>> {
    if payload_id == constants::PAYLOAD_ID_NONE {
        return None;
    }

    crate::payload_loader::load_payload(payload_id)
}

/// Deprecated method stub
fn method_deprecated(_ctx: &mut UacmeContext, _params: &MethodParams) -> MethodResult {
    constants::STATUS_NOT_SUPPORTED
}

/// Test method
fn method_test(ctx: &mut UacmeContext, _params: &MethodParams) -> MethodResult {
    println!("[*] Test method - launching payload");
    
    let payload = ctx.get_payload();
    let payload_str = from_wide_string(payload);
    
    println!("[+] Payload: {}", payload_str);
    
    // In a real implementation, this would execute the payload with elevation
    constants::STATUS_SUCCESS
}

