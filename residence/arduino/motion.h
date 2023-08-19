#ifndef CLOSURE
#define CLOSURE

#include "lib.h"

// Provides a motion sensor specific implementation for the handler() when execute() is called
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

// Data retrieval callback 
auto motionClosure = [](uint* pin, uint id, bool motion){
  jsonOutput.motion_sensor[id] = digitalRead(*pin);

  // Commented out, depends on implementation of sensors
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
