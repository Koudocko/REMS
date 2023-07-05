#ifndef CLOSURE
#define CLOSURE

#include "lib.h"

auto motionClosure = []{
  bool motion{};

  return [motion](uint* pin, uint id) mutable{
    jsonOutput.motion_detected[id] = motion;

    if (digitalRead(*pin)){
      if (!motion){
        motion = true;
      }
    }
    else{
      if (motion){
        motion = false;
      }
    }
  };
};

#endif
