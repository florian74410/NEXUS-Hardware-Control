#pragma once
#include <Arduino.h>
#include <freertos/queue.h>
struct Snapshot {
  uint32_t heap, minHeap, flash, psram, mhz, packets, uptime;
  uint8_t dataCore;
  bool seen, recent;
};
extern QueueHandle_t snapshots;
