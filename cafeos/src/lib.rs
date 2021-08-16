#![no_std]
#![feature(alloc_error_handler)]

pub mod screen;

#[macro_use]
mod runtime;

#[global_allocator]
static GLOBAL_ALLOCATOR: runtime::MEMDefaultHeapAllocator = runtime::MEMDefaultHeapAllocator;
