import time
from adafruit_bme280 import basic as adafruit_bme280
import stat
import os
import grp
import board
from datetime import datetime

log_path = "sensor_logs"

def bme280_log():
   bme280 = adafruit_bme280.Adafruit_BME280_I2C(board.I2C())
   return "BME280", "temp: %0.1f" % bme280.temperature + ", humidity: %0.1f %%" % bme280.relative_humidity

def refresh_log():
   data = open(log_path, "r").readlines()
   file_handle = open(log_path, "w")
   count = 0

   while count < len(data) and (datetime.now() - datetime.strptime(datetime.now().strftime("%Y-%m-%d ") + data[count].rsplit("TIME:", 1)[1].strip(), "%Y-%m-%d %H:%M:%S")).total_seconds() > 300:
      count += 1
   file_handle.writelines(data[count::])
   file_handle.close()

# log_path = "/web-server/sensor_logs"
sensors = [bme280_log]

file_handle = open(log_path, "a")
# os.chown(log_path, 0, grp.getgrnam("web-server").gr_gid)
# os.chmod(log_path, stat.S_IRWXG)

while True:
   for sensor in sensors:
      refresh_log()
      res = sensor()
      data_time = datetime.now().strftime("%H:%M:%S")

      file_handle.seek(0, 2)
      file_handle.write("MODULE: " + res[0] + "- READ: " + res[1] + " - TIME: " + data_time + "\n")
      file_handle.flush()
   time.sleep(10)

file_handle.close()
