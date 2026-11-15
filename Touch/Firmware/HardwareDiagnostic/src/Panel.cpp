#include "Panel.h"
#include <Preferences.h>
#include <cmath>
namespace {
constexpr uint16_t BG=0x0863, PANEL=0x10A5, DARK=0x2188, WHITE=0xE71C, GREEN=0x27EB, RED=0xF92B, BLUE=0x06FF, AMBER=0xFD20;
uint16_t accents[]={BLUE,RED,0x37E8};
const char* pages[]={"OVERVIEW","CPU","GPU","RAM","POWER","COOLING","STORAGE","NETWORK","THERMAL","BOARD","SYSTEM","ESP32","SETTINGS"};
constexpr int COUNT=13;
float percent(float a,float b){return isfinite(a)&&isfinite(b)&&b>0?100*a/b:NAN;}
int level(float n,int max){return isfinite(n)?int(roundf(fminf(100,fmaxf(0,n))*max/100)):-1;}
void label(TFT_eSPI& t,const char* s,int x,int y,uint16_t c){t.setTextColor(c,BG);t.drawString(s,x,y,2);}
}
void Panel::calibrate() {
  tft.fillScreen(BG);
  tft.setTextColor(accents[palette],BG);
  tft.drawCentreString("NEXUS / TOUCH CALIBRATION",240,140,2);
  tft.drawCentreString("Touch each corner target",240,165,2);
  tft.calibrateTouch(cal,accents[palette],BG,15);
  tft.setTouch(cal);
  Preferences prefs;
  if(prefs.begin("nhc-touch",false)) {
    prefs.putBytes("cal-v2",cal,sizeof(cal)); prefs.end();
  }
}

