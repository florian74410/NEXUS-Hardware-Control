#pragma once
#include <TFT_eSPI.h>
#include "Effects.h"
// Vector-inspired procedural artwork, drawn only on page entry.
// Reserved decoration regions never overlap numeric text tiles.
namespace Backdrop {
inline void draw(TFT_eSPI& t,uint8_t page,uint16_t bg,uint16_t a){
 uint16_t dim=Effects::blend(bg,a,1,6), mid=Effects::blend(bg,a,1,3);
 t.drawLine(3,35,3,12,mid);t.drawLine(3,12,13,3,mid);t.drawFastHLine(13,3,110,dim);
 t.drawFastHLine(356,3,110,dim);t.drawLine(466,3,476,13,mid);t.drawFastVLine(476,13,22,mid);
 for(int i=0;i<12;i++){int x=182+i*8;t.drawLine(x,3,x+3,6,i%3==0?a:dim);}
 // Trace channels down both edges, not behind live data.
 for(int y=86;y<245;y+=32){
 t.drawLine(2,y,5,y+3,dim);t.drawFastVLine(5,y+3,8,mid);t.drawPixel(2,y,mid);
 t.drawLine(477,y,474,y+3,dim);t.drawFastVLine(474,y+3,8,mid);
 }
 // Page index navigator in the footer, 13 segments.
 for(int i=0;i<13;i++)t.fillRect(170+i*11,315,7,2,i==page?a:dim);
 // Holographic instrument reticles outside the metric update rectangles.
 if(page==0||page==1||page==2||page==5){
 for(int x:{125,355}){
 t.drawCircle(x,148,82,dim);
 for(int q=0;q<4;q++){float angle=(q*90+45)*0.0174532925f;int px=x+int(cosf(angle)*86),py=148+int(sinf(angle)*86);t.drawRect(px-2,py-2,5,5,mid);}
 }
 t.drawFastVLine(240,88,126,dim);
 for(int y=94;y<212;y+=18)t.drawFastHLine(237,y,7,mid);
 }else if(page!=12){
 for(int i=0;i<4;i++){int y=84+i*44;t.drawFastHLine(20,y-4,17,dim);t.drawFastHLine(443,y-4,17,mid);}
 }
}
}
