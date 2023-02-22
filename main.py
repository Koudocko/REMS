import stat
import os
import grp
import board
from datetime import datetime

log_path = "/web-server/sensor_logs"

sensor_name = "Heat Sensor A53422B34F"
sensor_data = "029.69C/085.44F"
data_time = datetime.now().strftime("%H:%M:%S")

file_handle = open(log_path, "a")
os.chown(log_path, 0, grp.getgrnam("web-server").gr_gid)
os.chmod(log_path, stat.S_IRWXG)
file_handle.write("MODULE: " + sensor_name + "- READ: " + sensor_data + " - TIME: " + data_time + "\n")

file_handle.close()
