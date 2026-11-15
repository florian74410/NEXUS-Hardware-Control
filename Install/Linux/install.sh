#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)"
if [[ $EUID -eq 0 ]]; then echo "Run as your normal user, not root."; exit 1; fi
if [[ -e /run/ostree-booted ]]; then echo "Immutable OS: host installation not yet supported."; exit 1; fi
if [[ ${1:-} != --yes ]]; then
  echo "NHC installer: may install build tools, sensor utilities and load detected sensor modules and install a root sensor publisher via sudo."
  echo "Run bash Install/Linux/install.sh --yes to authorize."; exit 0
fi
if command -v apt-get >/dev/null; then
  packages=(build-essential pkg-config libudev-dev cargo rustc python3)
  missing=()
  for p in "${packages[@]}"; do
    if dpkg-query -W -f='${Status}' "$p" 2>/dev/null | grep -qx 'install ok installed'; then echo "[OK] $p"; else
      if [[ $p == cargo || $p == rustc ]] && command -v "$p" >/dev/null; then echo "[OK] $p"; else missing+=("$p"); fi
    fi
  done
  if (( ${#missing[@]} )); then sudo apt-get update; sudo apt-get install -y "${missing[@]}"; fi
elif command -v dnf >/dev/null; then
  missing=()
  for p in gcc pkgconf-pkg-config systemd-devel cargo rust python3; do
    if rpm -q "$p" >/dev/null; then echo "[OK] $p"; elif [[ $p == cargo ]] && command -v cargo >/dev/null; then echo "[OK] cargo"; elif [[ $p == rust ]] && command -v rustc >/dev/null; then echo "[OK] rustc"; else missing+=("$p"); fi
  done
  if (( ${#missing[@]} )); then sudo dnf install -y "${missing[@]}"; fi
else echo "Unsupported distribution"; exit 1; fi
cd "$ROOT/Core"
cargo test
cargo build --release
./target/release/nhc dependencies install --yes
if command -v systemctl >/dev/null && [[ -d /run/systemd/system ]]; then
  getent group nhc-sensors >/dev/null || sudo groupadd --system nhc-sensors
  sudo install -o root -g root -m 0755 -D "$ROOT/Install/Linux/nhc-sensor-helper.py" /usr/local/libexec/nhc-sensor-helper.py
  sudo install -o root -g root -m 0644 "$ROOT/Install/Linux/nhc-sensors.service" /etc/systemd/system/nhc-sensors.service
  sudo usermod -aG nhc-sensors "$(id -un)"
  sudo systemctl daemon-reload
  sudo systemctl enable nhc-sensors.service
  sudo systemctl restart nhc-sensors.service
  echo "[OK] NHC sensor publisher installed"
  echo "New login or newgrp nhc-sensors required if group was newly added."
else
  echo "[WARN] systemd unavailable; privileged sensors not configured"
fi
install -Dm755 target/release/nhc "$HOME/.local/bin/nhc"
echo "[OK] NHC installed: $HOME/.local/bin/nhc"
if systemctl --user show-environment >/dev/null 2>&1; then
  install -Dm644 "$ROOT/Install/Linux/nhc-core.service" "$HOME/.config/systemd/user/nhc-core.service"
  install -Dm644 "$ROOT/Install/Linux/nhc-touch.service" "$HOME/.config/systemd/user/nhc-touch.service"
  systemctl --user daemon-reload
  systemctl --user enable --now nhc-core.service
  systemctl --user restart nhc-core.service
  systemctl --user enable nhc-touch.service
  systemctl --user restart nhc-touch.service
  echo "[OK] NHC user collector and automatic Touch connection enabled"
fi
"$HOME/.local/bin/nhc" --version
"$HOME/.local/bin/nhc" doctor
echo "Add $HOME/.local/bin to PATH if needed. USB access may require distro-specific serial group and a new login."
