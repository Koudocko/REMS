# Overview of BBP (Big Brother Prometheus) 

## Directory Structure
* **weather-pimoroni.service**: Systemd unit file
    * Unit is named **weather-pimoroni**

The BBP directory is composed by three sub directories, **grafana**, **prometheus**, and **server**. Each of the following containt a systemd unit file for startup.

### Grafana
* **bbp-grafana.service**: Systemd unit file
    * Unit is named **bbp-grafana**
* **datasource.yml**: Grafana configuration file
    * Sets default datasource to Prometheus
    * Points to URL http://bbp-prometheus:7878

### Prometheus
* **bbp-prometheus.service**: Systemd unit file
    * Unit is named **bbp-prometheus**
* **prometheus.yml**: Prometheus configuration file
    * Sets scrape location to bbp-server:7878

### Server
* **Dockerfile**: Docker image build file
    * Image and container are named **bbp-server**
    * Builds Rust project with Cargo
    * Exposes server port 7879
    * Runs finished executable
* **bbp-server.service**: Systemd unit file
    * Unit is named **bbp-server**
* **Cargo.toml**: Rust project specification
    * Specifies executable name as bbp-server
    * Adds serde, tokio, reqwest, serde_json, chrono, threadpool, prometheus, prometheus_exporter, local-ip-address, and dotenvy as dependencies
* **src/main.rs**: Main Rust file
    * Pulls Grafana API token from .env file
    * Asynchronously handles all incoming client residence TCP connections
    * Logs connection information via stdout and log file
    * Validates package syntax and matches header for endpoint
    * Responds to client with response feedback
* **src/commands.rs**: Request commands' implementations
    * Associate residence ID with TCP stream
    * Upload collected metrics to Prometheus server
    * Create Grafana dashboards/folders for new residence IDs

## How it Works
BBP functions across three different docker containers all hosted on the docker network **bbp**. At its core lies a main TCP server at **bbp-server:7879** which manages requests from clients (residences) on the local network and uses their supplied payload data to update the running Prometheus server at **bbp-prometheus:7878**. A client will make a request to the server, the server will bind to the client's socket and monitor it asynchronously for data, the data will be parsed as packages and deserialized into rust structs to be logged in the corresponding Prometheus metrics. The first request made by the client has the header **SET_ID** in order to link a residence id with the TCP stream and group metrics with it in the Prometheus database. The following requests have the header "UPDATE_DATA" to send over a JSON payload of sensor data to be deserialized into rust-usable metrics. Whenever a metric is registered during the runtime, it is automatically registered in the Prometheus server due to the **prometheus_exporter** crate. Once these metrics with their values are logged in the Prometheus server, the runtime automatically ensures the Grafana container hosted at **bbp-grafana:3000** contains a folder with the residence id and several dashboards which utilize these newly registered metrics. All-in-all, these three docker containers work together to provide what is Big Brother Prometheus.

## How to Install/Deploy
Simply utilize the **bbp_build.sh** script found in the root of the bbp directory. Run with **./bbp_build.sh** and simply enter your sudo password when prompted and answer any questions accordingly. Near the end you are prompted to enter a Grafana API token. To obtain the token please head to the Grafana dashboard at localhost:3000, click the side menu on the left, click administration, then service accounts, add a new service account with admin, then finally add a token and paste it when prompted. If you mess up this step, simply paste it after the **API_TOKEN=** in **/rems/files/server/.env**. If all went well, you should have a BBP server running on your machine's local IP.
