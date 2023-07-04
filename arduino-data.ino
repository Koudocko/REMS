// 0 - equipment, 1 - laundry, 2 - kitchen, 3 - livingRoom

#include <OneWire.h>
#include <DHT.h>
#include <DallasTemperature.h>

using uint = unsigned int;

OneWire oneWire(23);
DallasTemperature sensors(&oneWire);
int deviceCount = 0;

struct Module{
  Module() = default;

  Module(unsigned long delay, void (*callback)(uint*), uint* pins) : 
    delay(delay), callback(callback), pins(pins){}

  void execute(){
    unsigned int curr = millis();
    this->last = curr - this->last >= delay ? (callback(), curr) : this->last;
  }

private:
  unsigned long delay, last{};
  uint* pins{};
  void (*callback)(uint*);
};

struct{
  float ds18b20_temperature[3]{};
  float dht22_humidity{};
  float dht22_temperature{};
  bool water_sensor_trigger{};
  bool motion_detected{};
  int soil_moisture{};

  String to_string(){
    String json = "{\n";
  
    json += "\t\"ds18b20_temperature\": [";
    for (int i = 0; i < 3; ++i){
      json += ds18b20_temperature[i];
      if (i < 2)
        json += ", ";
    }
    json += "],\n";

    json += String("\t\"dht22_humidity\": ") + dht22_humidity + ",\n";
    json += String("\t\"dht22_temperature\": ") + dht22_temperature + ",\n";
    json += String("\t\"water_sensor_trigger\": ") + water_sensor_trigger + ",\n";
    json += String("\t\"motion_detected\": ") + motion_detected + ",\n";
    json += String("\t\"soil_moisture\": ") + soil_moisture + ",\n";

    json += "}\n";

    return json;
  }
} json_output;

auto tempClosure = [](uint dhtPin) {
  bool stable{};
  DHT dht(dhtPin, DHT22);  

  return [stable, dht](uint* pins) mutable{
    float humidity = dht.readHumidity();
    float temperature = dht.computeHeatIndex(dht.readTemperature(), humidity, false);
    float target = 20.0f, range = 3.0f;

    if (abs(temperature - target) >= range){
      stable = false;
    }

    if (!stable){
      if (temperature - target < -0.25f){
        digitalWrite(pins[0], HIGH);
        digitalWrite(pins[1], LOW);
      }
      else if (temperature - target > 0.25f){
        digitalWrite(pins[0], LOW);
        digitalWrite(pins[1], HIGH);
      }
      else{
        digitalWrite(pins[0], HIGH);
        digitalWrite(pins[1], HIGH);
        stable = true;
      }
    }

    json_output.dht22_humidity = humidity;
    json_output.dht22_temperature = temperature;
  };
};

Module temps[4];
temps[0] = Module(3000, tempClosure(9), new uint{ 15, 14 });
temps[1] = Module(3000, tempClosure(9), new uint{ 15, 14 });
temps[2] = Module(3000, tempClosure(9), new uint{ 15, 14 });
temps[3] = Module(3000, tempClosure(9), new uint{ 15, 14 });

auto motionClosure = []{
  bool motion{};

  return [motion](uint* pin){
    json_output.motion_detected = motion;

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

Module motions[4];
motions[0] = Module(1000, motionClosure(), new uint{17});
motions[1] = Module(1000, motionClosure(), new uint{17});
motions[2] = Module(1000, motionClosure(), new uint{17});
motions[3] = Module(1000, motionClosure(), new uint{17});

Module ds18b20_timer = Module(5000, [](uint*){
  sensors.requestTemperatures(); 
  
  for (int i = 0; i < deviceCount;  i++)
  {
    float tempC = sensors.getTempCByIndex(i);
    if (tempC != -127)
      json_output.ds18b20_temperature[i] = tempC;
  }
}, nullptr);

auto vibClosure = [](uint* pins){
  bool trigger = digitalRead(pins[0]);
  digitalWrite(pins[1], !trigger);

  json_output.water_sensor_trigger = trigger;
};

Module vibs[4];
vibs[0] = Module(1000, vibClosure, new uint[]{ 27, 35 });
vibs[1] = Module(1000, vibClosure, new uint[]{ 27, 35 });
vibs[2] = Module(1000, vibClosure, new uint[]{ 27, 35 });
vibs[3] = Module(1000, vibClosure, new uint[]{ 27, 35 });

Module soil_timer = Module(3000, [](uint* pin){
  json_output.soil_moisture = analogRead(*pin);
}, new uint{ A14 });

void pinInit(){
  for (auto pins : vibs)
    pinMode(pins[1], OUTPUT);
  pinMode(HCSR_PIN, INPUT);
  pinMode(SW_PIN, INPUT);
  pinMode(FAN_PIN, OUTPUT);
  pinMode(LAMP_PIN_A, OUTPUT);
  pinMode(LAMP_PIN_B, OUTPUT);
  pinMode(WATER_PIN, OUTPUT);
  pinMode(SOIL_PIN, INPUT);
}

void setup(void)
{
  sensors.begin();  
  Serial.begin(9600);
  dht.begin();

  pinInit();

  while (!deviceCount){
    deviceCount = sensors.getDeviceCount();
    delay(1000);
  }
}

void loop(void){
  dht_timer.execute();
  ds18b20_timer.execute();
  hrc_timer.execute();
  sw420_timer.execute();
  soil_timer.execute();

  digitalWrite(16, HIGH);
  Serial.print(json_output.to_string());
  delay(10);
}
