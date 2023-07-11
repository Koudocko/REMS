// 0 - equipment, 1 - laundry, 2 - kitchen, 3 - livingRoom
/*
Pin D22 - DHT22 Eq
Pin D24 - DHT22 Laundry
Pin D26 - DHT22 Kitchen
Pin D28 - DHT22 Living
Pin D23 - DAOKI Sound Sensor Eq
Pin D25 - DAOKI Sound Sensor Laundry
Pin D27 - DAOKI Sound Sensor Kitchen
Pin D29 - DAOKI Sound Sensor Living
Pin D30 - HCSR-505 Eq
Pin D32 - HCSR-505 Laundry
Pin D34 - HCSR-505 Kitchen
Pin D36 - HCSR-505 Living
*/

#include <OneWire.h>
#include <DallasTemperature.h>
#include "lib.h"
#include "temp.h"
#include "motion.h"
#include "vib.h"

OneWire oneWire(23);
DallasTemperature sensors(&oneWire);
int deviceCount = 0;

TempModule temps[4]{
  TempModule(3000, tempClosure, new uint[3]{ 15, 14, 22 }, 0),
  TempModule(3000, tempClosure, new uint[3]{ 15, 14, 24 }, 1),
  TempModule(3000, tempClosure, new uint[3]{ 15, 14, 26 }, 2),
  TempModule(3000, tempClosure, new uint[3]{ 15, 14, 28 }, 3)
};
MotionModule motions[4]{
  MotionModule(1000, motionClosure, new uint{30}, 0),
  MotionModule(1000, motionClosure, new uint{32}, 0),
  MotionModule(1000, motionClosure, new uint{34}, 0),
  MotionModule(1000, motionClosure, new uint{36}, 0)
};
Module vibs[4]{
  Module(1000, vibClosure, new uint[2]{ 23, 35 }, 0),
  Module(1000, vibClosure, new uint[2]{ 25, 35 }, 0),
  Module(1000, vibClosure, new uint[2]{ 27, 35 }, 0),
  Module(1000, vibClosure, new uint[2]{ 29, 35 }, 0)
};

JsonFormat jsonOutput;

/*Module ds18b20_timer = Module(5000, [](uint*){
  sensors.requestTemperatures(); 
  
  for (int i = 0; i < deviceCount;  i++)
  {
    float tempC = sensors.getTempCByIndex(i);
    if (tempC != -127)
      jsonOutput.ds18b20_temperature[i] = tempC;
  }
}, nullptr, 0);*/

/*Module soil_timer = Module(3000, [](uint* pin){
  jsonOutput.soil_moisture = analogRead(*pin);
}, new uint{ A14 }, 0);*/

void pinInit(){
  for (auto mod : vibs)
    pinMode(mod.pins[1], OUTPUT);

  for (auto mod : temps){
    pinMode(mod.pins[0], OUTPUT);
    pinMode(mod.pins[1], OUTPUT);
  }

  pinMode(16, OUTPUT);
}

DHT dht(28, DHT22);

void setup(void)
{
  //sensors.begin();  
  Serial.begin(9600);

  pinInit();
  dht.begin();

  /*while (!deviceCount){
    deviceCount = sensors.getDeviceCount();
    delay(1000);
  }*/
}

void loop(void){
  for (auto mod : vibs)
    mod.execute();

  for (auto mod : temps)
    mod.execute();

  for (auto mod : motions)
    mod.execute();

  digitalWrite(31, HIGH);
  Serial.print(jsonOutput.to_string());

  delay(10);
}
