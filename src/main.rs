#![no_std] // don't link the rust standard library
#![no_main] // disable all rust-level entry points


use core::panic::PanicInfo;

mod vga_buffer;
#[allow(dead_code)]
static MSG_CRAB: &str = r#"
_    ~^~^~_
\) /  o o  \ (/
  '_   U   _'
  \ '-----' /
"#;
static MSG_TEXT: &str = r#"
lexi's rust os
 
thank you to;
rustfoundation.org (rust) 
os.phil-opp.com (tutorials)
rustacean.net (ferris ascii art)
"#;

#[allow(dead_code)]

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Booted\n").unwrap();
    vga_buffer::WRITER.lock().write_str(MSG_CRAB).unwrap();
    vga_buffer::WRITER.lock().write_str(MSG_TEXT).unwrap(); 
    println!("TODO: add a command line");
    loop {}
}

// function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
