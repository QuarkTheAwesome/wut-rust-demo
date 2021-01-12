#![no_std]
#![feature(alloc_error_handler)]

#[macro_use]
mod runtime;

#[global_allocator]
static GLOBAL_ALLOCATOR: runtime::MEMDefaultHeapAllocator = runtime::MEMDefaultHeapAllocator;
