# Overview of Residence 

## Directory Structure
The Residence directory is composed by three sub directories, **arduino**, **client**, and **serial**. Only Client and Serial contain systemd unit files for startup (see **Arduino** for info on why it is not).

### Arduino
* **arduino.ino**: Main Arduino C file
    * Initializes sensor modules
    * Sets module pinouts
    * Polls modules and prints JSON to serial stream
* **lib.ino|h**: Core structs 
    * Module struct and methods
    * JSON format struct and methods
* **motion|temp|vib.h**: Sensor specific implementations
    * Specialized callback functions per module instance
* **arduino-upload**: Arduino code upload script
    * Run arduino-upload to compile and upload the arduino code 
    * Restarts the Arduino serial scraper

### Client
* **Dockerfile**: Docker image build file
    * Image and container are named **residence-client**
    * Builds Rust project with Cargo
    * Runs finished executable
* **residence-client.service**: Systemd unit file
    * Unit is named **residence-client**
* **Cargo.toml**: Rust project specification
    * Specifies executable name as residence-client
    * Adds serde, serde_json, and dotenvy as dependencies
* **src/main.rs**: Main Rust file
    * Pulls server IP and residence id from .env file
    * Creates TCP connection to BBP and sends JSON data

### Serial
* **Dockerfile**: Docker image build file
    * Image and container are named **residence-serial**
    * Installs arduino-cli executable
    * Builds Rust project with Cargo
    * Installs APT dependencies
    * Updates and install Arduino board core
    * Runs finished executable
* **residence-serial.service**: Systemd unit file
    * Unit is named **residence-serial**
* **Cargo.toml**: Rust project specification
    * Specifies executable name as residence-serial
    * Adds serde and serde_json as dependencies
* **src/main.rs**: Main Rust file
    * Forks and executes arduino-monitor script with stdout pipe
    * Parsed runtime stdout for JSON and writes to data file
* **arduino-monitor**: Shell script to read serial port
    * Bundles arduino-cli monitor + flags into one command

## How it Works
A client node works in a very linear way. Firstly, the arduino script **residence-arduino** initializes several sensor modules, which is essentially a wrapper struct which provides access to pinout, polling times, and an interface via .execute() which handles non-blocking data retrieval. Each of these sensors write their values to a global instance of a struct holding sensor readings. At the end of the event loop, this struct instance is converted into a stringified JSON format which is sent across the serial connection. Secondly, the serial container **residence-serial** runs it's compiled Rust executable to scrape the serial device **/dev/ttyACM0** where the arduino is outputting the JSON. Once the serial output is parsed, it is written to the file at **/rems/readings/residence.json**. Lastly, the client container **residence-client** establishes a connection with BBP and sets its residence id. I then validates the JSON in that file by deserializing it into a rust-usable struct, subsequently sending it to BBP for processing. Specifics about the TCP requests format is talked about at **REMS/bbp/README.md**. The serial and client containers are constantly running and performing actions on updated data.

## How to Install/Deploy
Simply utilize the **residence_build.sh** script found in the root of the residence directory. Run with **./residence_build.sh** and simply enter your sudo password when prompted and answer any questions accordingly. Near the end you are prompted to enter two things. One, the residence id used to distinguish this machine's data on BBP. Two, the server IP in the format of the BBP machine. These environment variables will simply be output to a file at **/rems/files/client/.env**, so you can manualy change it in there if you need to. In the .env file, if you must, simply input your value after the equals sign **=** of the corresponding field. 
