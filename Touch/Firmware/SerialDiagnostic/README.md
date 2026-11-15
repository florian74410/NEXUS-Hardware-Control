# Serial diagnostic (experimental)
Arduino ESP32 core 3.2.0 proposed; install ArduinoJson 7. Target ESP32 Dev Module, 4 MiB flash and 240 MHz. PSRAM disabled initially (reported zero does not prove absence). No LCD or GPIO changes. Upload only after verified private backup. This sketch answers hello/ping and reports CPU clock, heap and flash. No Wi-Fi/Bluetooth required. Not compiled or hardware-tested at publication.

## 0.0.2-serial
Removed WiFi and Bluetooth headers and calls; radios are not initialized by this sketch. Keep the default partition for the first size comparison. Protocol unchanged. Compile and flash validation pending. Baseline: 898306 program bytes, 49408 static RAM bytes.
