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
  else return;
  seen=true; last=millis(); ++packets;
  JsonDocument out;
  out["type"]=reply; out["version"]=1; out["nonce"]=nonce;
  out["device"]="nhc-touch"; out["firmware"]="0.0.3-diagnostic";
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
    xQueueOverwrite(snapshots,&s);
  }
}
