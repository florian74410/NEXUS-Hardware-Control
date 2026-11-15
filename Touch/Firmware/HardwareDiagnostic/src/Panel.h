#pragma once
#include "Model.h"
#include <TFT_eSPI.h>
#include "RegionRenderer.h"
class Panel {
 TFT_eSPI tft; RegionRenderer renderer{tft};
 bool ready=false,pressed=false,pwm=false,saving=false;
 uint8_t page=0,brightness=30,palette=0; uint32_t lastDraw=0,saveAt=0;
 uint16_t cal[5]{};
 int bars[4]={-2,-2,-2,-2}; int gauges[2]={-2,-2};
 void layout(); void metrics(const Snapshot&); void calibrate();
 void light();void save();void gauge(int idx,int x,int y,float value,uint16_t color);
 void bar(int idx,int x,int y,int w,float value,uint16_t color);
 void field(int id,int x,int y,int w,float value,const char* unit,uint16_t color,int digits=1);
public: void begin();void tick(const Snapshot&);
};
