FROM bullseye-slim:latest
WORKDIR /app
COPY . .
RUN gcc serial.c -o serial
CMD ["./serial"]
