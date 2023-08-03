#ifndef LIB
#define LIB

using uint = unsigned int;

struct Module{
  Module() = default;
  Module(unsigned long delay, void (*callback)(uint*, uint), uint* pins, uint id);
  Module(unsigned long delay, uint* pins, uint id);

  void execute();
  virtual void handler();

  uint* pins{}, id{};

private:
  unsigned long delay, last{};
  void (*callback)(uint*, uint);
};

struct JsonFormat{
  float ds18b20_temperature[3]{};
  float dht22_humidity[4]{};
  float dht22_temperature[4]{};
  bool sound_sensor[4]{};
  bool motion_sensor[4]{};
  int soil_moisture{};

	String to_string();
};

extern JsonFormat jsonOutput;

#endif
