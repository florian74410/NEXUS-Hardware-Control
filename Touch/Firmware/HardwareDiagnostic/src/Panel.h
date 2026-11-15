#pragma once
#include "Model.h"
#include <TFT_eSPI.h>
class Panel {
  TFT_eSPI tft;
  bool ready=false, pressed=false, detail=false;
  uint32_t lastDraw=0, frames=0, fpsStart=0, fps=0, renderUs=0;
  uint16_t cal[5]{};
  void layout();
  void metrics(const Snapshot&);
public:
  void begin();
  void tick(const Snapshot&);
};
