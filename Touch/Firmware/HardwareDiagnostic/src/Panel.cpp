#include "Panel.h"
#include <Preferences.h>
#include <cmath>
namespace {
constexpr uint16_t BG=0x0863, CYAN=0x06FF, DARK=0x19A6, WHITE=0xE71C, GREEN=0x27EB, RED=0xF92B;
const char* labels[]={"Heap free KiB","Heap min KiB","PSRAM KiB (enabled)","RX requests","Uptime s","Data task core","UI task core","UI updates/s","Last draw us"};
}
void Panel::calibrate() {
  tft.fillScreen(BG);
  tft.setTextColor(CYAN,BG);
  tft.drawCentreString("NEXUS / TOUCH CALIBRATION",240,140,2);
  tft.drawCentreString("Touch each corner target",240,165,2);
  tft.calibrateTouch(cal,CYAN,BG,15);
  tft.setTouch(cal);
  Preferences prefs;
  if(prefs.begin("nhc-touch",false)) {
    prefs.putBytes("cal-v2",cal,sizeof(cal)); prefs.end();
  }
}
void Panel::begin() {
  tft.init(); tft.setRotation(1);
  tft.setTextFont(2); tft.setTextSize(1); tft.setTextDatum(TL_DATUM); tft.setTextPadding(0);
  if(tft.width()!=480 || tft.height()!=320) return;
  bool stored=false;
  Preferences prefs;
  if(prefs.begin("nhc-touch",true)) {
    stored=prefs.getBytesLength("cal-v2")==sizeof(cal) &&
      prefs.getBytes("cal-v2",cal,sizeof(cal))==sizeof(cal) && cal[1]>0 && cal[3]>0;
    prefs.end();
  }
  if(!stored) calibrate(); else tft.setTouch(cal);
  renderer.begin();
  ready=true; layout(); fpsStart=millis();
}
void Panel::layout() {
  renderer.invalidate();
  tft.fillScreen(BG); tft.drawRect(6,6,468,308,DARK);
  tft.drawFastHLine(14,37,452,CYAN); tft.setTextColor(CYAN,BG);
  tft.drawString("NEXUS // INTERNAL MONITOR",18,15,2);
  tft.drawString(detail?"DIAGNOSTICS":"DUAL CORE // SHARED CLOCK",18,46,2);
  if(!detail) {
    for(int x : {125,355}) {
      tft.drawCircle(x,135,65,CYAN); tft.drawCircle(x,135,70,DARK);
      for(int i=0;i<36;++i) {
        float a=i*0.174532925f;
        tft.drawLine(x+int(cosf(a)*73),135+int(sinf(a)*73),
          x+int(cosf(a)*76),135+int(sinf(a)*76),DARK);
      }
      tft.drawCentreString(x==125?"CORE 0 / DATA":"CORE 1 / UI",x,100,2);
      tft.drawCentreString("MHz",x,156,2);
    }
  } else {
    tft.setTextColor(WHITE,BG);
    for(int i=0;i<9;++i) tft.drawString(labels[i],24,75+i*20,2);
  }
  tft.setTextColor(CYAN,BG);
  tft.drawRect(16,276,215,29,CYAN); tft.drawRect(249,276,215,29,CYAN);
  tft.drawCentreString(detail?"OVERVIEW":"DETAILS",123,283,2);
  tft.drawCentreString("CALIBRATE",356,283,2);
}
bool Panel::metrics(const Snapshot& s) {
  char buf[80]; bool changed=false;
  uint16_t col=s.recent?GREEN:(s.seen?RED:CYAN);
  changed |= renderer.draw(0,319,12,142,20,s.recent?"RX ACTIVE":(s.seen?"LAST RX > 5s":"AWAIT HOST"),col,BG,2,true);
  if(!detail) {
    if(s.mhz) snprintf(buf,sizeof(buf),"%lu",(unsigned long)s.mhz); else strcpy(buf,"--");
    changed |= renderer.draw(1,82,120,86,32,buf,WHITE,BG,4);
    changed |= renderer.draw(2,312,120,86,32,buf,WHITE,BG,4);
    snprintf(buf,sizeof(buf),"HEAP %lu KiB   FLASH %lu MiB",(unsigned long)(s.heap/1024),(unsigned long)(s.flash/1048576));
    changed |= renderer.draw(3,16,225,448,19,buf,WHITE,BG);
    snprintf(buf,sizeof(buf),"RX %lu   UI updates/s %lu",(unsigned long)s.packets,(unsigned long)fps);
    changed |= renderer.draw(4,16,247,448,19,buf,WHITE,BG);
  } else {
    const unsigned long vals[]={s.heap/1024,s.minHeap/1024,s.psram/1024,s.packets,s.uptime,
      s.dataCore,(unsigned long)xPortGetCoreID(),fps,renderUs};
    for(int i=0;i<9;i++) {
      snprintf(buf,sizeof(buf),"%lu",vals[i]);
      changed |= renderer.draw(1+i,330,75+i*20,123,19,buf,WHITE,BG,2,true);
    }
  }
  return changed;
}
void Panel::tick(const Snapshot& s) {
  if(!ready) return;
  uint16_t x=0,y=0; bool down=tft.getTouch(&x,&y);
  if(down&&!pressed&&y>=276&&y<=305) {
    if(x>=16&&x<=231) {detail=!detail;layout(); lastDraw=millis()-250;}
    else if(x>=249&&x<=464) {
      // Wait for the triggering touch to be released before calibration.
      while(tft.getTouch(&x,&y)) delay(20);
      calibrate(); layout(); lastDraw=millis()-250;
      fpsStart=millis(); frames=0; fps=0;
    }
  }
  pressed=down;
  uint32_t now=millis();
  if(uint32_t(now-fpsStart)>=1000) {fps=frames*1000/(now-fpsStart);frames=0;fpsStart=now;}
  if(uint32_t(now-lastDraw)>=250) {
    lastDraw=now; uint32_t start=micros();
    if(metrics(s)) {renderUs=micros()-start;++frames;}
  }
}
