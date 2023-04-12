#include <OneWire.h>
#include <DHT.h>
#include <DallasTemperature.h>

#define ONE_WIRE_BUS 23
#define DHTPIN 29
#define HCSRPIN 17
#define FANPIN 15
#define LEDPIN 14

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

DHT dht(DHTPIN, DHT22);  
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
      digitalWrite(FANPIN, HIGH);
      digitalWrite(LEDPIN, LOW);
    }
    else if (temperature - target > 0.25f){
      Serial.println("COOL");
      digitalWrite(FANPIN, LOW);
      digitalWrite(LEDPIN, HIGH);
    }
    else{
      Serial.println("STABLE");
      digitalWrite(FANPIN, HIGH);
      digitalWrite(LEDPIN, HIGH);
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

  if (digitalRead(HCSRPIN)){
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

Timer ds18b20_timer = Timer(1800000, []{
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

void setup(void)
{
  sensors.begin();  
  Serial.begin(9600);
  dht.begin();
  pinMode(HCSRPIN, INPUT);
  pinMode(FANPIN, OUTPUT);
  pinMode(LEDPIN, OUTPUT);

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
  //ds18b20_timer.execute();
  //hrc_timer.execute();

  delay(10);
}
