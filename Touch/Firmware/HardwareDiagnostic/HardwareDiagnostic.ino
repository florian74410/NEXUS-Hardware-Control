#include <Arduino.h>
#include "src/Model.h"
#include "src/Link.h"
#include "src/Panel.h"

QueueHandle_t snapshots;
static void dataTask(void*) {
  static Link link;
  link.begin();
  for (;;) {
    link.poll();
    vTaskDelay(pdMS_TO_TICKS(2));
  }
}
static void graphicsTask(void*) {
  // Zero-initialize library object storage before construction; keep UI ownership here.
  static Panel panel;
  panel.begin();
  Snapshot s{};
  for (;;) {
    xQueueReceive(snapshots, &s, 0);
    panel.tick(s);
    vTaskDelay(pdMS_TO_TICKS(10));
  }
}
void setup() {
  setCpuFrequencyMhz(240);
  Serial.begin(115200);
  snapshots = xQueueCreate(1, sizeof(Snapshot));
  if (!snapshots) { Serial.println("NHC queue allocation failed"); return; }
  if (xTaskCreatePinnedToCore(dataTask, "NHC_Data", 8192, nullptr, 2, nullptr, 0) != pdPASS)
    Serial.println("NHC data task failed");
  if (xTaskCreatePinnedToCore(graphicsTask, "NHC_UI", 8192, nullptr, 2, nullptr, 1) != pdPASS)
    Serial.println("NHC graphics task failed");
}
void loop() { delay(1000); }
