#include "Link.h"
#include <ArduinoJson.h>
static constexpr int R=22,G=16,B=17;
static void rgb(bool r,bool g,bool b) {
  digitalWrite(R,!r); digitalWrite(G,!g); digitalWrite(B,!b);
}
void Link::begin() {
  for (int pin : {R,G,B}) { pinMode(pin,OUTPUT); digitalWrite(pin,HIGH); }
  rgb(false,false,true);
}
void Link::handle() {
  JsonDocument in;
  if(deserializeJson(in,frame,len)) return;
  const char* type=in["type"]|"";
  const char* nonce=in["nonce"]|"";
  if ((in["version"]|0)!=1 || !nonce[0] || strlen(nonce)>128) return;
  const char* reply=nullptr;
  if(!strcmp(type,"hello")) reply="hello_ack";
  else if(!strcmp(type,"ping")) reply="pong";
  else if(!strcmp(type,"telemetry")) {
    if(!in["data"].is<JsonObject>()) return;
    JsonVariant d=in["data"];
    host.hostCpu=d["cpu"]["usage_percent"].is<float>() ? d["cpu"]["usage_percent"].as<float>() : NAN;
    host.hostMhz=d["cpu"]["frequency_mhz"].is<float>() ? d["cpu"]["frequency_mhz"].as<float>() : NAN;
    host.hostTemp=d["cpu"]["temperature_c"].is<float>() ? d["cpu"]["temperature_c"].as<float>() : NAN;
    host.hostRamUsed=d["ram"]["used_mib"].is<float>() ? d["ram"]["used_mib"].as<float>() : NAN;
    host.hostRamTotal=d["ram"]["total_mib"].is<float>() ? d["ram"]["total_mib"].as<float>() : NAN;
    host.hostGpu=d["gpu"]["usage_percent"].is<float>() ? d["gpu"]["usage_percent"].as<float>() : NAN;
    host.hostGpuAwake=d["gpu"]["non_rc6_percent"].is<float>() ? d["gpu"]["non_rc6_percent"].as<float>() : NAN;
    host.hostGpuMhz=d["gpu"]["frequency_mhz"].is<float>() ? d["gpu"]["frequency_mhz"].as<float>() : NAN;
    host.hostGpuTemp=d["gpu"]["temperature_c"].is<float>() ? d["gpu"]["temperature_c"].as<float>() : NAN;
    host.hostGpuPower=d["gpu"]["power_w"].is<float>() ? d["gpu"]["power_w"].as<float>() : NAN;
    host.hostVramUsed=d["gpu"]["vram_used_mib"].is<float>() ? d["gpu"]["vram_used_mib"].as<float>() : NAN;
    host.hostVramTotal=d["gpu"]["vram_total_mib"].is<float>() ? d["gpu"]["vram_total_mib"].as<float>() : NAN;
    host.hostCpuPower=d["power"]["cpu_package_w"].is<float>() ? d["power"]["cpu_package_w"].as<float>() : NAN;
    host.hostSystemPower=d["power"]["system_total_w"].is<float>() ? d["power"]["system_total_w"].as<float>() : NAN;
    host.hostFan=d["fan"]["rpm"].is<float>() ? d["fan"]["rpm"].as<float>() : NAN;
    host.hostDiskTemp=d["storage"]["temperature_c"].is<float>() ? d["storage"]["temperature_c"].as<float>() : NAN;
    host.hostDiskRead=d["storage"]["read_bytes_s"].is<float>() ? d["storage"]["read_bytes_s"].as<float>() : NAN;
    host.hostDiskWrite=d["storage"]["write_bytes_s"].is<float>() ? d["storage"]["write_bytes_s"].as<float>() : NAN;
    host.hostDiskCap=d["storage"]["capacity_bytes"].is<float>() ? d["storage"]["capacity_bytes"].as<float>() : NAN;
    host.hostDiskWear=d["storage"]["percentage_used"].is<float>() ? d["storage"]["percentage_used"].as<float>() : NAN;
    host.hostRx=d["network"]["rx_bytes_s"].is<float>() ? d["network"]["rx_bytes_s"].as<float>() : NAN;
    host.hostTx=d["network"]["tx_bytes_s"].is<float>() ? d["network"]["tx_bytes_s"].as<float>() : NAN;
    host.hostBattery=d["battery"]["capacity_percent"].is<float>() ? d["battery"]["capacity_percent"].as<float>() : NAN;
    host.hostUptime=d["system"]["uptime_seconds"].is<float>() ? d["system"]["uptime_seconds"].as<float>() : NAN;
    host.hostSensorCount=d["thermal"]["count"].is<float>() ? d["thermal"]["count"].as<float>() : NAN;
    host.hostSensorMax=d["thermal"]["max_c"].is<float>() ? d["thermal"]["max_c"].as<float>() : NAN;
    host.hostVoltageCount=d["motherboard"]["voltage_count"].is<float>() ? d["motherboard"]["voltage_count"].as<float>() : NAN;
    host.hostRootUsage=d["storage"]["root_filesystem_usage_percent"].is<float>()?d["storage"]["root_filesystem_usage_percent"].as<float>():NAN;
    host.hostDiskHealth=d["storage"]["healthy"].is<bool>() ? (d["storage"]["healthy"].as<bool>()?1:0) : -1;
    hostSeen=true; hostLast=millis(); reply="telemetry_ack";
  } else return;
  seen=true; last=millis(); ++packets;
  JsonDocument out;
  out["type"]=reply; out["version"]=1; out["nonce"]=nonce;
  out["device"]="nhc-touch"; out["firmware"]="0.1.6-neon-icons";
  out["telemetry"]=true; out["telemetry_extended"]=true;
  out["cpu_mhz"]=getCpuFrequencyMhz(); out["heap_free"]=ESP.getFreeHeap();
  out["heap_min"]=ESP.getMinFreeHeap(); out["flash_bytes"]=ESP.getFlashChipSize();
  out["psram_bytes"]=ESP.getPsramSize(); out["data_core"]=xPortGetCoreID();
  out["packets"]=packets; out["uptime_ms"]=millis();
  serializeJson(out,Serial); Serial.println();
}
void Link::poll() {
  for(int i=0; i<512 && Serial.available(); ++i) {
    char c=(char)Serial.read();
    if(c=='\n') { if(!dropping && len) handle(); len=0; dropping=false; }
    else if(!dropping) { if(len<sizeof(frame)-1) frame[len++]=c; else {dropping=true;len=0;} }
  }
  uint32_t now=millis();
  bool recent=seen && uint32_t(now-last)<5000;
  if(recent) rgb(false,true,false);
  else if(seen) rgb(true,false,false);
  else rgb(false,false,(now/500)%2);
  if(uint32_t(now-published)>=200) {
    published=now;
    Snapshot s{ESP.getFreeHeap(),ESP.getMinFreeHeap(),ESP.getFlashChipSize(),
      ESP.getPsramSize(),getCpuFrequencyMhz(),packets,now/1000,
      uint8_t(xPortGetCoreID()),seen,recent};
    s.hostCpu=host.hostCpu;
    s.hostMhz=host.hostMhz;
    s.hostTemp=host.hostTemp;
    s.hostRamUsed=host.hostRamUsed;
    s.hostRamTotal=host.hostRamTotal;
    s.hostGpu=host.hostGpu;
    s.hostGpuAwake=host.hostGpuAwake;
    s.hostGpuMhz=host.hostGpuMhz;
    s.hostGpuTemp=host.hostGpuTemp;
    s.hostGpuPower=host.hostGpuPower;
    s.hostVramUsed=host.hostVramUsed;
    s.hostVramTotal=host.hostVramTotal;
    s.hostCpuPower=host.hostCpuPower;
    s.hostSystemPower=host.hostSystemPower;
    s.hostFan=host.hostFan;
    s.hostDiskTemp=host.hostDiskTemp;
    s.hostDiskRead=host.hostDiskRead;
    s.hostDiskWrite=host.hostDiskWrite;
    s.hostDiskCap=host.hostDiskCap;
    s.hostDiskWear=host.hostDiskWear;
    s.hostRx=host.hostRx;
    s.hostTx=host.hostTx;
    s.hostBattery=host.hostBattery;
    s.hostUptime=host.hostUptime;
    s.hostSensorCount=host.hostSensorCount;
    s.hostSensorMax=host.hostSensorMax;
    s.hostVoltageCount=host.hostVoltageCount;
    s.hostRootUsage=host.hostRootUsage;
    s.hostDiskHealth=host.hostDiskHealth;
    s.hostFresh=hostSeen && uint32_t(now-hostLast)<3500;
    xQueueOverwrite(snapshots,&s);
  }
}
