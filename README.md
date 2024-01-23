# AudioHub



## Overview

AudioHub is a comprehensive digital platform designed for content creators to share their audiobooks.  

The website is deployed and published in a Kubernetes cluster. 

Use this link https://audiohub.dyn.cloud.e-infra.cz/ to evaluate the app without having to set up your own Postgres instance and clone this repo
## Features

- Book Management: Enables users to browse, purchase, and view books. Each book has properties like name, price, stock in storage, overall rating, genres, publisher, and authors.
- Author and Publisher Management: Provides administrative privileges to view, add, update, and delete authors and publishers.
- User Account Management: Allows customers to create accounts, review their purchase history, rate books, and create wishlists.
- Rating Management: Facilitates users to rate books and leave comments. The rating value and comments can be updated.

## Architecture
The application is built using Actix, Askama and HTMX. 

## Components
The application has several components each handling a specific functionality:

- Book Management: This component is responsible for all operations related to books. It allows users to view, add, update, and delete books. Each book has properties like name, price, stock in storage, overall rating, genres, publisher, and authors.
- Author and Publisher Management: This component handles all operations related to authors and publishers. It allows users to view, add, update, and delete authors and publishers.
- Rating Management: This is where users can rate books and leave comments. The rating value and comments can be updated.
- Request Logging: All requests to the application are logged with details like the request method and path. The logs are stored in a text file.


## Database Schema
![ERD Diagram](db.png)

## Setup
We have provided multiple options to test and deploy this web application for you. 

If you choose a local deployment, please bear in mind that media files are stored locally for obvious reasons (DB size, RAM). 
But we will include a couple of short songs and thumbnails in this repository (by subtracting them from .gitignore with `!`) to showcase the functionality out of the box. 

You should add your owns books though.  

### Kubernetes
The application is deployed using CERIT-SC's Kubernetes cluster: https://audiohub.dyn.cloud.e-infra.cz/ for your convenience.

While we do not intend to modify our project after submission, we understand your need to check the project at the time of submission. Therefore, we provided other options to test the app locally.

### Local app - Postgres in Kubernetes
This project uses the Postgres database deployed in Kubernetes with the Cloudnative-PG operator. 

Our `.env` file is obviously included in `.gitignore`, but since we will have given you the maintainer role in our Gitlab repository, you should be able to get all the data from the environment variables there https://gitlab.fi.muni.cz/xmarianc/pv281-project/-/settings/ci_cd

Please use these secrets modify the `example.env` file. 

### Local app - Local Postgres with our DB contents
If you do not wish to use our database you can always restore the database from the database dump file `db_dump.sql` and then reload it with this command `psql -d audiobooks -f db_dump.sql`.


## Deployment
We chose the CERIT-SC's Kubernetes cluster to deploy this application. YAML manifests used to do so are located in the `kubernetes` folder.

The Kubernetes' NGINX ingress controller automatically creates a trusted certificate and sets up the NGINX reverse proxy.  

There are other options too, but you should secure the website with a reverse proxy yourself:
- for bare metal deployment you could run `cargo build` and then create a *systemd* unit to manage it.
- for Docker deployment you should firstly run `docker build -t audiohub-image .` to build the image and then `docker run -p 80:8000 --name audiohub audiohub-image --restart=always`.
- for Podman deployment the steps would be similar `podman build -t audiohub-image .` to build the image. But Podman containers are daemon-less, therefore it is recommended to use so called *Quadlets* to manage Podman containers with *systemd*.

Something like this should work. Note that I did not test it, paths may be incorrect. I provide this example just as a curiosity.
```shell
[Unit]
Description=AudioHub
After=network-online.target

[Container]
Image=audiohub-image
ContainerName=audiohub
PublishPort=80:8000
Volume={{ local_volume_path }}:/usr/src/audiobooks/media:z

[Service]
Restart=always
TimeoutStartSec=900

[Install]
WantedBy=multi-user.target default.target
```

***
