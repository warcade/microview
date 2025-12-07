# microview

A fully functional WebView2 browser in as little as **2,576 bytes** (2.51 KB).

```
┌──────────────────────────────────────────────────────────────┐
│ Chrome (430 MB)    ██████████████████████████████████████████│
│ Brave (400 MB)     ███████████████████████████████████████   │
│ Edge (370 MB)      ████████████████████████████████████      │
│ Firefox (250 MB)   ████████████████████████                  │
│ Electron (150 MB)  ██████████████                            │
│ Tauri (3 MB)       ▏                                        │
│ microview (2.5 KB) ▏ (less than 1 pixel at this scale)      │
└──────────────────────────────────────────────────────────────┘
```

## What is this?

A Windows executable that opens a fully functional Chromium-based browser window. It loads a webpage and works exactly like any Electron/Tauri app - but in just 2.51 KB.

This works because Windows 10/11 ships with the WebView2 runtime (the Edge browser engine). We just create a window and ask WebView2 to render in it.

## Quick Start

```rust
// main.rs - that's it, one line!
webview::go(800, 600, b"https://google.com\0");
```

## Download

Grab the pre-built binary from [Releases](https://github.com/warcade/microview/releases) and run it. No installation needed.

## Build

Requires Rust and Windows:

```powershell
git clone https://github.com/warcade/microview.git
cd microview
cargo build --release
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
│   ├── main.rs      # Entry point (12 lines)
│   └── webview.rs   # WebView2 wrapper (115 lines)
├── .cargo/
│   └── config.toml  # Linker flags
├── Cargo.toml
└── README.md
```

## API

### `webview::go(width: i32, height: i32, url: &[u8])`

Creates a window, initializes WebView2, navigates to the URL, and runs the message loop. Blocks until the window is closed. The URL must be ASCII with a null terminator.

```rust
webview::go(800, 600, b"https://github.com\0");
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
| **Total** | **~2,576** |

## Comparison

### Browsers

| Browser | Size | vs microview |
|---------|------|--------------|
| Chrome | 430 MB | 175,000x larger |
| Brave | 400 MB | 163,000x larger |
| Edge | 370 MB | 151,000x larger |
| Vivaldi | 290 MB | 118,000x larger |
| Firefox | 250 MB | 102,000x larger |
| Opera | 240 MB | 98,000x larger |
| NetSurf | 16 MB | 6,500x larger |
| Dillo | 1.5 MB | 612x larger |
| Tiny Browser | 200 KB | 80x larger |
| **microview** | **2.51 KB** | **1x** |

### App Frameworks (empty "Hello World" app)

| Framework | Size | vs microview |
|-----------|------|--------------|
| Electron | 150 MB | 52,000x larger |
| NW.js | 120 MB | 41,000x larger |
| CEF (C++) | 100 MB | 34,000x larger |
| Qt WebEngine | 60 MB | 21,000x larger |
| Flutter | 25 MB | 8,600x larger |
| Neutralino | 3 MB | 1,000x larger |
| Tauri | 3 MB | 1,000x larger |
| Wails | 8 MB | 2,800x larger |
| **microview** | **2.51 KB** | **1x** |

## Requirements

- Windows 10/11 (WebView2 runtime pre-installed)
- x64 architecture
- Rust toolchain

## Limitations

- Windows only
- No error handling - crashes if WebView2 unavailable
- Single window only (uses global state)
- ASCII URLs only (max 64 chars)
- WebView doesn't resize with window (fixed initial size)

## License

Public domain. Do whatever you want.

## Acknowledgments

- Inspired by the demoscene tradition of extreme size optimization
- WebView2 runtime by Microsoft
