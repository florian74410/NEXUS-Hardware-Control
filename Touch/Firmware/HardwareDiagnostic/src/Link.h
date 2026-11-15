#pragma once
#include "Model.h"
class Link {
  char frame[4097]{};
  size_t len=0;
  bool dropping=false, seen=false;
  uint32_t last=0, packets=0, published=0;
  void handle();
public:
  void begin();
  void poll();
};
