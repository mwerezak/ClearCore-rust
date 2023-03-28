# ClearCore-Rust
This project is aimed at enabling the use of the Rust programming language to create embedded firmware for the ClearCore I/O and Motion controller produced by Teknic Inc.

This project is built on top of the [project template](https://github.com/Teknic-Inc/ClearCore-library) provided by Teknic, instead of trying to reinvent the wheel (so that I can focus on making the ClearCore wrapper library). 

Under construction is a safe-Rust wrapper around the ClearCore library to facilitate ClearCore development using Rust.  

If needed, you can access peripherals or ATSAME53N functionality not exposed by the ClearCore library using the `atsame53n` peripheral access crate on [crates.io](https://crates.io/crates/atsame53n).

## Requirements
### Rust
To build the Rust firmware you will need to have the Rust target `thumbv7em-none-eabihf` installed (e.g. using `rustup`). 

Additionally, this project uses `bindgen` to create the raw ClearCore bindings, so there is a build-time dependency on LLVM (for parsing the C/C++ headers). You can get LLVM from the [LLVM website](https://releases.llvm.org/download.html). You will need to set the `LIBCLANG_PATH` environment variable pointing to the `bin` directory of your LLVM install.

### ClearCore-Library
To build the ClearCore C/C++ firmware you will need both Microchip Studio 7.0 with the following device packs installed:

* SAME53_DFP version 1.1.118
* CMSIS version 4.5.0

Unfortunately Microchip Studio 7.0 requires a Windows environment.

## Usage

The `ProjectTemplate` folder is a template project that you can use to write your ClearCore firmware. It contains both a cargo project and an .atsln file. You should not need to modify any of the Microchip Studio files, they contain the startup code (inside the `Device_Startup` folder) and are only used to build the final firmware image.

The workflow is more or less:

1. Rename `ProjectTemplate` and write your Rust firmware in it (using the `clearcore` wrapper and possibly the `atsame53n` PAC crates).
2. Build the Rust code using `cargo build` or `cargo build --release`. This creates `rustmain.a` which supplies the `main()` method.
3. Build the .atsln in Microchip Studio. This creates the actual firmware images (.elf, .bin, and .uf2 files).
4. Use your desired tool (in Microchip Studio or using OpenOCD) to program the ClearCore.

Debugging works pretty much the same as for a C/C++ project, except that Microchip Studio doesn't have syntax highlighting for Rust code. Alternatively you can use OpenOCD/gdb/gdbgui with the Atmel-ICE debugger which does have Rust syntax highlighting.
