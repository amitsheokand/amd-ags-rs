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

#[repr(C)]
pub struct AGSDeviceInfo {
    pub adapter_string: *const i8, // const char* in C/C++ is represented as *const i8 in Rust
    pub asic_family: AsicFamily,
    pub is_apu: u32, // bitfields are not directly supported in Rust, use u32 and bitwise operations
    pub is_primary_device: u32,
    pub is_external: u32,
    pub reserved_padding: u32,

    pub vendor_id: i32,
    pub device_id: i32,
    pub revision_id: i32,

    pub num_cus: i32,
    pub num_wgps: i32,

    pub num_rops: i32,
    pub core_clock: i32,
    pub memory_clock: i32,
    pub memory_bandwidth: i32,
    pub tera_flops: f32,

    pub local_memory_in_bytes: u64,
    pub shared_memory_in_bytes: u64,

    pub num_displays: i32,
    pub displays: *mut AGSDisplayInfo,

    pub eyefinity_enabled: i32,
    pub eyefinity_grid_width: i32,
    pub eyefinity_grid_height: i32,
    pub eyefinity_resolution_x: i32,
    pub eyefinity_resolution_y: i32,
    pub eyefinity_bezel_compensated: i32,

    pub adl_adapter_index: i32,
    pub reserved: i32,
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum AsicFamily {
    AsicFamilyUnknown,
    AsicFamilyPreGCN,
    AsicFamilyGCN1,
    AsicFamilyGCN2,
    AsicFamilyGCN3,
    AsicFamilyGCN4,
    AsicFamilyVega,
    AsicFamilyRDNA,
    AsicFamilyRDNA2,
    AsicFamilyRDNA3,
    AsicFamilyCount,
}

pub type AGSAllocCallback = Option<unsafe extern "stdcall" fn(size: usize) -> *mut std::ffi::c_void>;
pub type AGSFreeCallback = Option<unsafe extern "stdcall" fn(ptr: *mut std::ffi::c_void)>;

#[repr(C)]
pub struct AGSConfiguration {
    pub alloc_callback: AGSAllocCallback, // Optional memory allocation callback. If not supplied, malloc() is used
    pub free_callback: AGSFreeCallback,   // Optional memory freeing callback. If not supplied, free() is used
}

#[repr(C)]
pub struct AGSGPUInfo {
    pub driver_version: *const i8, // const char* in C/C++ is represented as *const i8 in Rust
    pub radeon_software_version: *const i8, // const char* in C/C++ is represented as *const i8 in Rust

    pub num_devices: i32, // int in C/C++ is represented as i32 in Rust
    pub devices: *mut AGSDeviceInfo, // AGSDeviceInfo* in C/C++ is represented as *mut AGSDeviceInfo in Rust
}

// C function signature:
// AMD_AGS_API AGSReturnCode agsInitialize( int agsVersion, const AGSConfiguration* config, AGSContext** context, AGSGPUInfo* gpuInfo );
type AGSInitialize = unsafe extern "C" fn(c_int, *const AGSConfiguration, *mut *mut AGSContext, *mut AGSGPUInfo) -> AGSReturnCode;
type AGSDeInitialize = unsafe extern "C" fn(*mut AGSContext) -> AGSReturnCode;

// Define the extension trait
trait U64Extensions {
    fn to_giga_bytes(&self) -> f64;
}

// Implement the extension trait for u64
impl U64Extensions for u64 {
    fn to_giga_bytes(&self) -> f64 {
        *self as f64 / 1_073_741_824.0 // 1 GB = 2^30 bytes
    }
}


fn main() {
    let lib = unsafe {
        lib::Library::new("AGS_SDK/ags_lib/lib/amd_ags_x64.dll").unwrap()
    };

    unsafe {
        let ags_initialize: lib::Symbol<AGSInitialize> = lib.get(b"agsInitialize").unwrap();
        let ags_deinitialize: lib::Symbol<AGSDeInitialize> = lib.get(b"agsDeInitialize").unwrap();


        let mut context: *mut AGSContext = ptr::null_mut();
        let mut gpu_info: AGSGPUInfo = std::mem::zeroed();
        let mut config: AGSConfiguration = std::mem::zeroed();

        let result = ags_initialize(AGS_CURRENT_VERSION as c_int, &config, &mut context, &mut gpu_info);

        println!("AGS result: {:?}", result);

        if result == AGSReturnCode::AGSSuccess {
            println!("AGS initialized successfully");

            // print all the display info
            println!("Device Info: {:?}", gpu_info.num_devices);
            println!("Device Info: {:?}", gpu_info.devices);
            println!("Device Info: {:?}", gpu_info.radeon_software_version);


            for i in 0..gpu_info.num_devices {
                let device = &(*gpu_info.devices.offset(i as isize));
                println!("adapter_string: {:?}", device.adapter_string);
                println!("asic_family: {:?}", device.asic_family);
                println!("is_apu: {:?}", device.is_apu);
                println!("is_primary_device: {:?}", device.is_primary_device);
                println!("is_external: {:?}", device.is_external);
                println!("vendor_id: {:?}", device.vendor_id);
                println!("device_id: {:?}", device.device_id);
                println!("revision_id: {:?}", device.revision_id);
                println!("num_cus: {:?}", device.num_cus);
                println!("num_wgps: {:?}", device.num_wgps);
                println!("num_rops: {:?}", device.num_rops);
                println!("core_clock: {:?}", device.core_clock);
                println!("memory_clock: {:?}", device.memory_clock);
                println!("memory_bandwidth: {:?}", device.memory_bandwidth);
                println!("tera_flops: {:?}", device.tera_flops);
                println!("local_memory_in_bytes: {:?}", device.local_memory_in_bytes);
                println!("shared_memory_in_bytes: {:?}", device.shared_memory_in_bytes.to_giga_bytes());
                println!("num_displays: {:?}", device.num_displays);
                println!("displays: {:?}", device.displays);
                println!("eyefinity enabled: {:?}", device.eyefinity_enabled);
                println!("eyefinity_grid_width: {:?}", device.eyefinity_grid_width);
                println!("eyefinity grid height: {:?}", device.eyefinity_grid_height);
                println!("eyefinity resolution x: {:?}", device.eyefinity_resolution_x);
                println!("eyefinity resolution y: {:?}", device.eyefinity_resolution_y);
                println!("eyefinity bezel compensated: {:?}", device.eyefinity_bezel_compensated);
                println!("adl_adapter_index: {:?}", device.adl_adapter_index);
                println!("reserved: {:?}", device.reserved);

                println!("------------------------------------");

                println!(
                    "local memory: {} MBs ({:.1} GB/s), shared memory: {} MBs\n\n",
                    (device.local_memory_in_bytes / (1024 * 1024)) as i32,
                    device.memory_bandwidth as f32 / 1024.0,
                    (device.shared_memory_in_bytes / (1024 * 1024)) as i32
                );
            }



            ags_deinitialize(context);
        } else {
            println!("AGS initialization failed");
        }
    }
}