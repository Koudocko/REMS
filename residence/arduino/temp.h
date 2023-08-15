#ifndef TEMP
#define TEMP

#include "lib.h"
#include <DHT.h>

struct TempModule : Module{
  TempModule() = default;

  TempModule(unsigned long delay, void (*callback)(uint*, uint, DHT&, bool), uint* pins, uint id)
    : Module(delay, pins, id), callback(callback), dht(pins[2], DHT22){
    dht.begin();
  }

  void handler(){
    callback(pins, id, dht, stable);
  }

private:
  DHT dht;
  bool stable{};
  void (*callback)(uint*, uint, DHT&, bool);
};

auto tempClosure = [](uint* pins, uint id, DHT& dht, bool stable){
  float humidity = dht.readHumidity();
  float temperature = dht.computeHeatIndex(dht.readTemperature(), humidity, false);
  float target = 20.0f, range = 3.0f;

  if (abs(temperature - target) >= range){
    stable = false;
  }

  if (!stable){
    if (temperature - target < -0.25f){
      /* digitalWrite(pins[0], HIGH); */
      /* digitalWrite(pins[1], LOW); */
    }
    else if (temperature - target > 0.25f){
      /* digitalWrite(pins[0], LOW); */
      /* digitalWrite(pins[1], HIGH); */
    }
    else{
      /* digitalWrite(pins[0], HIGH); */
      /* digitalWrite(pins[1], HIGH); */
      stable = true;
    }
  }

  if (!isnan(humidity))
    jsonOutput.dht22_humidity[id] = humidity;
  if (!isnan(temperature))
    jsonOutput.dht22_temperature[id] = temperature;
};

#endif
