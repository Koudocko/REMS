#ifndef LIB
#define LIB

using uint = unsigned int;

struct Module{
  Module() = default;

  Module(unsigned long delay, void (*callback)(uint*, uint), uint* pins, uint id);

  void execute();

  uint* pins{}, id{};
  unsigned long delay, last{};
  void (*callback)(uint*, uint);
};

struct JsonFormat{
  float ds18b20_temperature[3]{};
  float dht22_humidity[4]{};
  float dht22_temperature[4]{};
  bool water_sensor_trigger[4]{};
  bool motion_detected[4]{};
  int soil_moisture{};

	String to_string();
};

extern JsonFormat jsonOutput;

#endif
