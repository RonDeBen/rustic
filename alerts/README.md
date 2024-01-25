# Rustic Alert Service
## Overview

Rustic Alert Service is a component of the Rustic project designed to monitor workday progress and send notifications. It's a background service that checks for specific conditions (like nearing the end of an 8-hour workday) and alerts the user accordingly.
# Features

    Monitors the total time worked in a day.
    Sends notifications when certain thresholds are reached.
    Configurable via environment variables.

# Environment Variables

The behavior of the Rustic Alert Service can be customized through the following environment variables:
> `SERVER_URL`: URL of the Rustic server. Default is http://localhost:8001. \
>  `CHECK_INTERVAL_SECS`: Interval in seconds for how often the service checks the time entries. Default is 300 seconds (5 minutes).

# Installation
Clone the Rustic project:
```bash
git clone https://github.com/your-username/rustic.git
cd rustic
```

Build the alert service:

```bash
cargo build --release -p alerts
```

Copy the executable to a suitable location:

```bash
sudo cp target/release/alerts /usr/local/bin
```

# Setting Up as a Systemd Service

To enable the Rustic Alert Service to start on system boot:

Create a systemd service file:

```bash
sudo nano /etc/systemd/system/rustic-alert.service
```


Add the following content to the service file:

```ini
[Unit]
Description=Rustic Alert Service

[Service]
ExecStart=/usr/local/bin/alerts
Restart=always

[Install]
WantedBy=multi-user.target
```

Enable and start the service:


```bash
sudo systemctl daemon-reload
sudo systemctl enable rustic-alert.service
sudo systemctl start rustic-alert.service
```

Check the service status:

```bash
sudo systemctl status rustic-alert.service
```


