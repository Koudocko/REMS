FROM debian:latest
WORKDIR /app
COPY . .
RUN apt-get update 
RUN apt-get install -y python3 python3-pip wget unzip
RUN pip install python-dotenv
RUN wget https://github.com/pimoroni/breakout-garden/archive/master.zip
RUN unzip master.zip
RUN ./breakout-garden-master/install.sh
RUN rm -r breakout-garden-master
CMD ["python3 weather.py"]