void Panel::light(){if(pwm)ledcWrite(27,(brightness*255+50)/100);}
void Panel::save(){Preferences p;if(p.begin("nhc-touch",false)){p.putUChar("brightness",brightness);p.putUChar("palette",palette);p.end();}saving=false;}
void Panel::begin(){
 Serial.println("NHC UI: init");
 tft.init();tft.setRotation(1);tft.setTextFont(2);tft.setTextSize(1);tft.setTextDatum(TL_DATUM);tft.setTextPadding(0);
 Serial.printf("NHC UI: size %d x %d\n",tft.width(),tft.height());
 if(tft.width()!=480||tft.height()!=320){Serial.println("NHC UI: unexpected dimensions");return;}
 Preferences p;bool stored=false;
 if(p.begin("nhc-touch",true)){brightness=constrain(p.getUChar("brightness",30),5,100);palette=p.getUChar("palette",0)%3;stored=p.getBytesLength("cal-v2")==sizeof(cal)&&p.getBytes("cal-v2",cal,sizeof(cal))==sizeof(cal)&&cal[1]>0&&cal[3]>0;p.end();}
 // Recovery build: restore TFT_eSPI backlight/SPI ownership, no LEDC attach.
 pwm=false;
 Serial.println("NHC UI: PWM bypass; TFT_eSPI backlight");
 tft.fillScreen(BLUE);delay(250);
 if(!stored)calibrate();else tft.setTouch(cal);
 Serial.println("NHC UI: renderer begin");
 renderer.begin();ready=true;layout();
 Serial.println("NHC UI: ready");
}
void Panel::layout(){
 renderer.invalidate();for(int& v:bars)v=-2;for(int& v:gauges)v=-2;
 uint16_t a=accents[palette];tft.fillScreen(BG);
 tft.drawFastHLine(12,39,456,a);tft.drawFastHLine(12,269,456,DARK);
 tft.fillRect(12,10,5,20,a);label(tft,"NEXUS // HARDWARE",24,12,a);
 label(tft,pages[page],16,48,a);label(tft,"SET",410,48,a);
 for(int k=0;k<3;k++){int x=12+k*155;tft.drawRect(x,278,145,30,a);}
 tft.setTextColor(a,BG);tft.drawCentreString("< PREV",84,284,2);tft.drawCentreString("HOME",239,284,2);tft.drawCentreString("NEXT >",394,284,2);
 if(page==12){
 label(tft,"BACKLIGHT / PWM",20,81,a);tft.drawRect(24,111,432,26,DARK);
 tft.drawRect(24,154,132,36,a);tft.drawRect(174,154,132,36,a);tft.drawRect(324,154,132,36,a);
 tft.setTextColor(a,BG);tft.drawCentreString("NIGHT 10%",90,164,2);tft.drawCentreString("NORMAL 30%",240,164,2);tft.drawCentreString("MAX 100%",390,164,2);
 tft.drawRect(24,211,202,39,a);tft.drawRect(254,211,202,39,a);tft.drawCentreString("CHANGE COLOR",125,222,2);tft.drawCentreString("CALIBRATE",355,222,2);
 }else if(page==0||page==1||page==2||page==5){
 for(int x:{125,355}){tft.drawCircle(x,148,76,DARK);for(int i=0;i<=30;i++){float r=(135+i*9)*0.0174532925f;tft.drawLine(x+cosf(r)*69,148+sinf(r)*69,x+cosf(r)*75,148+sinf(r)*75,DARK);}}
 }else{
 for(int i=0;i<4;i++){int y=84+i*44;tft.drawFastHLine(20,y+32,440,DARK);}
 const char* names[4]={nullptr,nullptr,nullptr,nullptr};
 switch(page){
 case 3:names[0]="RAM USED";names[1]="RAM TOTAL";names[2]="UTILIZATION";break;
 case 4:names[0]="CPU PACKAGE";names[1]="GPU POWER";names[2]="SYSTEM TOTAL";names[3]="BATTERY";break;
 case 6:names[0]="SSD TEMP";names[1]="READ / WRITE MiB/s";names[2]="ROOT FS USED";names[3]="SMART";break;
 case 7:names[0]="DOWNLOAD";names[1]="UPLOAD";break;
 case 8:names[0]="SENSORS";names[1]="MAX READING";names[2]="CPU";names[3]="GPU";break;
 case 9:names[0]="VOLTAGE CHANNELS";names[1]="THERMAL CHANNELS";break;
 case 10:names[0]="HOST UPTIME";names[1]="BATTERY";break;
 case 11:names[0]="FREE HEAP";names[1]="CLOCK";names[2]="RX PACKETS";names[3]="UPTIME";break;
 }
 for(int i=0;i<4;i++)if(names[i])label(tft,names[i],20,84+i*44,WHITE);
 }
}
void Panel::field(int id,int x,int y,int w,float value,const char* unit,uint16_t color,int digits){
 char buf[48];if(isfinite(value))snprintf(buf,sizeof(buf),"%.*f %s",digits,value,unit);else strcpy(buf,"N/A");
 renderer.draw(id,x,y,w,32,buf,color,BG,2,true);
}
void Panel::gauge(int idx,int x,int y,float value,uint16_t color){
 int n=level(value,30);int old=gauges[idx];if(n==old)return;
 for(int i=0;i<30;i++){bool on=i<n;bool before=i<old;if(old!=-2&&on==before)continue;
 float r=(139.5f+i*9)*0.0174532925f;tft.drawLine(x+cosf(r)*59,y+sinf(r)*59,x+cosf(r)*66,y+sinf(r)*66,on?color:DARK);}
 gauges[idx]=n;
}
void Panel::bar(int idx,int x,int y,int w,float value,uint16_t color){
 int n=level(value,w);int old=bars[idx];if(n==old)return;
 if(old<0){tft.fillRect(x,y,w,8,DARK);old=0;}int v=n<0?0:n;
 if(v>old)tft.fillRect(x+old,y,v-old,8,color);else if(v<old)tft.fillRect(x+v,y,old-v,8,DARK);
 bars[idx]=n;
}
void Panel::metrics(const Snapshot& s){
 uint16_t a=accents[palette];bool fresh=s.hostFresh;auto v=[&](float x){return fresh?x:NAN;};
 renderer.draw(0,326,12,140,20,fresh?"LINK ONLINE":"HOST OFFLINE",fresh?GREEN:RED,BG,2,true);
 auto row=[&](int i,const char* name,float value,const char* unit,int digits=1){int y=84+i*44;(void)name;field(i+1,242,y,214,value,unit,a,digits);};
 auto dial=[&](int i,const char* name,float val,const char* unit,float fill){int x=i?355:125;gauge(i,x,148,fill,a);char b[40];if(isfinite(val))snprintf(b,sizeof(b),"%.1f %s",val,unit);else strcpy(b,"N/A");renderer.draw(i+1,x-54,134,108,32,b,WHITE,BG,4);renderer.draw(5+i,x-55,177,110,20,name,a,BG,2);};
 switch(page){
 case 0:dial(0,"CPU LOAD",v(s.hostCpu),"%",v(s.hostCpu));dial(1,"RAM",v(percent(s.hostRamUsed,s.hostRamTotal)),"%",v(percent(s.hostRamUsed,s.hostRamTotal)));field(3,20,235,205,v(s.hostTemp),"C",a);field(4,245,235,205,v(s.hostCpuPower),"W CPU",a);break;
 case 1:dial(0,"CPU LOAD",v(s.hostCpu),"%",v(s.hostCpu));dial(1,"CPU TEMP",v(s.hostTemp),"C",v(s.hostTemp));field(3,20,235,205,v(s.hostMhz),"MHz",a,0);field(4,245,235,205,v(s.hostCpuPower),"W",a);break;
 case 2:{bool real=isfinite(s.hostGpu);float load=real?s.hostGpu:s.hostGpuAwake;dial(0,real?"GPU LOAD":"NON-RC6",v(load),"%",v(load));dial(1,"GPU TEMP",v(s.hostGpuTemp),"C",v(s.hostGpuTemp));field(3,20,235,205,v(s.hostGpuMhz),"MHz",a,0);field(4,245,235,205,v(s.hostGpuPower),"W",a);break;}
 case 3:row(0,"RAM USED",v(s.hostRamUsed),"MiB",0);row(1,"RAM TOTAL",v(s.hostRamTotal),"MiB",0);row(2,"UTILIZATION",v(percent(s.hostRamUsed,s.hostRamTotal)),"%");bar(0,20,242,436,v(percent(s.hostRamUsed,s.hostRamTotal)),a);break;
 case 4:row(0,"CPU PACKAGE",v(s.hostCpuPower),"W");row(1,"GPU POWER",v(s.hostGpuPower),"W");row(2,"SYSTEM TOTAL",v(s.hostSystemPower),"W");row(3,"BATTERY",v(s.hostBattery),"%");break;
 case 5:dial(0,"FAN RPM",v(s.hostFan),"",NAN);dial(1,"CPU TEMP",v(s.hostTemp),"C",v(s.hostTemp));break;
 case 6:{row(0,"SSD TEMP",v(s.hostDiskTemp),"C");char io[48];if(fresh&&isfinite(s.hostDiskRead)&&isfinite(s.hostDiskWrite))snprintf(io,sizeof(io),"%.2f / %.2f",s.hostDiskRead/1048576,s.hostDiskWrite/1048576);else strcpy(io,"N/A");renderer.draw(2,252,128,196,20,io,a,BG,2,true);row(2,"ROOT FS USED",v(s.hostRootUsage),"%");renderer.draw(8,252,216,196,20,!fresh||s.hostDiskHealth<0?"N/A":s.hostDiskHealth?"PASSED":"FAILED",fresh&&s.hostDiskHealth==1?GREEN:RED,BG,2,true);bar(0,20,247,436,v(s.hostRootUsage),a);break;}
 case 7:row(0,"DOWNLOAD",v(s.hostRx/1024),"KiB/s");row(1,"UPLOAD",v(s.hostTx/1024),"KiB/s");break;
 case 8:row(0,"SENSORS",v(s.hostSensorCount),"",0);row(1,"MAX READING",v(s.hostSensorMax),"C");row(2,"CPU",v(s.hostTemp),"C");row(3,"GPU",v(s.hostGpuTemp),"C");break;
 case 9:row(0,"VOLTAGE CHANNELS",v(s.hostVoltageCount),"",0);row(1,"THERMAL CHANNELS",v(s.hostSensorCount),"",0);break;
 case 10:row(0,"HOST UPTIME",v(s.hostUptime/3600),"h");row(1,"BATTERY",v(s.hostBattery),"%");break;
 case 11:row(0,"FREE HEAP",s.heap/1024.0f,"KiB");row(1,"CLOCK",s.mhz,"MHz",0);row(2,"RX PACKETS",s.packets,"",0);row(3,"UPTIME",s.uptime,"s",0);break;
 case 12:field(1,270,79,180,brightness,"%",a,0);bar(0,27,120,426,brightness,a);if(!pwm)label(tft,"PWM bypass / recovery",20,250,RED);break;
 }
}
void Panel::tick(const Snapshot& s){
 if(!ready)return;uint16_t x=0,y=0;bool down=tft.getTouch(&x,&y);
 if(down&&!pressed){
 if(y>=44&&y<=72&&x>=390){page=12;layout();}
 else if(y>=278&&y<=308){if(x<157)page=(page+COUNT-1)%COUNT;else if(x<312)page=0;else page=(page+1)%COUNT;layout();}
 else if(page==12&&y>=211&&y<=250){if(x<240){palette=(palette+1)%3;saving=true;saveAt=millis();layout();}else{while(tft.getTouch(&x,&y))delay(20);calibrate();layout();}}
 }
 if(down&&page==12){int b=-1;if(y>=108&&y<=140&&x>=24&&x<=456)b=constrain(5+(int(x)-24)*95/432,5,100);else if(!pressed&&y>=154&&y<=190)b=x<166?10:x<314?30:100;if(b>=0&&b!=brightness){brightness=b;light();saving=true;saveAt=millis();}}
 pressed=down;if(saving&&!down&&millis()-saveAt>=800)save();
 if(millis()-lastDraw>=250){lastDraw=millis();metrics(s);}
}
