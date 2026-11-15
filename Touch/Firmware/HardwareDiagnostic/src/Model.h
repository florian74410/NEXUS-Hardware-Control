#pragma once
#include <Arduino.h>
#include <freertos/queue.h>
struct Snapshot {
  uint32_t heap, minHeap, flash, psram, mhz, packets, uptime;
  uint8_t dataCore;
  bool seen, recent;
  float hostCpu=-1, hostMhz=-1, hostTemp=-999, hostRamUsed=-1, hostRamTotal=-1;
  bool hostFresh=false;
};
extern QueueHandle_t snapshots;
