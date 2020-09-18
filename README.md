# psyche

> Fast Ethereum Virtual Machine implementation in Rust

## Project goals

- [x] Research, implement and optimize interpretation of EVM bytecode
   in stable Rust.
- [x] Explore around using special CPU instructions to speed up most opcodes.
- [x] Support a generic target for best portability (64-bits, 32 bits).
- [x] Optimize for the worst-case (do not optimize average-case at the expense of
   worst-case).
- [ ] Improve Rust compiler for better opcode dispatch via ```indirectbr``` instruction.
- [ ] Support the EVMC low-level ABI and integrate in OpenEthereum client.

## Build

Depending what you want to do you can either use:

- ```cargo build```
- ```cargo build --release```
- ```./build_debug.sh``` for an AVX2 build in debug
- ```./build_release.sh``` for an AVX2 build in release
- ```./build_avx2.sh``` for inspecting assembly code for AVX2
  (see target/release/deps/psyche.s)
- ```./build_ssse3.sh``` for inspecting assembly code for SSSE3
- ```./build_generic.sh``` for inspecting generic assembly code


Note that a build on macOS enables by default SSSE3 as those instructions are
always present on all Intel-based Macs.

If you want to inspect assembly code you will also need to uncomment the line
with ```#![feature(asm)]```.

## Debug

If you have LLDB installed on your machine you can add this line to your ```.lldbinit``` file:

```command script import evmd.py```

[![asciicast](https://asciinema.org/a/360189.svg)](https://asciinema.org/a/360189)


## License

[LICENSE](https://github.com/elmattic/psyche/blob/master/LICENSE)
