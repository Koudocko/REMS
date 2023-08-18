FROM debian:latest
WORKDIR /app
COPY . .
RUN gcc serial.c -o serial
CMD ["./serial"]
