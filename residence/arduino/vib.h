#ifndef VIB
#define VIB

#include "lib.h"

auto vibClosure = [](uint* pins, uint id){
  bool trigger = digitalRead(pins[0]);
  /* digitalWrite(pins[1], !trigger); */

  jsonOutput.sound_sensor[id] = trigger;
};

#endif
