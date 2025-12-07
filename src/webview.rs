#![allow(non_snake_case)]

use core::ffi::c_void;
use core::ptr::{null, null_mut};

type H = *mut c_void;
type P = *mut c_void;

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

#[repr(C)] struct R { l: i32, t: i32, r: i32, b: i32 }
#[repr(C)] struct MSG { h: H, m: u32, w: usize, l: isize, t: u32, x: i32, y: i32 }
#[repr(C)] struct WCA { sz: u32, st: u32, wnd: unsafe extern "system" fn(H, u32, usize, isize) -> isize, a: i32, b: i32, c: P, d: P, e: P, f: P, g: *const u8, h: *const u8, i: P }

static mut GH: H = null_mut();
static mut GW: P = null_mut();
static mut GWD: i32 = 0;
static mut GHT: i32 = 0;
static mut GU: *const u8 = null();

#[repr(C)] struct VT {
    q: unsafe extern "system" fn(P, *const c_void, *mut P) -> i32,
    a: unsafe extern "system" fn(P) -> u32,
    r: unsafe extern "system" fn(P) -> u32,
    i: unsafe extern "system" fn(P, i32, P) -> i32,
}
static VTB: VT = VT { q: vq, a: va, r: va, i: vi };
#[repr(C)] struct B { v: *const VT }
static mut VB: B = B { v: &VTB };

unsafe extern "system" fn vq(t: P, _: *const c_void, p: *mut P) -> i32 { *p = t; 0 }
unsafe extern "system" fn va(_: P) -> u32 { 1 }

unsafe extern "system" fn vi(_: P, hr: i32, rs: P) -> i32 {
    if hr != 0 { return hr; }
    let v = *(rs as *const *const P);
    let addref: fn(P) = core::mem::transmute(*v.offset(1));
    addref(rs);
    if GW.is_null() {
        GW = 1 as P;
        let createc: fn(P, H, P) -> i32 = core::mem::transmute(*v.offset(3));
        createc(rs, GH, &raw mut VB as P);
    } else {
        let putb: fn(P, R) -> i32 = core::mem::transmute(*v.offset(6));
        putb(rs, R { l: 0, t: 0, r: GWD, b: GHT });
        let getw: fn(P, *mut P) -> i32 = core::mem::transmute(*v.offset(25));
        let mut w: P = null_mut();
        getw(rs, &mut w);
        let wv = *(w as *const *const P);
        let nav: fn(P, *const u16) -> i32 = core::mem::transmute(*wv.offset(5));
        let mut url = [0u16; 64];
        let mut i = 0;
        while *GU.add(i) != 0 { url[i] = *GU.add(i) as u16; i += 1; }
        nav(w, url.as_ptr());
    }
    0
}

unsafe extern "system" fn wp(h: H, m: u32, w: usize, l: isize) -> isize {
    if m == 2 { ExitProcess(0); }
    DefWindowProcA(h, m, w, l)
}

pub fn go(w: i32, h: i32, url: &[u8]) {
    unsafe {
        GWD = w; GHT = h; GU = url.as_ptr();

        static KEY: &[u8] = b"SOFTWARE\\WOW6432Node\\Microsoft\\EdgeUpdate\\Clients\\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}\0";
        let mut loc = [0u8; 64];
        let mut ver = [0u8; 16];
        let mut len: u32 = 64;
        RegGetValueA(0x80000002u32 as _, KEY.as_ptr(), b"location\0".as_ptr(), 2, null_mut(), loc.as_mut_ptr() as _, &mut len);
        len = 16;
        RegGetValueA(0x80000002u32 as _, KEY.as_ptr(), b"pv\0".as_ptr(), 2, null_mut(), ver.as_mut_ptr() as _, &mut len);

        let mut path = [0u8; 128];
        let mut p = 0;
        let mut i = 0;
        while loc[i] != 0 { path[p] = loc[i]; p += 1; i += 1; }
        path[p] = b'\\'; p += 1;
        i = 0;
        while ver[i] != 0 { path[p] = ver[i]; p += 1; i += 1; }
        for &c in b"\\EBWebView\\x64\\EmbeddedBrowserWebView.dll\0" { path[p] = c; p += 1; }

        let dll = LoadLibraryExA(path.as_ptr(), null_mut(), 8);
        let cenv: fn(bool, P, P, P, P) -> i32 = core::mem::transmute(GetProcAddress(dll, b"CreateWebViewEnvironmentWithOptionsInternal\0".as_ptr()));

        static CN: &[u8] = b"X\0";
        RegisterClassExA(&WCA { sz: 80, st: 0, wnd: wp, a: 0, b: 0, c: null_mut(), d: null_mut(), e: null_mut(), f: 6 as _, g: null(), h: CN.as_ptr(), i: null_mut() });
        GH = CreateWindowExA(0, CN.as_ptr(), CN.as_ptr(), 0x10CF0000, 100, 100, w, h, null_mut(), null_mut(), null_mut(), null_mut());
        cenv(true, null_mut(), null_mut(), null_mut(), &raw mut VB as P);

        let mut msg: MSG = core::mem::zeroed();
        while GetMessageA(&mut msg, null_mut(), 0, 0) > 0 { DispatchMessageA(&msg); }
    }
}
