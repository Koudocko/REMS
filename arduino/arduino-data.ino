// 0 - equipment, 1 - laundry, 2 - kitchen, 3 - livingRoom

#include <OneWire.h>
#include <DallasTemperature.h>
#include "lib.h"
#include "temp.h"
#include "motion.h"
#include "vib.h"

OneWire oneWire(23);
DallasTemperature sensors(&oneWire);
int deviceCount = 0;

Module temps[4], vibs[4], motions[4];
JsonFormat jsonOutput;

Module ds18b20_timer = Module(5000, [](uint*){
  sensors.requestTemperatures(); 
  
  for (int i = 0; i < deviceCount;  i++)
  {
    float tempC = sensors.getTempCByIndex(i);
    if (tempC != -127)
      jsonOutput.ds18b20_temperature[i] = tempC;
  }
}, nullptr, 0);

Module soil_timer = Module(3000, [](uint* pin){
  jsonOutput.soil_moisture = analogRead(*pin);
}, new uint{ A14 }, 0);

void modInit(){
  temps[0] = Module(3000, tempClosure(28), new uint[2]{ 15, 14 }, 0);
  temps[1] = Module(3000, tempClosure(9), new uint[2]{ 15, 14 }, 1);
  temps[2] = Module(3000, tempClosure(9), new uint[2]{ 15, 14 }, 2);
  temps[3] = Module(3000, tempClosure(9), new uint[2]{ 15, 14 }, 3);

  motions[0] = Module(1000, motionClosure(), new uint{36}, 0);
  motions[1] = Module(1000, motionClosure(), new uint{17}, 1);
  motions[2] = Module(1000, motionClosure(), new uint{17}, 2);
  motions[3] = Module(1000, motionClosure(), new uint{17}, 3);
  
  vibs[0] = Module(1000, vibClosure, new uint[2]{ 36, 35 }, 0);
  vibs[1] = Module(1000, vibClosure, new uint[2]{ 27, 35 }, 1);
  vibs[2] = Module(1000, vibClosure, new uint[2]{ 27, 35 }, 2);
  vibs[3] = Module(1000, vibClosure, new uint[2]{ 27, 35 }, 3);
}

void pinInit(){
  for (auto mod : vibs)
    pinMode(mod.getPin(1), OUTPUT);

  for (auto mod : temps){
    pinMode(mod.getPin(0), OUTPUT);
    pinMode(mod.getPin(1), OUTPUT);
  }

  pinMode(16, OUTPUT);
}

void setup(void)
{
  sensors.begin();  
  Serial.begin(9600);

  modInit();
  pinInit();

  while (!deviceCount){
    deviceCount = sensors.getDeviceCount();
    delay(1000);
  }
}

void loop(void){
  for (auto mod : vibs)
    mod.execute();

  for (auto mod : temps)
    mod.execute();

  for (auto mod : motions)
    mod.execute();

  digitalWrite(16, HIGH);
  Serial.print(jsonOutput.to_string());
  delay(10);
}
