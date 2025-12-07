#![no_std]
#![no_main]

mod webview;

#[no_mangle]
extern "system" fn mainCRTStartup() {
    webview::go()
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! { loop {} }
