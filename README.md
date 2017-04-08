# minimal lv2 interface

A beginner's programming exercise in Rust.

LV2 docs: http://lv2plug.in/


## Note

The objective of this crate is to provide an idiomatic Rust interface to LV2.
However, I have no clue how to do this properly. See the included amplifier example and [this question](http://stackoverflow.com/questions/40944524/how-does-one-design-a-plugin-interface-for-digital-audio-workstation-hosts-in-pu) on stackoverflow.

The [lv2_raw crate](https://crates.io/crates/lv2_raw) tries to provide a 
more low-level "verbatim" (i.e. C-like) translation of the C interface.
That crate may be more useful for your project.

Copyright holders of the original C code include Steve Harris, Lars Luthman, Gabriel M. Beddingfield,
David Robillard, Richard W.E. Furse, Paul Barton-Davis, Stefan Westerfeld, and
possibly others.


