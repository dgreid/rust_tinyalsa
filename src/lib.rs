extern crate libc;

#[allow(dead_code)]
mod ffi;

// PCM class
//  mmap struct that calls begin on creation and commits when it goes out of scope
//  PCM builder that takes the params and opens a pcm, cleans up on drop.

