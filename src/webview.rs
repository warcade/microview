#![allow(non_snake_case)]

use core::ffi::c_void;

type H = *mut c_void;
type P = *mut c_void;
const N: P = 0 as P;

#[link(name = "kernel32")]
extern "system" {
    fn LoadLibraryExA(n: *const u8, f: P, flags: u32) -> H;
    fn GetProcAddress(m: H, n: *const u8) -> P;
    fn ExitProcess(c: u32);
}

#[link(name = "advapi32")]
extern "system" {
    fn RegGetValueA(k: H, sub: *const u8, val: *const u8, flags: u32, typ: *mut u32, data: P, len: *mut u32) -> i32;
}

#[link(name = "user32")]
extern "system" {
    fn RegisterClassExA(wc: *const WCA) -> u16;
    fn CreateWindowExA(ex: u32, cls: *const u8, t: *const u8, st: u32, x: i32, y: i32, w: i32, h: i32, p: H, m: P, i: P, l: P) -> H;
    fn DefWindowProcA(h: H, m: u32, w: usize, l: isize) -> isize;
    fn GetMessageA(m: *mut MSG, h: H, a: u32, b: u32) -> i32;
    fn DispatchMessageA(m: *const MSG) -> isize;
}

#[repr(C)] struct MSG { _: [usize; 7] }
#[repr(C)] struct WCA { sz: u32, st: u32, wnd: unsafe extern "system" fn(H, u32, usize, isize) -> isize, _a: [P; 6], h: *const u8, _i: P }

static mut GH: H = N;
static mut GW: P = N;

static VTB: [P; 4] = [vq as P, va as P, va as P, vi as P];
static mut VB: *const P = &VTB as *const _ as _;

unsafe extern "system" fn vq(t: P, _: *const c_void, p: *mut P) -> i32 { *p = t; 0 }
unsafe extern "system" fn va(_: P) -> u32 { 1 }

unsafe extern "system" fn vi(_: P, _: i32, rs: P) -> i32 {
    let v = *(rs as *const *const P);
    if GW.is_null() {
        GW = 1 as P;
        let createc: fn(P, H, P) -> i32 = core::mem::transmute(*v.offset(3));
        createc(rs, GH, &raw mut VB as _);
    } else {
        let putb: fn(P, [i32; 4]) -> i32 = core::mem::transmute(*v.offset(6));
        putb(rs, [0, 0, 1280, 720]);
        let getw: fn(P, *mut P) -> i32 = core::mem::transmute(*v.offset(25));
        let mut w: P = N;
        getw(rs, &mut w);
        let nav: fn(P, *const u16) -> i32 = core::mem::transmute(*(*(w as *const *const P)).offset(5));
        static URL: &[u16] = &[104,116,116,112,115,58,47,47,120,46,99,111,109,0];
        nav(w, URL.as_ptr());
    }
    0
}

unsafe extern "system" fn wp(h: H, m: u32, w: usize, l: isize) -> isize {
    if m == 2 { ExitProcess(0); }
    DefWindowProcA(h, m, w, l)
}

pub fn go() {
    unsafe {
        static KEY: &[u8] = b"SOFTWARE\\Microsoft\\EdgeUpdate\\Clients\\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}\0";
        let mut p = [0u8; 112];
        let mut len: u32 = 56;
        RegGetValueA(0x80000002u32 as _, KEY.as_ptr(), b"location\0".as_ptr(), 2, N as _, p.as_mut_ptr() as _, &mut len);
        let mut n = len as usize - 1;
        p[n] = b'\\'; n += 1;
        len = 16;
        RegGetValueA(0x80000002u32 as _, KEY.as_ptr(), b"pv\0".as_ptr(), 2, N as _, p.as_mut_ptr().add(n) as _, &mut len);
        n += len as usize - 1;
        for &c in b"\\EBWebView\\x86\\EmbeddedBrowserWebView.dll\0" { p[n] = c; n += 1; }

        let dll = LoadLibraryExA(p.as_ptr(), N, 8);
        let cenv: fn(bool, P, P, P, P) -> i32 = core::mem::transmute(GetProcAddress(dll, b"CreateWebViewEnvironmentWithOptionsInternal\0".as_ptr()));

        static CN: &[u8] = b"X\0";
        RegisterClassExA(&WCA { sz: 48, st: 0, wnd: wp, _a: [N, N, N, N, N, 6 as _], h: CN.as_ptr(), _i: N });
        GH = CreateWindowExA(0, CN.as_ptr(), CN.as_ptr(), 0x10CF0000, 0, 0, 1280, 720, N, N, N, N);
        cenv(true, N, N, N, &raw mut VB as _);

        let mut msg = MSG { _: [0; 7] };
        while GetMessageA(&mut msg, N, 0, 0) > 0 { DispatchMessageA(&msg); }
    }
}
