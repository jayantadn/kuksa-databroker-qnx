# Local Patch Note

Earlier an experimental modification introduced a RustCrypto-based ECDSA backend
for QNX (`target_os = "nto"`). That approach was reverted because this vendored
workspace configuration replaces the crates.io registry with the local `vendor/`
directory, preventing addition of new third-party crates (`ecdsa`, `p256`, `p384`).

Current state:
* The crate is back to the upstream ring-only implementation (version 0.17.4).
* No additional features or dependencies were retained.
* ECDSA still uses `ring`'s `_FIXED` algorithms producing raw (r||s) signatures.

If QNX builds still fail due to `SystemRandom` availability, consider instead:
1. Ensuring the QNX entropy source (/dev/random or equivalent) is accessible in the build/runtime env.
2. Upstreaming a patch that removes the RNG argument once/if ring exposes an API variant not needing it.
3. Disabling ES256/ES384 algorithms on QNX if they are not required.

This file documents that a RustCrypto detour was attempted and intentionally removed.
