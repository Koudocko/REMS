# Overview of Residence 

## Directory Structure
The Residence directory is composed by three sub directories, **arduino**, **client**, and **serial**. Only Client and Serial contain systemd unit files for startup (see **Arduino** for info on why it is not).

### Arduino
In here is all the build files written in Arduino C to be compiled and uploaded to the Arduino board. There also exists a shell script to compile and upload the Arduino C code with ease, just run **arduino-monitor**.
* **weather.py**: Python file
    * Reads data from sensors RPi's GPIO
    * Formats data into a JSON package
    * Transfers JSON package to webserver


### Client
The client directory contains a rust project with all the **.rs** files, alongside its own Dockerfile to build these files.

### Serial
The serial directory contains another Dockerfile, allowing it to run concurrently with the Arduino container and scrape its serial output for processing. This is a rust project utilizing the arduino-cli monitor output to scrape and transfer to a file.

## How it Works
A client node works in a very linear way. Firstly, the arduino script **residence-arduino** initializes several sensor modules, which is essentially a wrapper struct which provides access to pinout, polling times, and an interface via .execute() which handles non-blocking data retrieval. Each of these sensors write their values to a global instance of a struct holding sensor readings. At the end of the event loop, this struct instance is converted into a stringified JSON format which is sent across the serial connection. Secondly, the serial container **residence-serial** runs it's compiled Rust executable to scrape the serial device **/dev/ttyACM0** where the arduino is outputting the JSON. Once the serial output is parsed, it is written to the file at **/rems/readings/residence.json**. Lastly, the client container **residence-client** establishes a connection with BBP and sets its residence id. I then validates the JSON in that file by deserializing it into a rust-usable struct, subsequently sending it to BBP for processing. Specifics about the TCP requests format is talked about at **REMS/bbp/README.md**. The serial and client containers are constantly running and performing actions on updated data.

## How to Install/Deploy
Simply utilize the **residence_build.sh** script found in the root of the residence directory. Run with **./residence_build.sh** and simply enter your sudo password when prompted and answer any questions accordingly. Near the end you are prompted to enter two things. One, the residence id used to distinguish this machine's data on BBP. Two, the server IP in the format of the BBP machine. These environment variables will simply be output to a file at **/rems/files/client/.env**, so you can manualy change it in there if you need to. In the .env file, if you must, simply input your value after the equals sign **=** of the corresponding field. 
