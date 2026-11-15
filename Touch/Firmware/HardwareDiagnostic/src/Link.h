#pragma once
#include "Model.h"
class Link {
  char frame[4097]{};
  size_t len=0;
  bool dropping=false, seen=false;
  uint32_t last=0, packets=0, published=0;
  Snapshot host{};
  uint32_t hostLast=0;
  bool hostSeen=false;
  void handle();
public:
  void begin();
  void poll();
};
