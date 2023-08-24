# Overview of REMS 

## About
REMS is a system of software ever expanding in scope. Currently, it exists as a network between several residence client machines connected to one server, BBP (Big Brother Prometheus). It also includes a weather station which can be deployed on the local network and accessed through a web server. These components are split into three directories, each with their own source files/code, docker containers, systemd unit files, an install script, and additional documentation. 

## How to Use
### Install
For each of the three folders exists an install script named &lt;folder-name&gt;_build.sh.  
You must first **cd** into the chosen directory and then simply execute the shell script with **./&lt;folder-name&gt;_build.sh**.  
For the most part, the install is fully automated except for a few required pieces of information.  

### Monitor/Maintain
For each of the three folders exists a **README.md** like the current one you are reading.  
Under the **Directory Structure** header, each sub directory lists important files.  
Docker-related folders will list their Docker container/image names.  
If there is any mention of being Dockerized, then the following commands may be helpful:  
* **docker ps**: lists running Docker containers
* **docker ps -a**: lists installed Docker containers
* **docker image ls**: lists install Docker images
* **docker stop &lt;container-name&gt;**: stops running Docker container
* **docker start &lt;container-name&gt;**: start inactive Docker container
* **docker restart &lt;container-name&gt;**: restart running Docker container
* **docker attach &lt;container-name&gt;**: attach to running Docker container
* **docker rm &lt;container-name&gt;**: remove Docker container
* **docker rmi &lt;image-name&gt;**: remove Docker image

While you could manage the underlying Docker containers like this, for general use I reccomend using systemd.  
Systemd-related folders will list their service names.  
If there is any mention of systemd or service files, then the following commands may be helpful:  
* **systemctl status &lt;systemd-unit&gt;**: check if unit is active/enabled
* **systemctl enable &lt;systemd-unit&gt;**: enable unit autostart on boot
* **systemctl disable &lt;systemd-unit&gt;**: disable unit autostart on boot
* **systemcctl start &lt;systemd-unit&gt;**: startup unit
* **systemcctl stop &lt;systemd-unit&gt;**: shutdown unit
* **systemcctl restart &lt;systemd-unit&gt;**: restart unit

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
    * Wrote a serial parser in Rust for the Arduino's communication (uses arduino-cli now, not bare C)
* Weather Station
    * Wrote Python code to collect data from RPi GPIO sensors
    * Setup and wrote Django backend for web server
    * Created the web server's frontend using HTML, CSS, and JS 
* Wrote build scripts to dockerize and install every component with standardization of file system structure
* Documented all above components with a standard (both inline code and via writeup)

#### Note
I have some suggestions for future developers (things I did not get time to do).  
Using the Grafana API, you can setup alerts for the sensor array via HTTP requests; the same way I automatically setup dashboards and folders.  
Every client residence has an open TCP stream with the BBP server, thus the server can send instructions to the client to be read and acted on.  
Communication with the Arduino is not one-ended, you can setup a script/program to rewrite the source code based on e.g. swapping out special variables for runtime data then recompiling/uploading with the arduino-upload command.  

Contact me for any critical maintenance, bug fixes, and/or questions; as my contribution was still early in the experimental phase of development.
