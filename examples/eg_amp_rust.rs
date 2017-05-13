// This is a translation of http://lv2plug.in/git/cgit.cgi/lv2.git/tree/plugins/eg-amp.lv2/amp.c
// from C into Rust by S. Riha (2015)
// Read the README.txt of the original code:
// http://lv2plug.in/git/cgit.cgi/lv2.git/tree/plugins/eg-amp.lv2/README.txt
// The copyright notice of the original C file:

// Copyright 2006-2011 David Robillard <d@drobilla.net>
// Copyright 2006 Steve Harris <steve@plugin.org.uk>
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//

// Notes:
// 2015/07/11: I just started programming in Rust and I'm rather new to low level programming in
// general (memory management, etc.). I'm sure there is a lot of room for improvement here, if
// you have any ideas please let me know (hoitaus@gmail.com)
// 2015/07/11: It is currently not possible to allocate a struct that contains a C string. See
// comment in fn lv2_descriptor().

extern crate libc;
extern crate lv2;
use std::ptr;

// This is a simple "amplifier" plugin. It holds references to input and output
// buffers, and a reference to a single number (gain) by which the input buffer
// is multiplied. Memory managemet of these resources is done by the host.

pub struct AmpNew<'a> {
    gain: &'a f32,
    input: &'a [f32],
    output: &'a mut [f32],
}

// The main question is: Should one use raw pointers instead of references in this
// struct?

// I'd say NO, because:

// I want to implement "lv2::LV2Plugin"
// without having to use "unsafe", i.e. to "provide a safe interface" to the
// lv2 library. What is the correct (let's say idiomatic) way to "provide a safe
// interface"?

// I'd say YES, because:

// *) AFAIK it's impossible to implement a constructor "::new()" function for this
// struct, since the resources go out of scope when "new()" terminates. Instead, the
// plugin is instantiated by the lv2::instantiate::<>() function using
// libc::malloc(), similar to the code I commented at the bottom of this page.
// It seems weired to not being able to allocate the plugin from safe code.
// How would one design a plugin interface in pure Rust (i.e.
// with a host written in Rust)?

// *) There are potential problems arising with unkown buffer sizes. The function
// connect_port() below takes an "&'a mut [f32]", which is constructed in the
// calling function (containing unsafe code) by "slice::from_raw_parts_mut()",
// which needs a buffer size as argument. This size is not passed by host to
// the extern "C" connect_port(). Instead,
// the host passes the "n_samples" argument to the real-time "run()" function to
// indicate the length of the buffer.
// This may not be an issue, since one can pass a really high buffer size
// to "slice::from_raw_parts_mut()" without having to worry about perfomance/space,
// since no resources are actually allocated anyways, right?

impl<'a> lv2::LV2Plugin<'a> for AmpNew<'a> {
    // For now, initialize() is a placeholder function that doesn't do anything. More complicated plugins may scan host features, set a sample rate, etc.
    fn initialize(&mut self) {}
    fn connect_port(&mut self, port: u32, data: &'a mut [f32]) {
        match port {
            0 => self.gain = &data[0] as &f32, // data may be NULL pointer, so don't dereference!
            1 => self.input = data as &'a [f32],
            2 => self.output = data,
            _ => panic!("Not a valid PortIndex: {}", port),
        }
    }
    fn activate(&mut self) {}
    fn run(&mut self, n_samples: u32) {

        let coef: f32;
        match *self.gain > -90.0 {
            true => coef = (10.0 as f32).powf(self.gain * 0.05),
            false => coef = 0.0,
        }
        for x in 0..n_samples {
            let i = x as usize;
            self.output[i] = self.input[i] * coef;
        }

    }
    fn deactivate(&mut self) {}
    fn cleanup(&mut self) {}
}

type Newtype<'a> = AmpNew<'a>;

// If I understand correctly, the lv2::LV2Descriptor struct that is delivered
// to the host by "lv2_descriptor()" CANNOT be generic over "Newtype", since this
// would require "lv2_descriptor()" to be generic. But functions called from C
// (by their name) CANNOT be generic.
// The reason why "lv2::instantiate::<>" etc. CAN be generic, is that those functions
// get passed to C via FUNCTION POINTERS contained in a #[repr(C)] struct.
// A secondary question is: Is this necessary? How to implement this more
// effectively?

static S: &'static [u8] = b"http://example.org/eg-amp-rust\0";
static mut DESC: lv2::LV2Descriptor = lv2::LV2Descriptor {
    uri: 0 as *const libc::c_char, // ptr::null() isn't const fn (yet)
    instantiate: lv2::instantiate::<Newtype>,
    connect_port: lv2::connect_port::<Newtype>,
    activate: Some(lv2::activate),
    run: lv2::run::<Newtype>,
    deactivate: Some(lv2::deactivate),
    cleanup: lv2::cleanup,
    extension_data: lv2::extension_data,
};


#[no_mangle]
pub extern "C" fn lv2_descriptor(index: i32) -> *const lv2::LV2Descriptor {
    if index != 0 {
        return ptr::null();
    } else {
        // credits to ker on stackoverflow: http://stackoverflow.com/questions/31334356/static-struct-with-c-strings-for-lv2-plugin (duplicate) or http://stackoverflow.com/questions/25880043/creating-a-static-c-struct-containing-strings
        let ptr = S.as_ptr() as *const libc::c_char;
        unsafe {
            DESC.uri = ptr;
            return &DESC as *const lv2::LV2Descriptor;
        }
    }
}

// fn instantiate<T>() -> lv2_raw::LV2Handle {
//     let ptr: lv2_raw::LV2Handle;
//     unsafe {
//         ptr = libc::malloc(mem::size_of::<T>() as libc::size_t) as lv2_raw::LV2Handle;
//         let plgptr = ptr as *mut T;
//     }
//     ptr
// }

// impl<'a> AmpNew<'a> {
//     fn new_ptr() -> lv2_raw::LV2Handle {
//         instantiate::<AmpNew>()
//     }
// }