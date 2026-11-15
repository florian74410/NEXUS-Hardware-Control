// NHC preliminary serial-only diagnostic. No display pins or flash layout assumptions.
#include <Arduino.h>
#include <ArduinoJson.h>

static char frame[4097];
static size_t len = 0;
static bool dropping = false;

void handleFrame() {
  JsonDocument in;
  if (deserializeJson(in, frame, len)) return;
  const char* type = in["type"] | "";
  const char* nonce = in["nonce"] | "";
  if ((in["version"] | 0) != 1 || !nonce[0] || strlen(nonce) > 128) return;
  const char* reply = nullptr;
  if (!strcmp(type, "hello")) reply = "hello_ack";
  else if (!strcmp(type, "ping")) reply = "pong";
  else return;
  JsonDocument out;
  out["type"] = reply;
  out["version"] = 1;
  out["nonce"] = nonce;
  out["device"] = "nhc-touch";
  out["firmware"] = "0.0.2-serial";
  out["cpu_mhz"] = getCpuFrequencyMhz();
  out["heap_free"] = ESP.getFreeHeap();
  out["flash_bytes"] = ESP.getFlashChipSize();
  out["psram_bytes"] = ESP.getPsramSize();
  serializeJson(out, Serial);
  Serial.println();
}
void setup() {
  setCpuFrequencyMhz(240);
  // Radios are deliberately never initialized in this serial-only build.
  Serial.begin(115200);
}
void loop() {
  while (Serial.available()) {
    char c = (char)Serial.read();
    if (c == '\n') {
      if (!dropping && len) handleFrame();
      len = 0; dropping = false;
    } else if (!dropping) {
      if (len < sizeof(frame)-1) frame[len++] = c;
      else { dropping = true; len = 0; }
    }
  }
  delay(2);
}
