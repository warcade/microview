#![no_std]
#![no_main]
#![windows_subsystem = "windows"]

mod webview;
use webview::WebView;

const fn url<const N: usize>(s: &[u8; N]) -> [u16; N] {
    let mut out = [0u16; N];
    let mut i = 0;
    while i < N - 1 {
        out[i] = s[i] as u16;
        i += 1;
    }
    out
}

static URL: [u16; 26] = url(b"https://reddit.com/r/rust\0");

#[no_mangle]
pub extern "system" fn mainCRTStartup() {
    let wv = WebView::new(800, 600);
    wv.navigate(&URL);
    wv.run();
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! { loop {} }
