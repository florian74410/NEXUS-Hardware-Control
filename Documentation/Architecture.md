# Architecture NHC 0.0.1 — design
Core Rust: native daemon + CLI, one canonical store. Modules collect/analyze/control (control disabled in phase 1); OS/vendor adapters selected by capability, not hardcoded GPU model. Touch C++/LVGL: separate data cache and theme engine, never a second source of truth. Desktop NEXUS consumes IPC.

Boot: USB discovery and baseline sampling independently, retry drivers as they appear; never block boot waiting for GPU/USB. Touch states BOOT, WAIT_HOST, SYNC, LIVE, STALE, ERROR. On disconnect invalidate live values. Each installed OS has its own service. No POST support.

Security: least privilege, no shell over USB, explicit allowlist and validation for future writes, local IPC permissions. No hardware changes in release 0.1.

Memory: bounded telemetry cache, ring buffers, partial RGB565 rendering, optional SD theme assets, flash for firmware and settings; measure board flash, PSRAM, free heap and frame latency before committing to theme budgets. No unverified board pinout.
