# zenoh-flat

`zenoh-flat` exposes two complementary API tiers from one flat Rust crate surface.

## API split

- **base** contains the low-level Zenoh-facing API: `z_*` functions plus the supporting enums and value types needed to drive them directly.
- **expanded** contains wrapper data types, `impl Into<_>`-style ergonomic entry points, and one-hop expansion helpers that turn native Zenoh values into binding-friendly structs.

Internally, the source tree is organized as `src/base/` and `src/expanded/`, but the crate continues to re-export everything from `zenoh_flat::*`.

## Why the split exists

- **Languages with cheap native calls** can stay on the **base** tier and pull fields through fine-grained `z_*` accessors.
- **Languages with expensive native calls** can use the **expanded** tier, where generated bindings convert wrapper structs to and from native Zenoh values in fewer FFI hops.

That makes the same annotated Rust source usable for both C-style low-level bindings and higher-level generated bindings such as JNI/Kotlin.
