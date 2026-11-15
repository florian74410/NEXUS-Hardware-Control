#pragma once
#include <Arduino.h>
#include <cmath>
#include <freertos/queue.h>
struct Snapshot {
  uint32_t heap, minHeap, flash, psram, mhz, packets, uptime;
  uint8_t dataCore;
  bool seen, recent;
  float hostCpu=NAN, hostMhz=NAN, hostTemp=NAN, hostRamUsed=NAN, hostRamTotal=NAN, hostGpu=NAN, hostGpuAwake=NAN, hostGpuMhz=NAN, hostGpuTemp=NAN, hostGpuPower=NAN, hostVramUsed=NAN, hostVramTotal=NAN, hostCpuPower=NAN, hostSystemPower=NAN, hostFan=NAN, hostDiskTemp=NAN, hostDiskRead=NAN, hostDiskWrite=NAN, hostDiskCap=NAN, hostDiskWear=NAN, hostRx=NAN, hostTx=NAN, hostBattery=NAN, hostHostUptime=NAN, hostSensorCount=NAN, hostSensorMax=NAN, hostVoltageCount=NAN;
  int8_t hostDiskHealth=-1;
  bool hostFresh=false;
};
extern QueueHandle_t snapshots;
