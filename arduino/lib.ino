#include "lib.h"

Module::Module(unsigned long delay, void (*callback)(uint*, uint), uint* pins, uint id) :
  delay(delay), callback(callback), pins(pins){}

Module::Module(unsigned long delay, uint* pins, uint id) :
  delay(delay), pins(pins){}

void Module::execute(){
  unsigned int curr = millis();
  last = curr - last >= delay ? (callback(pins, id), curr) : last;
}

virtual void Module::handler(){
  callback(pins, id);
}

void Module::getPin(int idx){
  return pins[idx];
}

void Module::getId(){
  return id;
}

String JsonFormat::to_string(){
  String json = "{\n";

  json += "\t\"ds18b20_temperature\": [";
  for (int i = 0; i < 3; ++i){
    json += ds18b20_temperature[i];
    if (i < 2)
      json += ", ";
  }
  json += "],\n";

  json += "\t\"dht22_humidity\": [";
  for (int i = 0; i < 4; ++i){
    json += dht22_humidity[i];
    if (i < 2)
      json += ", ";
  }
  json += "],\n";

  json += "\t\"dht22_temperature\": [";
  for (int i = 0; i < 4; ++i){
    json += dht22_temperature[i];
    if (i < 2)
      json += ", ";
  }
  json += "],\n";

  json += "\t\"motion_detected\": [";
  for (int i = 0; i < 4; ++i){
    json += motion_detected[i];
    if (i < 2)
      json += ", ";
  }
  json += "],\n";

  json += "\t\"water_sensor_trigger\": [";
  for (int i = 0; i < 4; ++i){
    json += water_sensor_trigger[i];
    if (i < 2)
      json += ", ";
  }
  json += "],\n";

  json += String("\t\"soil_moisture\": ") + soil_moisture + ",\n";

  json += "}\n";

  return json;
}
