use std::borrow::Cow;
use std::ffi::CStr;
use std::mem::size_of;
use std::os::raw::c_char;

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

fn main() {
    let a: usize = 42;
    //let b: &[u8; 10] = &B;
    //let c: Box<[u8]> = Box::new(C);
    let b: String;
    let c: Cow<str>;

    unsafe {
        let b_ptr = &B as *const u8 as *mut u8;
        b = String::from_raw_parts(b_ptr, 10, 10);
        let c_ptr = &C as *const u8 as *const c_char;
        c = CStr::from_ptr(c_ptr).to_string_lossy();
    }

    let a_ptr = &a as *const usize;
    let a_addr: usize = unsafe { std::mem::transmute(a_ptr) };

    println!("a: {}, ({:p}...0x{:x})", a, a_ptr, a_addr + 7);
    println!("a: {}, b: {}, c: {}", a, b, c);
    println!("a (an unsigned integer):");
    println!("  location: {:p}", &a);
    println!("  size:     {:?} bytes", size_of::<usize>());
    println!("  value:    {:?}", a);
    println!();

    println!("b (a reference to B):");
    println!("  location:  {:p}", &b);
    println!("  size:      {:?} bytes", size_of::<&[u8; 10]>());
    //println!("  points to: {:p}", b);
    println!();

    println!("c (a 'box' for C):");
    println!("  location:  {:p}", &c);
    println!("  size:      {:?} bytes", size_of::<Box<[u8]>>());
    //println!("  points to: {:p}", c);
    println!();

    println!("B (an array of 10 bytes):");
    println!("  location: {:p}", &B);
    println!("  size:     {:?} bytes", size_of::<[u8; 10]>());
    println!("  value:    {:?}", B);
    println!();

    println!("C (an array of 11 bytes):");
    println!("  location: {:p}", &C);
    println!("  size:     {:?} bytes", size_of::<[u8; 11]>());
    println!("  value:    {:?}", C);
}
