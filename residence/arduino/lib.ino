#include "lib.h"

Module::Module(unsigned long delay, void (*callback)(uint*, uint), uint* pins, uint id) :
  delay(delay), callback(callback), pins(pins), id(id){}

Module::Module(unsigned long delay, uint* pins, uint id) :
  delay(delay), pins(pins), id(id){}

void Module::execute(){
  unsigned int curr = millis();
  last = curr - last >= delay ? (handler(), curr) : last;
}

void Module::handler(){
  callback(pins, id);
}

bool bool2str(bool input){
  if (input)
    return String("true");
  return String("false");
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
    if (i < 3)
      json += ", ";
  }
  json += "],\n";

  json += "\t\"dht22_temperature\": [";
  for (int i = 0; i < 4; ++i){
    json += dht22_temperature[i];
    if (i < 3)
      json += ", ";
  }
  json += "],\n";

  json += "\t\"motion_detected\": [";
  for (int i = 0; i < 4; ++i){
    json += bool2str(motion_sensor[i]);
    if (i < 3)
      json += ", ";
  }
  json += "],\n";

  json += "\t\"water_sensor_trigger\": [";
  for (int i = 0; i < 4; ++i){
    json += bool2str(sound_sensor[i]);
    if (i < 3)
      json += ", ";
  }
  json += "],\n";

  json += String("\t\"soil_moisture\": ") + soil_moisture + ",\n";

  json += "}\n";

  return json;
}
