FROM debian:latest
WORKDIR /app
COPY . .
RUN apt-get update 
RUN apt-get install -y python3 python3-pip wget unzip python3-dotenv python3-smbus python-is-python3 
RUN wget https://github.com/pimoroni/breakout-garden/archive/master.zip && \
    unzip master.zip && \
    cd breakout-garden-master && \
    python3 autodetect.py --install && \
    cd .. && \
    rm -r breakout-garden-master
CMD ["python3", "weather.py"]
