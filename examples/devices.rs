use std::{ffi::CStr, fmt};

use ibverbs_sys::ffi::{
    ibv_free_device_list, ibv_get_device_guid, ibv_get_device_list, ibv_get_device_name,
};

struct GUID([u8; 8]);

impl fmt::Debug for GUID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let guid = self.0;
        write!(
            f,
            "{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}",
            guid[0], guid[1], guid[2], guid[3], guid[4], guid[5], guid[6], guid[7]
        )
    }
}

impl From<u64> for GUID {
    fn from(val: u64) -> Self {
        Self(val.to_le_bytes())
    }
}

fn main() {
    let mut num_devices: i32 = -1;
    let list = unsafe { ibv_get_device_list(&mut num_devices as *mut i32) };

    let mut iter = list;
    while !unsafe { *iter }.is_null() {
        let device = unsafe { *iter };
        let name = unsafe { CStr::from_ptr(ibv_get_device_name(device)) };
        let guid = GUID::from(unsafe { ibv_get_device_guid(device) });
        println!("Device: {} ({:?})", name.to_string_lossy(), guid);
        iter = unsafe { iter.offset(1) };
    }
    println!("Number of devices: {}", num_devices);

    unsafe { ibv_free_device_list(list) };
}
