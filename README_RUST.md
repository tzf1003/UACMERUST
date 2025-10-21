# UACME Rust Port

This is a Rust port of the UACME (User Account Control Bypass) project, originally written in C/C++.

## ⚠️ Warning

This tool demonstrates security vulnerabilities that could be exploited maliciously. Use responsibly and only in controlled environments for educational and research purposes.

## Project Structure

```
uacme-rust/
├── shared/         # Shared library (strings, PE loader, Windows API wrappers)
├── akagi/          # Main executable
├── fubuki/         # General purpose payload DLL
├── akatsuki/       # WOW64 logger payload DLL
├── naka/           # Compression tool
├── yuubari/        # UAC information dumper
└── Source/         # Original C/C++ source (preserved for reference)
```

## Building

### Prerequisites

- Rust 1.70 or later
- Windows SDK (for cross-compilation targets)
- Visual Studio Build Tools (optional, for debugging)

### Build Commands

```bash
# Build all projects (release mode)
cargo build --release

# Build specific project
cargo build --release -p akagi
cargo build --release -p fubuki
cargo build --release -p naka

# Build for 32-bit (requires i686 target)
rustup target add i686-pc-windows-msvc
cargo build --release --target i686-pc-windows-msvc

# Build for 64-bit
cargo build --release --target x86_64-pc-windows-msvc
```

## Usage

### Akagi (Main Program)

```bash
# Run with method number
akagi.exe <method_number>

# Run with custom payload
akagi.exe <method_number> <path_to_executable>

# Examples
akagi.exe 23
akagi.exe 33 c:\windows\system32\calc.exe
```

### Naka (Compression Tool)

```bash
# Compress a file
naka.exe <input_file>

# Create secret tables
naka.exe --stable
```

### Yuubari (UAC Info Dumper)

```bash
# Dump UAC configuration
yuubari.exe
```

## Implemented Methods

Currently implemented UAC bypass methods:

- **Method 0**: Test method (basic functionality test)
- **Method 22**: SXS Consent (Windows 7 - Windows 10 RS1)
- **Method 23**: DISM (Windows 7 - Windows 10 RS1)
- **Method 33**: MS Settings (Windows 10 TH1 - RS1)
- **Method 34**: Disk Silent Cleanup (Windows 7 - Windows 10 RS3)

More methods are being ported from the original C implementation.

## Architecture

### Shared Library

The `shared` crate provides common functionality:

- **String operations**: Wide string manipulation (UTF-16)
- **PE Loader**: Manual PE image loading and relocation
- **Windows API extensions**: Registry, file operations, process creation
- **Anti-emulator**: Detection of sandboxes and emulators
- **Utilities**: System information, encoding, etc.

### Akagi

Main executable that:
1. Parses command line arguments
2. Initializes context (system paths, build number, etc.)
3. Dispatches to the appropriate UAC bypass method
4. Manages payload execution

### Fubuki

General purpose payload DLL that:
- Executes elevated commands
- Supports multiple entry points (DLL, EXE, UI Access)
- Communicates with Akagi via shared memory

### Akatsuki

Specialized payload for WOW64 logger method:
- Elevates to NT AUTHORITY\SYSTEM
- Creates processes in user sessions

## Differences from Original

### Improvements

- **Memory Safety**: Rust's ownership system prevents many classes of bugs
- **Modern Dependencies**: Uses `windows-rs` for Windows API bindings
- **Better Error Handling**: Result types instead of error codes
- **Modular Design**: Clear separation of concerns with Rust modules

### Limitations

- Some methods require `unsafe` code for low-level operations
- PE loading still needs manual memory management
- DLL entry points must use C ABI (`extern "system"`)

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run tests for specific crate
cargo test -p shared
cargo test -p akagi
```

### Logging

Set the `RUST_LOG` environment variable for debug output:

```bash
set RUST_LOG=debug
akagi.exe 23
```

## Security Considerations

1. **Educational Purpose Only**: This tool is for security research and education
2. **No Malicious Use**: Do not use this tool for unauthorized access
3. **Responsible Disclosure**: Report vulnerabilities to Microsoft
4. **Legal Compliance**: Ensure compliance with local laws and regulations

## Original Project

This is a port of [UACME by hfiref0x](https://github.com/hfiref0x/UACME).

All credit for the original research and implementation goes to the original authors.

## License

BSD 3-Clause License (same as original project)

## Contributing

Contributions are welcome! Please:

1. Maintain compatibility with the original C implementation
2. Follow Rust best practices
3. Add tests for new functionality
4. Update documentation

## Roadmap

- [ ] Port all 60+ UAC bypass methods
- [ ] Implement shared memory communication
- [ ] Add comprehensive tests
- [ ] Improve error handling
- [ ] Add method-specific documentation
- [ ] Performance optimization
- [ ] Cross-compilation support

## Support

For issues and questions:
- Check the original UACME documentation
- Review the Rust port documentation
- Open an issue on GitHub

## Disclaimer

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED. USE AT YOUR OWN RISK.

