#ifndef TEMP
#define TEMP

#include "lib.h"
#include <DHT.h>

auto tempClosure = [](uint dhtPin) {
  bool stable{};
  DHT dht(dhtPin, DHT22);  
  dht.begin();

  return [stable, dht](uint* pins, uint id) mutable{
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

    jsonOutput.dht22_humidity[id] = humidity;
    jsonOutput.dht22_temperature[id] = temperature;
  };
};

#endif
