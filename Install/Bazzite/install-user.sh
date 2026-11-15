#!/usr/bin/env bash
set -euo pipefail
# Run on HOST; provide a compatible Linux release binary built outside the immutable host.
[[ $EUID -ne 0 ]] || { echo "Run as normal user"; exit 1; }
[[ $# -eq 1 && -f "$1" ]] || { echo "Usage: bash Install/Bazzite/install-user.sh /path/to/nhc"; exit 1; }
ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)"
install -Dm755 "$1" "$HOME/.local/bin/nhc"
"$HOME/.local/bin/nhc" --version
install -Dm644 "$ROOT/Install/Linux/nhc-core.service" "$HOME/.config/systemd/user/nhc-core.service"
install -Dm644 "$ROOT/Install/Linux/nhc-touch.service" "$HOME/.config/systemd/user/nhc-touch.service"
systemctl --user daemon-reload
systemctl --user enable --now nhc-core.service nhc-touch.service
echo "Host user services installed. Verify serial group and optional sensor helpers separately."
