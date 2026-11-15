#pragma once
#include <TFT_eSPI.h>
#include <cmath>
// All effects are bounded, allocation-free and owned by the graphics task.
namespace Effects {
inline uint16_t rgb(int r,int g,int b){return uint16_t(((r&248)<<8)|((g&252)<<3)|(b>>3));}
inline uint16_t blend(uint16_t a,uint16_t b,int n,int max){
 n=constrain(n,0,max);
 return uint16_t(((((a>>11)*(max-n)+(b>>11)*n)/max)<<11)|
 (((((a>>5)&63)*(max-n)+((b>>5)&63)*n)/max)<<5)|
 (((a&31)*(max-n)+(b&31)*n)/max));
}
inline uint16_t spectrum(int p){
 p=constrain(p,0,100);
 constexpr uint16_t stops[]={0x041F,0x07FF,0xAFE0,0xFD20,0xF800};
 int section=p/25;if(section>=4)return stops[4];
 return blend(stops[section],stops[section+1],p%25,25);
}
inline int ease(int from,int to){if(from<0||to<0)return to;int delta=to-from;if(delta==0)return to;int step=(abs(delta)+3)/4;return from+(delta>0?step:-step);}
}
