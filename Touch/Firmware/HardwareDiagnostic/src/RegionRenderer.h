#pragma once
#include <TFT_eSPI.h>
#include <cstring>
// One reusable off-screen RGB565 tile; all access belongs to the UI task.
class RegionRenderer {
  TFT_eSPI& lcd;
  TFT_eSprite tile;
  struct Entry { char text[80]{}; uint16_t color=0; bool valid=false; };
  Entry cache[16]{};
  bool buffered=false;
public:
  explicit RegionRenderer(TFT_eSPI& display):lcd(display),tile(&display){}
  void begin() {
    tile.setColorDepth(16);
    buffered=tile.createSprite(448,32)!=nullptr;
    if(buffered) { tile.setTextFont(2); tile.setTextSize(1); tile.setTextPadding(0); }
  }
  void invalidate() { for(auto& e:cache)e.valid=false; }
  bool draw(unsigned id,int x,int y,int w,int h,const char* text,uint16_t fg,uint16_t bg,uint8_t font=2,bool right=false) {
    if(id>=16 || w<=0 || w>448 || h<=0 || h>32 || strlen(text)>=sizeof(cache[id].text)) return false;
    auto& e=cache[id];
    if(e.valid && e.color==fg && strcmp(e.text,text)==0) return false;
    if(buffered) {
      tile.fillSprite(bg);
      tile.setTextColor(fg,bg);
      if(right) tile.drawRightString(text,448-2,0,font);
      else tile.drawCentreString(text,448/2,0,font);
      // Transfer only the centered source rectangle, not the full tile.
      int left=right?448-w:(448-w)/2;
      tile.pushSprite(x,y,left,0,w,h);
    } else {
      lcd.fillRect(x,y,w,h,bg); lcd.setTextColor(fg,bg);
      if(right) lcd.drawRightString(text,x+w-2,y,font);
      else lcd.drawCentreString(text,x+w/2,y,font);
    }
    strcpy(e.text,text); e.color=fg; e.valid=true; return true;
  }
};
