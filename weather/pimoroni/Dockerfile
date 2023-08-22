FROM debian:latest
WORKDIR /app
COPY . .
RUN apt-get update 
RUN apt-get install -y python3 python3-pip wget unzip python3-dotenv python3-smbus python-is-python3
RUN wget https://github.com/pimoroni/breakout-garden/archive/master.zip
RUN unzip master.zip
RUN cd breakout-garden-master
RUN ./install.sh
RUN cd ..
RUN rm -r breakout-garden-master
CMD ["python3 weather.py"]
