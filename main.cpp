#include <OneWire.h>
#include <DHT.h>
#include <DallasTemperature.h>

#define ONE_WIRE_BUS 23
#define DHT_PIN 29
#define HCSR_PIN 17
#define FAN_PIN 15
#define LAMP_PIN_A 14
#define LAMP_PIN_B 35
#define SW_PIN 27
#define WATER_PIN 16
#define SOIL_PIN A14

DHT dht(DHT_PIN, DHT22);  
OneWire oneWire(ONE_WIRE_BUS);
DallasTemperature sensors(&oneWire);
int deviceCount = 0;

struct Timer{
  Timer(unsigned long delay, void (*callback)()) : delay(delay), callback(callback){}

  void execute(){
    unsigned int curr = millis();
    this->last = curr - this->last >= delay ? (callback(), curr) : this->last;
  }

private:
  unsigned long delay, last{};
  void (*callback)();
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


Timer dht_timer = Timer(3000, []{
  float humidity = dht.readHumidity();
  float temperature = dht.computeHeatIndex(dht.readTemperature(), humidity, false);
  static bool stable = false;
  float target = 20.0f, range = 3.0f;

  if (abs(temperature - target) >= range){
    stable = false;
  }

  if (!stable){
    if (temperature - target < -0.25f){
      digitalWrite(FAN_PIN, HIGH);
      digitalWrite(LAMP_PIN_A, LOW);
    }
    else if (temperature - target > 0.25f){
      digitalWrite(FAN_PIN, LOW);
      digitalWrite(LAMP_PIN_A, HIGH);
    }
    else{
      digitalWrite(FAN_PIN, HIGH);
      digitalWrite(LAMP_PIN_A, HIGH);
      stable = true;
    }
  }

  json_output.dht22_humidity = humidity;
  json_output.dht22_temperature = temperature;
});

Timer hrc_timer = Timer(1000, []{
  static bool motion = false;
  json_output.motion_detected = motion;

  if (digitalRead(HCSR_PIN)){
    if (!motion){
      motion = true;
    }
  }
  else{
    if (motion){
      motion = false;
    }
  }
});

Timer ds18b20_timer = Timer(5000, []{
  sensors.requestTemperatures(); 
  
  for (int i = 0; i < deviceCount;  i++)
  {
    float tempC = sensors.getTempCByIndex(i);
    if (tempC != -127){
      json_output.ds18b20_temperature[i] = tempC;
    }
  }
});

Timer sw420_timer = Timer(1000, []{
  bool trigger = digitalRead(SW_PIN);
  digitalWrite(LAMP_PIN_B, !trigger);
  json_output.water_sensor_trigger = trigger;
});

Timer soil_timer = Timer(3000, []{
  json_output.soil_moisture = analogRead(SOIL_PIN);
});


void setup(void)
{
  sensors.begin();  
  Serial.begin(9600);
  dht.begin();
  pinMode(HCSR_PIN, INPUT);
  pinMode(SW_PIN, INPUT);
  pinMode(FAN_PIN, OUTPUT);
  pinMode(LAMP_PIN_A, OUTPUT);
  pinMode(LAMP_PIN_B, OUTPUT);
  pinMode(WATER_PIN, OUTPUT);
  pinMode(SOIL_PIN, INPUT);

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

  digitalWrite(WATER_PIN, HIGH);
  Serial.print(json_output.to_string());
  delay(10);
}
