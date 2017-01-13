extern crate lv2_raw;
use libc;
use std::ptr;
use std::mem;
use std::slice;

/// A group of plugin methods that are defined by the plugin and called
/// by the host.
pub trait LV2Plugin<'a> {
    /// Does everything `instantiate()` does in the C code, except allocating
    /// memory.
    fn initialize(&mut self) {}
    fn connect_port(&mut self, _port: u32, _data: &'a mut [f32]) {}
    fn activate(&mut self) {}
    fn run(&mut self, _n_samples: u32) {}
    fn deactivate(&mut self) {}
    fn cleanup(&mut self) {}
}

pub extern "C" fn instantiate<'a, T: LV2Plugin<'a>>(_descriptor: *const lv2_raw::LV2Descriptor,
                                                    _rate: f64,
                                                    _bundle_path: *const i8,
                                                    _features: *const *const lv2_raw::LV2Feature)
                                                    -> lv2_raw::LV2Handle {

    let ptr: *mut libc::c_void;
    unsafe {
        ptr = libc::malloc(mem::size_of::<T>() as libc::size_t) as *mut libc::c_void;
        let plgptr = ptr as *mut T;
        (*plgptr).initialize()
    }
    return ptr;
}

pub extern "C" fn connect_port<'a, T: LV2Plugin<'a>>(handle: lv2_raw::LV2Handle,
                                                     port: u32,
                                                     data: *mut libc::c_void) {
    let d = data as *mut f32;
    let plgptr = handle as *mut T;
    unsafe {
        // TODO: This should be sample_count. How do we get that number? During initialization? Set to some random (high) number.
        // https://www.alsa-project.org/main/index.php/FramesPeriods
        let bs: &mut [f32] = slice::from_raw_parts_mut(d, 65536 * mem::size_of::<f32>());
        (*plgptr).connect_port(port, bs)
    }
}

pub extern "C" fn activate(_instance: lv2_raw::LV2Handle) {}
pub extern "C" fn run<'a, T: LV2Plugin<'a>>(instance: lv2_raw::LV2Handle, n_samples: u32) {
    let plgptr = instance as *mut T;
    unsafe { (*plgptr).run(n_samples) }
}

pub extern "C" fn deactivate(_instance: lv2_raw::LV2Handle) {}
pub extern "C" fn cleanup(instance: lv2_raw::LV2Handle) {

    unsafe {
        // ptr::read(instance as *mut Amp); // no need for this?
        libc::free(instance as lv2_raw::LV2Handle)
    }
}
pub extern "C" fn extension_data(_uri: *const u8) -> (*const libc::c_void) {
    ptr::null()
}