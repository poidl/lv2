use libc;
use lv2_raw;

pub struct LV2UIController(pub lv2_raw::LV2UIControllerRaw);

unsafe impl Send for LV2UIController {}

impl Copy for LV2UIController { }

impl Clone for LV2UIController {
    fn clone(&self) -> LV2UIController {
        *self
    }
}

pub type LV2UIWriteFunction = Option<extern "C" fn(controller: LV2UIController,
                                                      port_index: libc::c_uint,
                                                      buffer_size: libc::c_uint,
                                                      port_protocol: libc::c_uint,
                                                      buffer: *const libc::c_void)>;

#[repr(C)]
pub struct LV2UIDescriptor {
    pub uri: *const libc::c_char,
    pub instantiate: extern "C" fn(descriptor: *const LV2UIDescriptor,
                                       plugin_uri: *const libc::c_char,
                                       bundle_path: *const libc::c_char,
                                       write_function: LV2UIWriteFunction,
                                       controller: LV2UIController,
                                       widget: *mut lv2_raw::LV2UIWidget,
                                       features: *const (*const lv2_raw::LV2Feature))
                                       -> lv2_raw::LV2UIHandle,

    pub cleanup: extern "C" fn(lv2_raw::LV2UIHandle),
    pub port_event: extern "C" fn(ui: lv2_raw::LV2UIHandle,
                                      port_index: libc::c_uint,
                                      buffer_size: libc::c_uint,
                                      format: libc::c_uint,
                                      buffer: *const libc::c_void),
    pub extension_data: Option<extern "C" fn(*const libc::c_char) -> *const libc::c_void>,
}



                                                    