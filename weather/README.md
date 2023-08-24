# Overview of Weather Station 

## Directory Structure
The Residence directory is composed by two sub directories, **pimoroni** and **webserver**.

### Pimoroni
* **Dockerfile**: Docker image build file
    * Image and container are named **weather-pimoroni**
    * Installs Python APT dependencies
    * Creates virtual Python environment
    * Installs Python PIP dependencies
    * Runs Python script with virtual Python environment
* **weather-pimoroni.service**: Systemd unit file
    * Unit is named **weather-pimoroni**
* **weather.py**: Python file
    * Reads data from sensors RPi's GPIO 
    * Formats data into a JSON package
    * Transfers JSON package to webserver

### Webserver
The webserver contains a complete Django project, bundled in with static CSS/JS files and the HTML template. A Dockerfile is also included to containerize it.
* **Dockerfile**: Docker image build file
    * Image and container are named **weather-webserver**
    * Install Python PIP dependencies
    * Exposes server port
    * Activates Django server with Python script 
* **weather-webserver.service**: Systemd unit file
    * Unit is named **weather-pimoroni**
* **webserver/views.py**: Python web response functions
    * Displays entire webview render
    * Returns JSON from readings file 
* **webserver/urls.py**: URL endpoints
    * Sets root webview and JSON get endpoints
* **webserver/static**: Static elements folder
    * Website CSS folder
    * Website JS folder
* **webserver/templates**: Template elements folder
    * Website HTML folder
* **webserver/settings.py**: Django settings script
    * Manage entire Django behaviour

## How it Works
There are only two components of the weather station working together. These can either be deployed on the same server to allow for both GPIO interfacing and webserver hosting on the same machine, or seperately. Essentially, the Pimoroni container **weather-pimoroni** is a python script which scrapes data from a few sensors on the RPi's GPIO. It then takes this data collected, stores it at **/rems/readings/weather.json**, and utilizes FTP to send it to the webserver at the same directory. The webserver container **weather-webserver** which runs the Django framework runs on the local machine's IP at port 8080. Every 5 seconds, the backend grabs updated data from the **weather.json** file and displays it. The interaction between these two applications is pretty simple. 

## How to Install/Deploy
Simply utilize the **weather_build.sh** script found in the root of the weather directory. Run with **./weather_build.sh** and simply enter your sudo password when prompted and answer any questions accordingly. At some point, you are prompted with the option to install the Pimoroni project, the Webserver project, or both on the machine. You must specify with the corresponding number provided. This freedom is given due to the current state of the project being very fluid, thus you can edit where these two containers interact. On the Pimoroni side, you are prompted to enter a host IP (the IP of the webserver), the username of the user on the host, and the password of that user. These can be edited later at **/rems/files/pimoroni/.env** by pasting the value after the corresponding key. If you install both pimoroni and webserver on the same machine, the reference host IP will be the machine's local IP on the network. 
