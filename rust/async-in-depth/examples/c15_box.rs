// use std::alloc::{GlobalAlloc, Layout, System};

use jemallocator::Jemalloc;
use std::alloc::{GlobalAlloc, Layout};

struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        Jemalloc.alloc(layout)
        // let data = System.alloc(layout);
        // eprintln!("alloc: {:p}, size {}", data, layout.size());
        // data
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        Jemalloc.dealloc(ptr, layout)
    }
}
// unsafe impl GlobalAlloc for MyAllocator {
//     unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
//         // ...
//         let data = System.alloc(layout);
//         eprintln!("alloc: {:p}, size {}", data, layout.size());
//         data
//     }

//     unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
//         // ...
//         System.dealloc(ptr, layout);
//         eprintln!("dealloc: {:p}, size {}", ptr, layout.size());
//     }
// }

#[global_allocator]
static GLOBAL: MyAllocator = MyAllocator;

#[allow(dead_code)]
struct Matrix {
    data: [u8; 505],
}

impl Default for Matrix {
    fn default() -> Self {
        Self { data: [0; 505] }
    }
}

fn main() {
    let data = Box::new(Matrix::default());
    // 输出中有一个 1024 大小的内存分配，是 println! 导致的
    println!(
        "!!! allocated memory: {:p}, len: {}",
        &*data,
        std::mem::size_of::<Matrix>()
    );
}
