import ftplib
import json
import time
import os
from datetime import datetime
from dotenv import load_dotenv
import sys
import time

# Return SGP30 sensor data in JSON
def sgp30_log():
    result = sgp30.get_air_quality()
    return "SGP30", { "C02": + result.equivalent_co2, "TVOC": result.total_voc }

# Return MICS6814 sensor data in JSON
def mics_log():
    result = gas.read_all()
    return "MICS6814", { "Oxidising": result.oxidising, "Reducing": result.reducing, "NH3": result.nh3, "ADC": result.adc }

# Return BMP280 sensor data in JSON
def bmp280_log():
    return "BMP280", { "Temperature": bmp280.get_temperature(), "Pressure": bmp280.get_pressure() }

sensors = [sgp30_log, mics_log, bmp280_log]

try:
    from smbus2 import SMBus
except ImportError:
    from smbus import SMBus
try:
    from bmp280 import BMP280
    bmp280 = BMP280(i2c_dev=SMBus(1))
except ImportError:
    sensors.remove(bmp280_log)
try:
    from mics6814 import MICS6814
    gas = MICS6814()
except ImportError:
    sensors.remove(mics_log)
try:
    from sgp30 import SGP30
    sgp30 = SGP30()

    print("Sensor warming up, please wait...")
    def crude_progress_bar():
        sys.stdout.write('.')
        sys.stdout.flush()
    sgp30.start_measurement(crude_progress_bar)
    sys.stdout.write('\n')
except ImportError:
    sensors.remove(sgp30_log)

log_path = "/rems/readings/weather.json"
log_data = { "data": {}, "time": datetime.now().strftime("%H:%M:%S") }

load_dotenv()
HOST = os.getenv('HOST')
USER = os.getenv('USER')
PASS = os.getenv('PASS')

# FTP code to transfer weather.json to webserver
def transfer_file(file):
    ftp = ftplib.FTP(str(HOST))
    ftp.login(str(USER), str(PASS))
    ftp.storlines(f'STOR {log_path}', file)
    ftp.quit()

while True:
    file_handle = open(log_path, "w")

    # Add sensor data to JSON object
    file_handle.seek(0)
    for sensor in sensors:
       res = sensor()

       log_data["data"][res[0]] = res[1]

    # Write out JSON to file 
    file_handle.write(json.dumps(log_data))
    file_handle.flush()

    # Trasfer file to webserver
    file_handle.truncate()
    transfer_file(file_handle)

    file_handle.close()
    time.sleep(5)
