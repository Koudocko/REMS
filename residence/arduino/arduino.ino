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

// DHT22 sensors
TempModule temps[4]{
  TempModule(3000, tempClosure, new uint[3]{ 15, 14, 22 }, 0),
  TempModule(3000, tempClosure, new uint[3]{ 15, 14, 24 }, 1),
  TempModule(3000, tempClosure, new uint[3]{ 15, 14, 26 }, 2),
  TempModule(3000, tempClosure, new uint[3]{ 15, 14, 28 }, 3)
};
// HCSR-505 sensors
MotionModule motions[4]{
  MotionModule(1000, motionClosure, new uint{30}, 0),
  MotionModule(1000, motionClosure, new uint{32}, 1),
  MotionModule(1000, motionClosure, new uint{34}, 2),
  MotionModule(1000, motionClosure, new uint{36}, 3)
};
// DAOKI sound sensors 
Module vibs[4]{
  Module(10, vibClosure, new uint[2]{ 23, 35 }, 0),
  Module(10, vibClosure, new uint[2]{ 25, 35 }, 1),
  Module(10, vibClosure, new uint[2]{ 27, 35 }, 2),
  Module(10, vibClosure, new uint[2]{ 29, 35 }, 3)
};
                                    
// Global instance of JsonFormat to store all sensor readings
JsonFormat jsonOutput;

// DS18B20 sensors (commented out until needed)
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

// Initialize every pin used as OUTPUT or INPUT
void pinInit(){
  // Was relay pins, subject to change
  pinMode(35, OUTPUT);
  pinMode(33, OUTPUT);
  pinMode(37, OUTPUT);
  pinMode(31, OUTPUT);
  
  // Intialize DAOKI sound sensor pins 
  for (auto mod : vibs){
    pinMode(mod.pins[0], INPUT);
    pinMode(mod.pins[1], OUTPUT); 
  }

  // Intialize DHT22 pins 
  for (auto mod : temps){
    pinMode(mod.pins[0], OUTPUT);
    pinMode(mod.pins[1], OUTPUT);
    pinMode(mod.pins[2], INPUT);
  }

  // Initialze HCSR-505 pins
  for (auto mod : motions){
    pinMode(mod.pins[0], INPUT);
  }

  pinMode(16, OUTPUT);
}


// Program entry point, stage 1, passes to control to void loop(void) when done
void setup(void)
{
  Serial.begin(9600);
  pinInit();

  // Initialize DS18B20 sensors
  //sensors.begin();  
  /*while (!deviceCount){
    deviceCount = sensors.getDeviceCount();
    delay(1000);
  }*/
}

// Main driving loop
void loop(void){
  // Every execute statement retrieves data (if ready) and updates jsonOutput
  for (auto mod : vibs)
    mod.execute();

  for (auto mod : temps)
    mod.execute();
    
  for (auto mod : motions)
    mod.execute();

  // Send jsonOutput across the serial port
  Serial.print(jsonOutput.to_string());

  delay(10);
}
