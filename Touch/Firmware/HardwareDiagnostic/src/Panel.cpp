#include "Panel.h"
#include <Preferences.h>
#include <cmath>
namespace {
constexpr uint16_t BG=0x0863, CYAN=0x06FF, DARK=0x19A6, WHITE=0xE71C, GREEN=0x27EB, RED=0xF92B;
}
void Panel::begin() {
  tft.init(); tft.setRotation(1);
  tft.setTextFont(2);
  tft.setTextSize(1);
  tft.setTextDatum(TL_DATUM);
  tft.setTextPadding(0);
  if(tft.width()!=480 || tft.height()!=320) return;
  tft.fillScreen(BG);
  Preferences prefs;
  bool stored=false;
  if(prefs.begin("nhc-touch",false)) {
    stored=prefs.getBytesLength("cal-v1")==sizeof(cal) &&
      prefs.getBytes("cal-v1",cal,sizeof(cal))==sizeof(cal) && cal[1]>0 && cal[3]>0;
    if(!stored) {
      tft.setTextColor(CYAN,BG); tft.drawCentreString("NEXUS / TOUCH CALIBRATION",240,140,2);
      tft.drawCentreString("Touch the four corner targets",240,165,2);
      tft.calibrateTouch(cal,CYAN,BG,15);
      prefs.putBytes("cal-v1",cal,sizeof(cal));
    }
    prefs.end();
  } else {
    tft.calibrateTouch(cal,CYAN,BG,15);
  }
  tft.setTouch(cal);
  ready=true; layout(); fpsStart=millis();
}
void Panel::layout() {
  tft.fillScreen(BG);
  tft.drawRect(6,6,468,308,DARK);
  tft.drawFastHLine(14,37,452,CYAN);
  tft.setTextColor(CYAN,BG);
  tft.drawString("NEXUS // INTERNAL MONITOR",18,15,2);
  tft.drawString(detail?"DIAGNOSTICS":"DUAL CORE // 240 MHz TARGET",18,46,2);
  if(!detail) {
    for(int x : {125,355}) {
      tft.drawCircle(x,135,65,CYAN); tft.drawCircle(x,135,70,DARK);
      for(int i=0;i<36;++i) {
        float a=i*0.174532925f;
        tft.drawLine(x+int(cosf(a)*73),135+int(sinf(a)*73),
          x+int(cosf(a)*76),135+int(sinf(a)*76),DARK);
      }
    }
    tft.drawCentreString("CORE 0 / DATA",125,100,2);
    tft.drawCentreString("CORE 1 / UI",355,100,2);
  }
  tft.drawRect(16,276,448,29,CYAN);
  tft.drawCentreString(detail?"TOUCH / OVERVIEW":"TOUCH / DETAILS",240,283,2);
}
void Panel::metrics(const Snapshot& s) {
  char buf[64];
  uint16_t col=s.recent?GREEN:(s.seen?RED:CYAN);
  tft.fillRect(319,12,142,20,BG);
  tft.setTextColor(col,BG);
  tft.drawRightString(s.recent?"RX ACTIVE":(s.seen?"LAST RX > 5s":"AWAIT HOST"),460,15,2);
  tft.setTextColor(WHITE,BG);
  if(!detail) {
    for(int x : {125,355}) {
      tft.fillRect(x-43,120,86,32,BG);
      if(s.mhz) snprintf(buf,sizeof(buf),"%lu",(unsigned long)s.mhz); else snprintf(buf,sizeof(buf),"--");
      tft.drawCentreString(buf,x,120,4);
      tft.drawCentreString("MHz",x,156,2);
    }
    tft.fillRect(16,222,448,47,BG);
    snprintf(buf,sizeof(buf),"HEAP %lu KiB   FLASH %lu MiB",
      (unsigned long)(s.heap/1024),(unsigned long)(s.flash/1048576));
    tft.drawCentreString(buf,240,225,2);
    snprintf(buf,sizeof(buf),"RX %lu   UI updates/s %lu",
      (unsigned long)s.packets,(unsigned long)fps);
    tft.drawCentreString(buf,240,247,2);
  } else {
    tft.fillRect(16,73,448,194,BG);
    const unsigned long vals[]={s.heap/1024,s.minHeap/1024,s.psram/1024,s.packets,s.uptime,
      s.dataCore,(unsigned long)xPortGetCoreID(),fps,renderUs};
    const char* labels[]={"Heap free KiB","Heap min KiB","PSRAM KiB (enabled)","RX requests","Uptime s","Data task core","UI task core","UI updates/s","Last draw us"};
    for(int i=0;i<9;i++) {
      tft.drawString(labels[i],24,75+i*20,2);
      snprintf(buf,sizeof(buf),"%lu",vals[i]); tft.drawRightString(buf,451,75+i*20,2);
    }
  }
}
void Panel::tick(const Snapshot& s) {
  if(!ready) return;
  uint16_t x=0,y=0;
  bool down=tft.getTouch(&x,&y);
  if(down&&!pressed&&x>=16&&x<=464&&y>=276&&y<=305) { detail=!detail;layout();lastDraw=millis()-250; }
  pressed=down;
  uint32_t now=millis();
  if(uint32_t(now-fpsStart)>=1000) {fps=frames*1000/(now-fpsStart);frames=0;fpsStart=now;}
  if(uint32_t(now-lastDraw)>=250) {
    lastDraw=now; uint32_t start=micros();metrics(s);renderUs=micros()-start;++frames;
  }
}
