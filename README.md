# Overview of REMS 

## About
REMS is a system of software ever expanding in scope. Currently, it exists as a network between several residence client machines connected to one server, BBP (Big Brother Prometheus). It also includes a weather station which can be deployed on the local network and accessed through a web server. These components are split into three directories, each with their own source files/code, an install script, and additional documentation.

## Contributors (Add your information below, following markdown format)
### Tyler Wehrle
#### Connections
**Email:** tyler@pcquest.ca  
**Github:** https://github.com/Koudocko  
**LinkedIn:** https://www.linkedin.com/in/tyler-wehrle-308aa4250  

#### Contributions
* BBP (Big Brother Prometheus)
    * Wrote the framework for the rust TCP server / core data handler
    * Manages asynchronous connections to residences 
    * Write metrics to Prometheus database
    * Auto-intializes Grafana dashboard templates
* Residence Client Node
    * Wrote the rust client end of BBP's TCP stream, providing a trivial library
    * Created a system for developing Arduino modules in Arduino C
    * Setup sensor code for DHT22, DAOKI sound sensor, HCSR-505, DS18B20, moisture sensor
    * Wrote a serial parser in C for the Arduino's communication
* Weather Station
    * Wrote Python code to collect data from RPi GPIO sensors
    * Setup and wrote Django backend for web server
    * Created the web server's frontend using HTML, CSS, and JS 
* Wrote build scripts to dockerize and install every component with standardization of file system structure
* Documented all above components with a standard (both inline code and via writeup)

#### Note
Contact me for any maintenance and/or bug fixes as my contribution was still early in the experimental phase of development.
