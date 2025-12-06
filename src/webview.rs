#![allow(non_snake_case)]

use core::ffi::c_void;
use core::ptr::{null, null_mut};

type H = *mut c_void;
type P = *mut c_void;

#[link(name = "kernel32")]
extern "system" {
    fn LoadLibraryExA(n: *const u8, f: P, flags: u32) -> H;
    fn GetProcAddress(m: H, n: *const u8) -> P;
    pub fn ExitProcess(c: u32);
}

#[link(name = "advapi32")]
extern "system" {
    fn RegGetValueA(k: H, sub: *const u8, val: *const u8, flags: u32, typ: *mut u32, data: P, len: *mut u32) -> i32;
}

#[link(name = "user32")]
extern "system" {
    fn RegisterClassExA(wc: *const WCA) -> u16;
    fn CreateWindowExA(ex: u32, cls: *const u8, title: *const u8, style: u32, x: i32, y: i32, w: i32, h: i32, parent: H, menu: P, inst: P, param: P) -> H;
    fn DefWindowProcA(h: H, m: u32, w: usize, l: isize) -> isize;
    fn GetMessageA(m: *mut MSG, h: H, a: u32, b: u32) -> i32;
    fn DispatchMessageA(m: *const MSG) -> isize;
}

#[repr(C)] struct R { l: i32, t: i32, r: i32, b: i32 }
#[repr(C)] struct MSG { h: H, m: u32, w: usize, l: isize, t: u32, x: i32, y: i32 }
#[repr(C)] struct WCA { sz: u32, st: u32, wnd: unsafe extern "system" fn(H, u32, usize, isize) -> isize, a: i32, b: i32, c: P, d: P, e: P, f: P, g: *const u8, h: *const u8, i: P }

static mut GH: H = null_mut();
static mut GC: P = null_mut();
static mut GW: P = null_mut();
static mut GR: bool = false;
static mut GWD: i32 = 800;
static mut GHT: i32 = 600;

static VT: VTbl = VTbl { q: vq, a: va, r: va, i: vi };
#[repr(C)] struct B { v: *const VTbl }
static mut VB: B = B { v: &VT };

#[repr(C)] struct VTbl {
    q: unsafe extern "system" fn(P, *const c_void, *mut P) -> i32,
    a: unsafe extern "system" fn(P) -> u32,
    r: unsafe extern "system" fn(P) -> u32,
    i: unsafe extern "system" fn(P, i32, P) -> i32,
}

unsafe extern "system" fn vq(t: P, _: *const c_void, p: *mut P) -> i32 { *p = t; 0 }
unsafe extern "system" fn va(_: P) -> u32 { 1 }

unsafe extern "system" fn vi(_: P, hr: i32, rs: P) -> i32 {
    if hr != 0 || rs.is_null() { return hr; }
    let v = *(rs as *const *const P);
    let addref: unsafe extern "system" fn(P) = core::mem::transmute(*v.offset(1));
    addref(rs);

    if !GC.is_null() {
        GC = rs;
        let getw: unsafe extern "system" fn(P, *mut P) -> i32 = core::mem::transmute(*v.offset(25));
        let putb: unsafe extern "system" fn(P, R) -> i32 = core::mem::transmute(*v.offset(6));
        let mut w: P = null_mut();
        getw(rs, &mut w);
        if !w.is_null() { GW = w; }
        putb(rs, R { l: 0, t: 0, r: GWD, b: GHT });
        GR = true;
    } else {
        GC = rs;
        let createc: unsafe extern "system" fn(P, H, P) -> i32 = core::mem::transmute(*v.offset(3));
        createc(rs, GH, &raw mut VB as P);
    }
    0
}

unsafe extern "system" fn wp(h: H, m: u32, w: usize, l: isize) -> isize {
    match m {
        5 => {
            if !GC.is_null() && GR {
                let lo = (l & 0xFFFF) as i32;
                let hi = ((l >> 16) & 0xFFFF) as i32;
                let v = *(GC as *const *const P);
                let putb: unsafe extern "system" fn(P, R) -> i32 = core::mem::transmute(*v.offset(6));
                putb(GC, R { l: 0, t: 0, r: lo, b: hi });
            }
            0
        }
        2 => { ExitProcess(0); 0 }
        _ => DefWindowProcA(h, m, w, l)
    }
}

pub struct WebView;

impl WebView {
    pub fn new(width: i32, height: i32) -> Self {
        unsafe {
            GWD = width;
            GHT = height;

            static KEY: &[u8] = b"SOFTWARE\\WOW6432Node\\Microsoft\\EdgeUpdate\\Clients\\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}\0";

            let mut loc = [0u8; 80];
            let mut ver = [0u8; 24];
            let mut len: u32 = 80;

            RegGetValueA(0x80000002u32 as _, KEY.as_ptr(), b"location\0".as_ptr(), 2, null_mut(), loc.as_mut_ptr() as _, &mut len);
            len = 24;
            RegGetValueA(0x80000002u32 as _, KEY.as_ptr(), b"pv\0".as_ptr(), 2, null_mut(), ver.as_mut_ptr() as _, &mut len);

            let mut path = [0u8; 160];
            let mut p = 0;
            let mut i = 0;
            while loc[i] != 0 { path[p] = loc[i]; p += 1; i += 1; }
            path[p] = b'\\'; p += 1;
            i = 0;
            while ver[i] != 0 { path[p] = ver[i]; p += 1; i += 1; }
            for &c in b"\\EBWebView\\x64\\EmbeddedBrowserWebView.dll\0" { path[p] = c; p += 1; }

            let dll = LoadLibraryExA(path.as_ptr(), null_mut(), 8);

            let cenv: unsafe extern "system" fn(bool, P, P, P, P) -> i32 =
                core::mem::transmute(GetProcAddress(dll, b"CreateWebViewEnvironmentWithOptionsInternal\0".as_ptr()));

            static CN: &[u8] = b"X\0";
            let wc = WCA { sz: 80, st: 0, wnd: wp, a: 0, b: 0, c: null_mut(), d: null_mut(), e: null_mut(), f: 6 as _, g: null(), h: CN.as_ptr(), i: null_mut() };
            RegisterClassExA(&wc);

            GH = CreateWindowExA(0, CN.as_ptr(), CN.as_ptr(), 0x10CF0000, 100, 100, width, height, null_mut(), null_mut(), null_mut(), null_mut());
            cenv(true, null_mut(), null_mut(), null_mut(), &raw mut VB as P);

            Self
        }
    }

    pub fn navigate(&self, url: &[u16]) {
        unsafe {
            let mut msg: MSG = core::mem::zeroed();
            while !GR { if GetMessageA(&mut msg, null_mut(), 0, 0) > 0 { DispatchMessageA(&msg); } }

            if !GW.is_null() {
                let v = *(GW as *const *const P);
                let nav: unsafe extern "system" fn(P, *const u16) -> i32 = core::mem::transmute(*v.offset(5));
                nav(GW, url.as_ptr());
            }
        }
    }

    pub fn run(&self) {
        unsafe {
            let mut msg: MSG = core::mem::zeroed();
            while GetMessageA(&mut msg, null_mut(), 0, 0) > 0 { DispatchMessageA(&msg); }
        }
    }
}
