# Overview of Residence 

## Directory Structure
The Residence directory is composed by three sub directories, **arduino**, **client**, and **serial**.

### Arduino
In here is all the build files written in Arduino C to be compiled and uploaded to the Arduino board. There also exists a Dockerfile, purely for organization/isolation to startup the board with the executable.

### Client
The client directory contains a rust project with all the **.rs** files, alongside its own Dockerfile to build these files.

### Serial
The serial directory contains another Dockerfile, allowing it to run concurrently with the Arduino container and scrape its serial output for processing. This is a single C file, low level to customize the file descriptor to the serial device.

## How it Works
A client node works in a very linear way. Firstly, the arduino container **residence-arduino** initializes several sensor modules, which is essentially a wrapper struct which provides access to pinout, polling times, and an interface via .execute() which handles non-blocking data retrieval. Each of these sensors write their values to a global instance of a struct holding sensor readings. At the end of the event loop, this struct instance is converted into a stringified JSON format which is sent across the serial connection. Secondly, the serial container **residence-serial** runs it's compiled C executable to scrape the serial device **/dev/ttyACM0** where the arduino is outputting the JSON. C is used because it provides very low level control over how devices can be interfaced, which is vital to accurately read from the serial device. Once the serial output is parsed, it is written to the file at **/rems/readings/residence.json**. Lastly, the client container **residence-client** establishes a connection with BBP and sets its residence id. I then validates the JSON in that file by deserializing it into a rust-usable struct, subsequently sending it to BBP for processing. Specifics about the TCP requests format is talked about at **REMS/bbp/README.md**. The serial and client containers are constantly running and performing actions on updated data. The Arduino container however does not stay loaded and instead just uploads the binary then exits. It exists as a container to seperate the binary from the rest of the system (due to how arduino-cli handles this) and for organization. 

## How to Install/Deploy
Simply utilize the **residence_build.sh** script found in the root of the residence directory. Run with **./residence_build.sh** and simply enter your sudo password when prompted and answer any questions accordingly. Near the end you are prompted to enter two things. One, the residence id used to distinguish this machine's data on BBP. Two, the server socket in the format of **IP:PORT** of the BBP machine. These environment variables will simply be output to a file at residence/client/.env, so you can manuall change it in there if you need to. In the .env file, if you must, simply input your value after the equals sign **=** of the corresponding field. 
