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

DHT dht(DHT_PIN, DHT22);  
OneWire oneWire(ONE_WIRE_BUS);
DallasTemperature sensors(&oneWire);
int deviceCount = 0;

Timer dht_timer = Timer(3000, []{
  float humidity = dht.readHumidity();
  float temperature = dht.computeHeatIndex(dht.readTemperature(), humidity, false);
  static bool stable = false;
  float target = 20.0f, range = 3.0f;

  if (abs(temperature - target) >= range){
    Serial.println("NOT STABLE");
    stable = false;
  }

  if (!stable){
    if (temperature - target < -0.25f){
      Serial.println("WARM");
      digitalWrite(FAN_PIN, HIGH);
      digitalWrite(LAMP_PIN_A, LOW);
    }
    else if (temperature - target > 0.25f){
      Serial.println("COOL");
      digitalWrite(FAN_PIN, LOW);
      digitalWrite(LAMP_PIN_A, HIGH);
    }
    else{
      Serial.println("STABLE");
      digitalWrite(FAN_PIN, HIGH);
      digitalWrite(LAMP_PIN_A, HIGH);
      stable = true;
    }
  }

  Serial.print("DHT22 Sensor Humidity : ");
  Serial.println(humidity);
  Serial.print("DHT22 Sensor Temperature : ");
  Serial.println(temperature);
});

Timer hrc_timer = Timer(1000, []{
  static bool motion = false;

  if (digitalRead(HCSR_PIN)){
    if (!motion){
      motion = true;
      Serial.println("HRSR501 Sensor : Motion detected");
    }
  }
  else{
    if (motion){
      motion = false;
      Serial.println("HRSR501 Sensor : Motion ended");
    }
  }
});

Timer ds18b20_timer = Timer(5000, []{
  sensors.requestTemperatures(); 
  
  for (int i = 0; i < deviceCount;  i++)
  {
    Serial.print("DS18B20 Sensor ");
    Serial.print(i+1);
    Serial.print(" : ");
    float tempC = sensors.getTempCByIndex(i);
    Serial.print(tempC);
    Serial.print("C  |  ");
    Serial.print(DallasTemperature::toFahrenheit(tempC));
    Serial.println("F");
  }
});

Timer sw420_timer = Timer(1000, []{
  if (digitalRead(SW_PIN)){
    Serial.println("WATER ON");
    digitalWrite(LAMP_PIN_B, LOW);
  }
  else{
    Serial.println("WATER OFF");
    digitalWrite(LAMP_PIN_B, HIGH);
  }
});

Timer soil_timer = Timer(3000, []{
  Serial.print("Soil Moisture Value: ");
  Serial.println(analogRead(SOIL_PIN));
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
    Serial.print("Locating devices...");
    Serial.print("Found ");
    deviceCount = sensors.getDeviceCount();
    Serial.print(deviceCount, DEC);
    Serial.println(" devices.\n");
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
  delay(10);
}
