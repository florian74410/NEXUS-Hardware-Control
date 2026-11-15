#pragma once
#include "Model.h"
#include <TFT_eSPI.h>
#include "RegionRenderer.h"
class Panel {
  TFT_eSPI tft;
  RegionRenderer renderer{tft};
  bool ready=false, pressed=false, detail=false; uint8_t page=0;
  uint32_t lastDraw=0, frames=0, fpsStart=0, fps=0, renderUs=0;
  uint16_t cal[5]{};
  void layout();
  bool metrics(const Snapshot&);
  void calibrate();
public:
  void begin();
  void tick(const Snapshot&);
};
