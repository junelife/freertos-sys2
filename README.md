# freertos-sys2

Raw/direct bindings to FreeRTOS functions and types

## Principles

 1. Support APIs for directly interacting with FreeRTOS APIs and datastructures
 2. Keep the build fast and robust
   - Right now, this means we don't use bindgen (but bindgen is not totally excluded)
 3. Make sure that `check` can always pass
   - `cargo check` is important for developer tooling. Make sure this always
     can complete check regardless of features selected (unless they are
     specifically incompatible) and the rust target used
   - Right now, there are no target cfgs or feature cfgs (this can change)

## Other freertos bindings that might work for your project

 1. [`freertos-rust`](https://crates.io/crates/freertos-rust)
    - Like `freertos-sys2`, this assumes that FreeRTOS is built seperately and
      linked in by some external mechanism
    - Unlike `freertos-sys2`, this binds all FreeRTOS APIs by using stub
      functions written in C.
    - Unlike `freertos-sys2`, this provides higher level APIs directly. Does
      not expose direct bindings to FreeRTOS APIs
 2. [`freertos-sys`](https://crates.io/crates/freertos-sys)
    - Unlike `freertos-sys2`, builds FreeRTOS in it's build.rs script
    - Unlike `freertos-sys2`, does not export any FreeRTOS specific functions,
      instead exporting cmsis-rtos2
 3. [`freertos_rs`](https://crates.io/crates/freertos_rs)
    - Unlike `freertos-sys2`, uses shims written in C to interact with FreeRTOS
      rather than binding the symbols directly
    - Unlike `freertos-sys2`, does not expose the low level function calls.
      Only high level abstractions are exposed
    - Like `freertos-sys2`, assumes that FreeRTOS is built seperately and
      linked together by some external mechanism.

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
