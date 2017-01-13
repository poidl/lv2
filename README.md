# minimal lv2 interface

A beginner's programming exercise in Rust.

LV2 docs: http://lv2plug.in/


## Note

The objective of this crate is to provide an idiomatic Rust interface to LV2.
However, I have no clue how to do this properly. See the included amplifier example and [this question](http://stackoverflow.com/questions/40944524/how-does-one-design-a-plugin-interface-for-digital-audio-workstation-hosts-in-pu) on stackoverflow.

The [lv2_raw crate](https://crates.io/crates/lv2_raw) tries to provide a 
more low-level "verbatim" (i.e. C-like) translation of the C interface.
That crate may be more useful for your project.

The original (C language) LV2 package defines contains some "helper" functions,
which are defined in C-headers. As temporary solution, this crate contains
some of those functions, although they may not correspond to idiomatic Rust.
The documentation of these functions is copied from the original C files, whose
copyright holders include Steve Harris, Lars Luthman, Gabriel M. Beddingfield,
David Robillard, Richard W.E. Furse, Paul Barton-Davis, Stefan Westerfeld, and
possibly others.


