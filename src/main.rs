extern crate libloading as lib;

use std::ptr;
use std::os::raw::{c_int, c_void, c_uint};


pub const AMD_AGS_VERSION_MAJOR: c_int = 6;
pub const AMD_AGS_VERSION_MINOR: c_int = 2;
pub const AMD_AGS_VERSION_PATCH: c_int = 0;

#[macro_export]
macro_rules! AGS_MAKE_VERSION {
    ($major:expr, $minor:expr, $patch:expr) => {
        (($major as u32) << 22 | ($minor as u32) << 12 | $patch as u32)
    };
}

pub const AGS_UNSPECIFIED_VERSION: c_uint = 0xFFFFAD00;
pub const AGS_CURRENT_VERSION: c_uint = AGS_MAKE_VERSION!(AMD_AGS_VERSION_MAJOR, AMD_AGS_VERSION_MINOR, AMD_AGS_VERSION_PATCH);


#[repr(C)]
pub struct AGSContext {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum AGSReturnCode {
    AGSSuccess = 0,
    AGSFailure = 1,
    AGSInvalidArgs = 2,
    AGSOutOfMemory = 3,
    AGSMissingD3DDll = 4,
    AGSLegacyDriver = 5,
    AGSNoAmdDriverInstalled = 6,
    AGSExtensionNotSupported = 7,
    AGSAdlFailure = 8,
    AGSDxFailure = 9,
    AGSD3DDeviceNotCreated = 10,
}

/// The rectangle struct used by AGS.
#[repr(C)]
pub struct AGSRect {
    pub offset_x: i32,
    pub offset_y: i32,
    pub width: i32,
    pub height: i32,
}

#[repr(C)]
pub struct AGSDisplayInfo {
    pub name: [i8; 256], // char is typically represented as i8 in Rust
    pub display_device_name: [i8; 32], // char is typically represented as i8 in Rust

    pub is_primary_display: u32, // bitfields are not directly supported in Rust, use u32 and bitwise operations
    pub hdr10: u32,
    pub dolby_vision: u32,
    pub freesync: u32,
    pub freesync_hdr: u32,
    pub eyefinity_in_group: u32,
    pub eyefinity_preferred_display: u32,
    pub eyefinity_in_portrait_mode: u32,
    pub reserved_padding: u32,

    pub max_resolution_x: i32,
    pub max_resolution_y: i32,
    pub max_refresh_rate: f32,

    pub current_resolution: AGSRect,
    pub visible_resolution: AGSRect,
    pub current_refresh_rate: f32,

    pub eyefinity_grid_coord_x: i32,
    pub eyefinity_grid_coord_y: i32,

    pub chromaticity_red_x: f64,
    pub chromaticity_red_y: f64,

    pub chromaticity_green_x: f64,
    pub chromaticity_green_y: f64,

    pub chromaticity_blue_x: f64,
    pub chromaticity_blue_y: f64,

    pub chromaticity_white_point_x: f64,
    pub chromaticity_white_point_y: f64,

    pub screen_diffuse_reflectance: f64,
    pub screen_specular_reflectance: f64,

    pub min_luminance: f64,
    pub max_luminance: f64,
    pub avg_luminance: f64,

    pub logical_display_index: i32,
    pub adl_adapter_index: i32,
    pub reserved: i32,
}

type AGSInitialize = unsafe extern "C" fn(c_int, *const c_void, *mut *mut AGSContext, *mut AGSDisplayInfo) -> AGSReturnCode;
type AGSDeInitialize = unsafe extern "C" fn(*mut AGSContext) -> AGSReturnCode;

fn main() {
    let lib = unsafe {
        lib::Library::new("AGS_SDK/ags_lib/lib/amd_ags_x64.dll").unwrap()
    };

    unsafe {
        let ags_initialize: lib::Symbol<AGSInitialize> = lib.get(b"agsInitialize").unwrap();
        let ags_deinitialize: lib::Symbol<AGSDeInitialize> = lib.get(b"agsDeInitialize").unwrap();

        // C function signature:
        // AMD_AGS_API AGSReturnCode agsInitialize( int agsVersion, const AGSConfiguration* config, AGSContext** context, AGSGPUInfo* gpuInfo );


        let mut context: *mut AGSContext = ptr::null_mut();
        let result = ags_initialize(AGS_CURRENT_VERSION as c_int, ptr::null(), &mut context, ptr::null_mut());

        println!("AGS result: {:?}", result);

        if result == AGSReturnCode::AGSSuccess {
            println!("AGS initialized successfully");

            // Use the AGS context...

            ags_deinitialize(context);
        } else {
            println!("AGS initialization failed");
        }
    }
}