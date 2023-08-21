# Overview of Weather Station 

## Directory Structure
The Residence directory is composed by three sub directories, **arduino**, **client**, and **serial**.

### Apache
The Apache directory only contains a configuration file to point it to the django project.

### Pimoroni
Pimoroni consists of the python code interfacing the GPIO on the Raspberry PI as well as a shell script which transfers the collected data to the webserver.

### Webserver
The webserver contains a complete Django project, bundled in with static CSS/JS files and the HTML template. A Dockerfile is also included to containerize it.

## How it Works

## How to Install/Deploy
