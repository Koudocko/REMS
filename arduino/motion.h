#ifndef CLOSURE
#define CLOSURE

#include "lib.h"

struct MotionModule : Module{
  MotionModule() = default;

  MotionModule(unsigned long delay, void (*callback)(uint*, uint, bool), uint* pins, uint id)
    : Module(delay, pins, id), callback(callback){}

  void handler(){
    callback(pins, id, motion);
  }

private:
  bool motion{};
  void (*callback)(uint*, uint, bool);
};

auto motionClosure = [](uint* pin, uint id, bool motion){
  jsonOutput.motion_detected[id] = digitalRead(*pin);

  /*if (digitalRead(*pin)){
    if (!motion){
      motion = true;
    }
  }
  else{
    if (motion){
      motion = false;
    }
  }*/
};

#endif
