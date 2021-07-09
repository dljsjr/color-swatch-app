# CColor Color Swatch App

The extra C stands for Cool.

This is a quick-and-dirty app demonstrating some basic web app development skills.

## Quick Start

If you have Docker installed, the easiest way to get up and running quickly is to edit the Caddyfile to read as follows:

```
http://localhost { # <- We change this here to localhost and disable auto HTTPS by specifying HTTP
    templates

    encode gzip zstd

    try_files {path}.html {path}

    reverse_proxy * app:3000
}
```

Then `docker compose up -d`. Once the whole cluster has started the app can be viewed at http://localhost

The Caddy proxy attempts to bind directly to 80 and 443 so if your Docker setup can't open those ports on the host then you may run in to issues.

## Brief Architecture Overview

The app uses a services architecture, consisting of an `api` service, an `app` service with both a backend and frontend, a relational databased provided by PostgreSQL, and an auto-SSL-enabled `proxy` Caddy service to reverse proxy w/ HTTPS.

### Directories
- `api`: Contains the Dockerfile and source code for the REST API, written in Rust using the Rocket 0-5.0 release candidate.
- `app`: Contains the Dockerfile and source code for the web app, written in TypeScript using SvelteKit, backend running on node.js
- `scripts`: Contains utility scripts, currently the only script is a simple Python 3 script for populating the database over REST calls.

The root directory contains a `Caddyfile` that gets mounted inside of the `proxy` container for easy proxy configuration changes. The root directory also contains the `docker-compose.yml` with the expose ports commented out which can be re-enabled for debugging purposes.

The `scripts/color_generator` directory contains a `requirements.txt` file to install the required Python dependencies. Install with `pip3 install -r requirements.txt` from inside of the directory to install them (not tested w/ Python 2). `./color_gen.py --help` will show usage of the script.

### Helpful Human Tasks Completed

This repo is based on the assignment found here: https://github.com/HelpfulHuman/interview-challenge

#### Core Goals

- [x] Replicate Design
  - [x] Font
  - [x] Styles
  - [x] Iconography
- [x] Replicate Functionality 
  - [x] Create a database of colors (minimum 100)
  - [x] Paginate data
  - [x] Display the swatch and the label in the preview
  - [x] Ability to select a random color
  - [x] Clicking the swatch changes to a detail view

#### Stretch Goals
- [ ] Design
  - [ ]  Make it responsive
    - **The design uses responsive breakpoints but it lacks a small screen/touch UX for mobile devices**
- [x] Functionality
  - [x] Generate color list from a script

#### Bonus Stretch Goals
- [ ] Design
  - [ ] include interaction design
  - [ ] Add tints/shades in detail view
- [x] Functionality
  - [x] Add search functionality
  - [x] Color generation script guarantees same colors and order
  - [x] Group by color (make sidebar menu functional)
- [ ] Data
  - [ ] Fetch data with GraphQL