extern crate lv2_raw;
use std::mem;

/** Pad a size to 64 bits. */
pub fn lv2_atom_pad_size(size: u32) -> (u32) {
    return (size + 7u32) & (!7u32);
}

/** Get an iterator pointing to the first event in a Sequence body. */
pub fn lv2_atom_sequence_begin(body: *const lv2_raw::LV2_Atom_Sequence_Body)
                               -> (*const lv2_raw::Lv2AtomEvent) {
    unsafe { return body.offset(1) as *const lv2_raw::Lv2AtomEvent }
}

/** Return an iterator to the element following `i`. */
pub fn lv2_atom_sequence_next(i: *const lv2_raw::Lv2AtomEvent) -> (*const lv2_raw::Lv2AtomEvent) {
    unsafe {
        let addr_of_first_byte = i as *const u8;
        let size_in_bytes_1 = mem::size_of::<lv2_raw::Lv2AtomEvent>() as isize;
        let size_in_bytes_2 = lv2_atom_pad_size((*i).body.size) as isize;
        let j = addr_of_first_byte.offset(size_in_bytes_1 + size_in_bytes_2);
        return j as *const lv2_raw::Lv2AtomEvent;
    }
}

/** Return true iff `i` has reached the end of `body`. */
pub fn lv2_atom_sequence_is_end(body: *const lv2_raw::LV2_Atom_Sequence_Body,
                                size: u32,
                                i: *const lv2_raw::Lv2AtomEvent)
                                -> (bool) {
    let addr_of_first_byte = body as *const u8;
    unsafe { return (i as *const u8) >= addr_of_first_byte.offset(size as isize) }
}
