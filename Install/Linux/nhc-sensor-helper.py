#!/usr/bin/python3
"""No request interface: publish a fixed set of sensor readings."""
import glob
import json
import os
import re
import shutil
import subprocess
import threading
import time
from pathlib import Path

RUNTIME = Path("/run/nhc-sensors")
SMARTCTL = next((p for p in ("/usr/sbin/smartctl", "/usr/bin/smartctl") if os.path.isfile(p)), None)

def publish(name, data):
    target = RUNTIME / (name + ".json")
    tmp = RUNTIME / (name + ".tmp")
    with tmp.open("w", encoding="utf-8") as stream:
        os.fchmod(stream.fileno(), 0o640)
        json.dump({"version": 1, "timestamp": time.time(), "data": data}, stream, allow_nan=False)
    os.replace(tmp, target)

def number(path):
    return int(path.read_text().strip())

def power_loop():
    previous = {}
    while True:
        start = time.monotonic()
        next_values, rows = {}, []
        paths = set()
        for pattern in ("/sys/class/powercap/*rapl*", "/sys/class/powercap/*rapl*/*rapl*", "/sys/class/powercap/*rapl*/*rapl*/*rapl*"):
            paths.update(Path(p).resolve() for p in glob.glob(pattern))
        for p in sorted(paths):
            try:
                energy = number(p / "energy_uj")
                label = (p / "name").read_text().strip()
                maximum = number(p / "max_energy_range_uj")
                now = time.monotonic()
                watts = None
                if p in previous:
                    old, t = previous[p]
                    delta = energy - old if energy >= old else maximum - old + energy
                    if 0 <= delta and now > t:
                        watts = delta / 1e6 / (now - t)
                next_values[p] = (energy, now)
                rows.append({"name": label, "source": str(p), "energy_uj": energy, "power_w": watts})
            except (OSError, ValueError):
                continue
        previous = next_values
        publish("rapl", rows)
        time.sleep(max(0.1, 1 - (time.monotonic() - start)))

def smart_sample():
    rows = []
    for p in sorted(Path("/sys/block").glob("*")):
        name = p.name
        if not re.fullmatch(r"(?:sd[a-z]+|hd[a-z]+|nvme[0-9]+n[0-9]+)", name):
            continue
        if SMARTCTL is None:
            rows.append({"device": name, "status": "tool_missing"})
            continue
        try:
            result = subprocess.run([SMARTCTL, "-j", "-a", "-n", "standby", "/dev/" + name],
                                    stdin=subprocess.DEVNULL, capture_output=True, timeout=5, check=False)
            v = json.loads(result.stdout)
            if result.returncode < 0 or result.returncode & 7:
                rows.append({"device": name, "status": "unavailable_or_standby", "exit_code": result.returncode})
                continue
            attrs = v.get("ata_smart_attributes", {}).get("table", [])
            rows.append({"device": name, "status": "read", "healthy": v.get("smart_status", {}).get("passed"),
                         "temperature_c": v.get("temperature", {}).get("current"),
                         "percentage_used": v.get("nvme_smart_health_information_log", {}).get("percentage_used"),
                         "power_on_hours": v.get("power_on_time", {}).get("hours"),
                         "ata_attributes": attrs})
        except (OSError, ValueError, subprocess.TimeoutExpired):
            rows.append({"device": name, "status": "unavailable"})
    return rows

def main():
    threading.Thread(target=power_loop, daemon=True).start()
    while True:
        publish("smart", smart_sample())
        time.sleep(60)

if __name__ == "__main__":
    main()
