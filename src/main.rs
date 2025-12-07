#![no_std]
#![no_main]

mod webview;

#[no_mangle]
extern "system" fn mainCRTStartup() {
    webview::go(1280, 720, b"https://x.com\0")
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! { loop {} }
