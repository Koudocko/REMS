# Overview of BBP (Big Brother Prometheus) 

## Directory Structure
The BBP directory is composed by three sub directories, **grafana**, **prometheus**, and **server**. Each of the following containt a systemd unit file for startup.

### Grafana
The only file in here is a configuration file called **datasource.yml** which sets the default datasource to prometheus and to point to the Prometheus server on the network via **http://bbp-prometheus:7878**.

### Prometheus
Like Grafana, there is only one configuration file in here for Prometheus called **prometheus.yml** which sets Prometheus to scrape from **bbp-server:7878**.

### Server
Unlike the previous two directories, this directory contains an entire rust project along with a Dockerfile. The rust project contains all the source **.rs** files and the Dockerfile how to build those files into a container.

## How it Works
BBP functions across three different docker containers all hosted on the docker network **bbp**. At its core lies a main TCP server at **bbp-server:7879** which manages requests from clients (residences) on the local network and uses their supplied payload data to update the running Prometheus server at **bbp-prometheus:7878**. A client will make a request to the server, the server will bind to the client's socket and monitor it asynchronously for data, the data will be parsed as packages and deserialized into rust structs to be logged in the corresponding Prometheus metrics. The first request made by the client has the header **SET_ID** in order to link a residence id with the TCP stream and group metrics with it in the Prometheus database. The following requests have the header "UPDATE_DATA" to send over a JSON payload of sensor data to be deserialized into rust-usable metrics. Whenever a metric is registered during the runtime, it is automatically registered in the Prometheus server due to the **prometheus_exporter** crate. Once these metrics with their values are logged in the Prometheus server, the runtime automatically ensures the Grafana container hosted at **bbp-grafana:3000** contains a folder with the residence id and several dashboards which utilize these newly registered metrics. All-in-all, these three docker containers work together to provide what is Big Brother Prometheus.

## How to Install/Deploy
Simply utilize the **bbp_build.sh** script found in the root of the bbp directory. Run with **./bbp_build.sh** and simply enter your sudo password when prompted and answer any questions accordingly. Near the end you are prompted to enter a Grafana API token. To obtain the token please head to the Grafana dashboard at localhost:3000, click the side menu on the left, click administration, then service accounts, add a new service account with admin, then finally add a token and paste it when prompted. If you mess up this step, simply paste it after the **API_TOKEN=** in bbp/server/.env. If all went well, you should have a BBP server running on your machine's local IP.
