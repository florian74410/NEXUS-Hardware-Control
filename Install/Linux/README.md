# Linux Mint
Planned systemd host service under dedicated account, executable in /opt/nhc, configuration in /etc/nhc. Read-only monitoring with minimal device permissions. No desktop session or network dependency.

## Core 0.0.6 terminal installer
Run `bash Install/Linux/install.sh` for preview, then `bash Install/Linux/install.sh --yes` from repository root. Installs missing build packages and detected optional sensor tools through apt/dnf, compiles and tests, installs per-user binary, and runs doctor. No GUI. No driver replacement, no permissions changes, no permanent module loading or system service yet. Immutable Bazzite and Windows/macOS unsupported. SMART/RAPL permissions diagnosed but not relaxed. Intel GPU tools installed only on detected Intel graphics; collector does not yet invoke intel_gpu_top. Fedora package availability and builds remain to validate.
