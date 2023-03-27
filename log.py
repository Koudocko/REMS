import json
import time
import stat
import os
import grp
from datetime import datetime
from mics6814 import MICS6814
import logging
from sgp30 import SGP30
import sys
import time
try:
    from smbus2 import SMBus
except ImportError:
    from smbus import SMBus
from bme280 import BME280

log_path = "/rems/temp/log.txt"

sgp30 = SGP30()
gas = MICS6814()
bme280 = BME280(i2c_dev=SMBus(1))

print("Sensor warming up, please wait...")
def crude_progress_bar():
    sys.stdout.write('.')
    sys.stdout.flush()

sgp30.start_measurement(crude_progress_bar)
sys.stdout.write('\n')

def sgp30_log():
    result = sgp30.get_air_quality()
    return "SGP30", { "C02": + result.equivalent_co2, "TVOC": result.total_voc }

def mics_log():
    result = gas.read_all()
    return "MICS6814", { "Oxidising": result.oxidising, "Reducing": result.reducing, "NH3": result.nh3, "ADC": result.adc }

def bme280_log():
    return "BME280", { "Temperature": bme280.get_temperature(), "Pressure": bme280.get_pressure(), "Humidity": bme280.get_humidity() }

sensors = [sgp30_log, mics_log, bme280_log]
log_data = { "data": {}, "time": datetime.now().strftime("%H:%M:%S") }

file_handle = open(log_path, "w")
file_handle.seek(0)

for sensor in sensors:
   res = sensor()
   log_data["data"][res[0]] = res[1]

file_handle.write(json.dumps(log_data))
file_handle.flush()
file_handle.truncate()
file_handle.close()

os.system("/rems/commands/rems_2_transfer_sh")
