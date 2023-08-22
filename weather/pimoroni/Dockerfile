FROM debian:latest
WORKDIR /app
COPY . .
RUN apt-get update 
RUN apt-get install -y python3 python3-pip wget unzip python3-dotenv python3-smbus python-is-python3 python3-venv
RUN python3 -m venv venv
RUN . venv/bin/activate && \
    pip install smbus2 pimoroni-mics6814 pimoroni-sgp30 bmp280
CMD ["venv/bin/python3", "weather.py"]
