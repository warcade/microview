# microview

A fully functional WebView2 browser in **2,960 bytes** (2.89 KB).

```
┌─────────────────────────────────────────────────────────────┐
│ Chrome (430 MB)                                             │
│ █████████████████████████████████████████████████████████   │
│                                                             │
│ microview.exe (2.89 KB)                                     │
│ (less than 1 pixel at this scale)                           │
└─────────────────────────────────────────────────────────────┘

Chrome is 145,000x larger.
```

## What is this?

A Windows executable that opens a fully functional Chromium-based browser window. It loads a webpage, handles window resizing, and works exactly like any Electron/Tauri app - but in under 3 KB.

This works because Windows 10/11 ships with the WebView2 runtime (the Edge browser engine). We just create a window and ask WebView2 to render in it.

## Quick Start

```rust
// main.rs
let wv = WebView::new(800, 600);
wv.navigate(&url(b"https://google.com\0"));
wv.run();
```

## Build

Requires Rust nightly and Windows:

```powershell
rustup install nightly
cargo +nightly build --release
```

The executable will be at `target/x86_64-pc-windows-msvc/release/microview.exe`

## How is this possible?

### 1. No Standard Library (`#![no_std]`)

The Rust standard library adds ~300KB. We use `#![no_std]` and call Windows APIs directly.

### 2. ANSI APIs

Windows has two versions of most functions:
- `CreateWindowExW` - takes UTF-16 strings (2 bytes per char)
- `CreateWindowExA` - takes ANSI strings (1 byte per char)

We use ANSI everywhere except `Navigate()` which requires UTF-16.

### 3. Direct WebView2 Runtime Loading

Instead of using Microsoft's `WebView2Loader.dll` (162 KB), we:
1. Read the Edge location from registry
2. Load `EmbeddedBrowserWebView.dll` directly with `LoadLibraryExA`
3. Call the internal `CreateWebViewEnvironmentWithOptionsInternal` function

### 4. Manual COM Implementation

WebView2 uses COM interfaces. Instead of a COM library, we manually implement vtables:

```rust
#[repr(C)]
struct VTbl {
    QueryInterface: fn(...),
    AddRef: fn(...),
    Release: fn(...),
    Invoke: fn(...),  // Our callback
}
```

### 5. Aggressive Linker Flags

```toml
# .cargo/config.toml
rustflags = [
    "-C", "link-arg=/ALIGN:16",
    "-C", "link-arg=/FILEALIGN:1",
    "-C", "link-arg=/MERGE:.rdata=.text",
    "-C", "link-arg=/MERGE:.pdata=.text",
    "-C", "link-arg=/MERGE:.data=.text",
    "-C", "link-arg=/MERGE:.bss=.text",
]
```

### 6. No Error Handling

If WebView2 isn't installed, it crashes. This saves hundreds of bytes of error handling code.

## Project Structure

```
microview/
├── src/
│   ├── main.rs      # Entry point (30 lines)
│   └── webview.rs   # WebView2 wrapper (160 lines)
├── .cargo/
│   └── config.toml  # Linker flags
├── Cargo.toml
└── README.md
```

## API

### `WebView::new(width: i32, height: i32) -> WebView`

Creates a window and initializes WebView2.

### `webview.navigate(&url)`

Navigates to a URL. The URL must be UTF-16 encoded. Use the `url()` helper:

```rust
static URL: [u16; 23] = url(b"https://example.com\0");
wv.navigate(&URL);
```

### `webview.run()`

Runs the message loop. Blocks until the window is closed.

## URL Helper

WebView2's `Navigate` function requires UTF-16 strings. We provide a compile-time converter:

```rust
/// Converts ASCII to UTF-16 at compile time
const fn url<const N: usize>(s: &[u8; N]) -> [u16; N] {
    let mut out = [0u16; N];
    let mut i = 0;
    while i < N - 1 {
        out[i] = s[i] as u16;
        i += 1;
    }
    out
}

// Usage - the \0 is required for null termination
static URL: [u16; 24] = url(b"https://github.com\0");
```

## Size Breakdown

| Component | Bytes |
|-----------|-------|
| PE headers | ~400 |
| Import table | ~300 |
| Registry/DLL path strings | ~200 |
| Window creation | ~400 |
| COM callback handlers | ~500 |
| WebView2 init + navigation | ~600 |
| Message loop | ~200 |
| URL data | ~50 |
| **Total** | **2,960** |

## Comparison

| Browser | Size |
|---------|------|
| Chrome | 430 MB |
| Firefox | 250 MB |
| Electron app (empty) | 150 MB |
| Tauri app (empty) | 3 MB |
| **microview** | **2.89 KB** |

## Requirements

- Windows 10/11 (WebView2 runtime pre-installed)
- x64 architecture
- Rust nightly toolchain

## Limitations

- Windows only
- No error handling - crashes if WebView2 unavailable
- Single window only (uses global state)
- ASCII URLs only (use `url()` helper)

## License

Public domain. Do whatever you want.

## Acknowledgments

- Inspired by the demoscene tradition of extreme size optimization
- WebView2 runtime by Microsoft
