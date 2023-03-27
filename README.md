# ClearCore-Rust
This project is aimed at enabling the use of the Rust programming language to create embedded firmware for the ClearCore I/O and Motion controller produced by Teknic Inc.

This project is built on top of the [project template](https://github.com/Teknic-Inc/ClearCore-library) provided by Teknic, instead of trying to reinvent the wheel. 

The basic idea is that your Rust code is compiled into a static library that is linked into the firmware image built in Microchip Studio. This static library provides the "main" function instead of main.cpp.

Instead the main effort will be put towards creating a safe wrapper around the ClearCore library to ensure ClearCore development using Rust is safe and ergonomic. Additionally, access to peripherals or ATSAME53N functionality not exposed by the ClearCore library can be obtained using the `atsame53n` peripheral access crate.

## Requirements and Target Architecture
To build the ClearCore C/C++ firmware you will need both Microchip Studio 7.0 with the following device packs installed:

* SAME53_DFP version 1.1.118
* CMSIS version 4.5.0

To build the Rust firmware you will need to have the Rust target `thumbv7em-none-eabihf` installed (e.g. using `rustup`). 

Additionally, this project uses `bindgen` to create the raw ClearCore bindings, so there is a build-time dependency on LLVM (for parsing the C/C++ headers). You can get LLVM from the [LLVM website](https://releases.llvm.org/download.html). You will need to set the `LIBCLANG_PATH` environment variable pointing to the `bin` directory of your LLVM install.
