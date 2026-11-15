#pragma once
#include <TFT_eSPI.h>
#include <cmath>
// 16px monochrome glyphs. Each primitive stays inside its own cell.
namespace Icons {
enum Kind {Radar,Chip,Gpu,Ram,Bolt,Fan,Disk,Network,Temp,Board,Terminal,Esp,Settings,Clock,Battery,Down,Up,Shield,Wave,Usb,Home,Left,Right,Sun};
inline void draw(TFT_eSPI& t,Kind k,int x,int y,uint16_t c){
 auto l=[&](int a,int b,int d,int e){t.drawLine(x+a,y+b,x+d,y+e,c);};
 auto r=[&](int a,int b,int w,int h){t.drawRect(x+a,y+b,w,h,c);};
 auto o=[&](int a,int b,int rad){t.drawCircle(x+a,y+b,rad,c);};
 switch(k){
 case Radar:o(8,8,7);o(8,8,4);l(8,8,13,3);break;
 case Chip:case Esp:r(3,3,11,11);r(6,6,5,5);for(int i=4;i<=12;i+=4){l(i,0,i,2);l(i,14,i,16);l(0,i,2,i);l(14,i,16,i);}break;
 case Gpu:r(0,3,16,10);o(5,8,3);o(12,8,3);l(3,14,12,14);break;
 case Ram:r(0,3,16,10);for(int i=2;i<14;i+=4)r(i,5,3,5);l(2,14,6,14);l(9,14,14,14);break;
 case Bolt:l(9,0,3,9);l(3,9,8,9);l(8,9,6,16);l(6,16,14,6);l(14,6,9,6);l(9,6,9,0);break;
 case Fan:o(8,8,7);o(8,8,2);l(8,6,5,2);l(5,2,2,6);l(10,8,14,5);l(14,5,10,2);l(8,10,11,14);l(11,14,14,10);l(6,8,2,11);l(2,11,6,14);break;
 case Disk:r(1,2,15,13);l(3,5,13,5);l(3,8,13,8);o(4,12,1);break;
 case Network:o(8,13,1);l(5,10,8,7);l(8,7,11,10);l(2,7,8,1);l(8,1,14,7);break;
 case Temp:o(8,12,3);l(6,10,6,2);l(10,10,10,2);l(6,2,10,2);l(8,5,8,12);break;
 case Board:r(1,1,15,15);r(4,4,6,6);l(10,7,14,7);l(7,10,7,14);l(3,12,3,14);break;
 case Terminal:r(1,2,15,12);l(3,5,6,8);l(6,8,3,11);l(8,11,13,11);break;
 case Settings:o(8,8,5);o(8,8,2);for(int i=0;i<8;i++){float a=i*0.78539816f;l(8+int(cosf(a)*5),8+int(sinf(a)*5),8+int(cosf(a)*8),8+int(sinf(a)*8));}break;
 case Clock:o(8,8,7);l(8,3,8,8);l(8,8,12,10);break;
 case Battery:r(0,4,14,9);r(14,6,2,5);l(3,7,10,7);l(3,9,10,9);break;
 case Down:case Up:{int a=k==Down?14:2,b=k==Down?9:7;l(8,2,8,14);l(3,b,8,a);l(8,a,13,b);break;}
 case Shield:l(8,1,2,4);l(2,4,3,11);l(3,11,8,15);l(8,15,13,11);l(13,11,14,4);l(14,4,8,1);l(5,8,7,10);l(7,10,11,6);break;
 case Wave:l(0,8,4,8);l(4,8,6,2);l(6,2,10,14);l(10,14,12,8);l(12,8,16,8);break;
 case Usb:l(8,1,8,14);l(8,1,5,4);l(8,1,11,4);l(8,10,3,7);l(8,8,13,5);o(3,6,1);r(12,3,3,3);o(8,15,1);break;
 case Home:l(1,7,8,1);l(8,1,15,7);r(4,7,9,9);r(7,11,3,5);break;
 case Left:case Right:{int a=k==Left?2:14,b=k==Left?10:6;l(2,8,14,8);l(a,8,b,2);l(a,8,b,14);break;}
 case Sun:o(8,8,4);for(int i=0;i<8;i++){float a=i*0.78539816f;l(8+int(cosf(a)*6),8+int(sinf(a)*6),8+int(cosf(a)*8),8+int(sinf(a)*8));}break;
 }
}
inline Kind page(uint8_t p){const Kind a[]={Radar,Chip,Gpu,Ram,Bolt,Fan,Disk,Network,Temp,Board,Terminal,Esp,Settings};return a[p<13?p:0];}
inline Kind metric(int p,int i){
 switch(p){
 case 3:return Ram;
 case 4:return i==3?Battery:Bolt;
 case 6:return i==0?Temp:i==1?Wave:i==2?Disk:Shield;
 case 7:return i==0?Down:Up;
 case 8:return i==0?Board:Temp;
 case 9:return i==0?Bolt:Temp;
 case 10:return i==0?Clock:Battery;
 case 11:return i==0?Ram:i==1?Wave:i==2?Usb:Clock;
 default:return page(p);
 }
}
}
