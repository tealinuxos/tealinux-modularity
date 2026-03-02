# 🛠️ Services Configuration — Tealinux Modularitea

This document describes all **systemd services** that are automatically enabled by Modularitea profiles, along with verification commands and configuration guides for each service.

> **How services work in Modularitea:**  
> When a profile is applied, Modularitea calls `systemctl enable --now <service>` for every entry listed in the profile's `[services] enable = [...]` array.  
> After the profile installation finishes you can verify each service using the guides below.

---

## 📋 Quick Reference — Services per Profile

| Profile       | Services Enabled                        | Category    |
|---------------|-----------------------------------------|-------------|
| **Backend**   | `docker`, `postgresql`, `redis`, `nginx`| Development |
| **DevOps**    | `docker`                                | Development |
| **Hacking**   | *(none)*                                | Security    |
| **Programmer**| *(none)*                                | Development |

---

## 🐋 Docker

**Profiles that enable this service:** `Backend`, `DevOps`

Docker provides container runtime for running isolated application environments.

### Verify Installation

```bash
# Check Docker version
docker version
```

Expected output:
```
Client:
 Version:           27.x.x
 API version:       1.47
 ...

Server: Docker Engine
 Engine:
  Version:          27.x.x
  ...
```

### Verify Service Status

```bash
# Check if the docker daemon (service) is running
systemctl status docker

# Or use docker itself to check daemon connectivity
docker info
```

### Verify Running Containers

```bash
# List all running containers
docker ps

# List ALL containers (including stopped)
docker ps -a

# Example output:
# CONTAINER ID   IMAGE     COMMAND   CREATED   STATUS    PORTS     NAMES
# (empty if no containers are running — this is normal after a fresh install)
```

### Post-Install Setup

```bash
# Add your user to the docker group so you can run docker without sudo
sudo usermod -aG docker $USER

# Apply the group change (or log out and back in)
newgrp docker

# Test without sudo
docker run hello-world
```

### Enable/Disable Service Manually

```bash
# Enable and start docker on boot
sudo systemctl enable --now docker

# Stop docker service
sudo systemctl stop docker

# Disable docker from starting on boot
sudo systemctl disable docker
```

---

## 🐘 PostgreSQL

**Profiles that enable this service:** `Backend`

PostgreSQL is a powerful, open-source relational database system.

### Verify Installation

```bash
# Check PostgreSQL version
psql --version
# or
postgres --version
```

Expected output:
```
psql (PostgreSQL) 16.x
```

### Verify Service Status

```bash
# Check if PostgreSQL service is running
systemctl status postgresql
```

### First-Time Setup (Required after install)

PostgreSQL requires initialization before it can start:

```bash
# Step 1: Initialize the database cluster
sudo -u postgres initdb -D /var/lib/postgres/data

# Step 2: Enable and start the service
sudo systemctl enable --now postgresql

# Step 3: Connect to verify
sudo -u postgres psql

# Inside psql shell:
\l          -- list all databases
\du         -- list all users/roles
\q          -- quit psql
```

### Create a User and Database

```bash
# Enter PostgreSQL shell as postgres superuser
sudo -u postgres psql

# Inside psql:
CREATE USER myuser WITH PASSWORD 'mypassword';
CREATE DATABASE mydb OWNER myuser;
GRANT ALL PRIVILEGES ON DATABASE mydb TO myuser;
\q
```

### Connect to a Database

```bash
# Connect as a specific user to a specific database
psql -U myuser -d mydb -h localhost

# Connect as postgres superuser
sudo -u postgres psql -d mydb
```

### Check Running Connections

```bash
# Show active connections (run inside psql)
sudo -u postgres psql -c "SELECT pid, usename, application_name, state FROM pg_stat_activity;"
```

---

## 🔴 Redis

**Profiles that enable this service:** `Backend`

Redis is an in-memory key-value store used for caching, sessions, and pub/sub messaging.

### Verify Installation

```bash
# Check Redis version
redis-cli --version
```

Expected output:
```
redis-cli 7.x.x
```

### Verify Service Status

```bash
# Check if Redis service is running
systemctl status redis
```

### Enable/Start Service

```bash
# Enable and start redis on boot
sudo systemctl enable --now redis
```

### Test Redis Connectivity

```bash
# Connect to Redis CLI
redis-cli

# Inside redis-cli:
ping          # should return: PONG
set foo bar   # set a key
get foo       # get the key → returns "bar"
exit
```

### Monitor Redis

```bash
# Real-time commands monitor
redis-cli monitor

# Get server info and stats
redis-cli info

# Check memory usage
redis-cli info memory

# List all keys (use with caution on production)
redis-cli keys "*"
```

### Redis Configuration File

The default config file is located at:
```
/etc/redis/redis.conf
```

Common settings to adjust:
```ini
# Bind to all interfaces (default: 127.0.0.1 — loopback only)
bind 127.0.0.1

# Set password authentication
requirepass yourpassword

# Maximum memory usage
maxmemory 256mb
maxmemory-policy allkeys-lru
```

After editing the config, restart the service:
```bash
sudo systemctl restart redis
```

---

## 🌐 Nginx

**Profiles that enable this service:** `Backend`

Nginx is a high-performance web server and reverse proxy.

### Verify Installation

```bash
# Check Nginx version
nginx -v
```

Expected output:
```
nginx version: nginx/1.26.x
```

### Verify Service Status

```bash
# Check if Nginx service is running
systemctl status nginx
```

### Enable/Start Service

```bash
# Enable and start nginx on boot
sudo systemctl enable --now nginx
```

### Test Nginx is Serving

```bash
# Default page should be accessible at http://localhost
curl -I http://localhost

# Expected response:
# HTTP/1.1 200 OK
# Server: nginx/1.26.x
```

### Test Configuration Validity

```bash
# Validate nginx config before reloading
sudo nginx -t

# Expected output:
# nginx: the configuration file /etc/nginx/nginx.conf syntax is ok
# nginx: configuration file /etc/nginx/nginx.conf test is successful
```

### Reload vs Restart

```bash
# Graceful reload (no downtime — use after editing config)
sudo systemctl reload nginx

# Full restart (use if reload doesn't work)
sudo systemctl restart nginx
```

### Configure a Virtual Host (Server Block)

Create a new site config at `/etc/nginx/sites-available/myapp.conf`:

```nginx
server {
    listen 80;
    server_name myapp.local;

    root /var/www/myapp;
    index index.html index.htm;

    location / {
        try_files $uri $uri/ =404;
    }
}
```

Enable the site:
```bash
sudo ln -s /etc/nginx/sites-available/myapp.conf /etc/nginx/sites-enabled/
sudo nginx -t && sudo systemctl reload nginx
```

### Reverse Proxy to an App (e.g., Node.js on port 3000)

```nginx
server {
    listen 80;
    server_name api.myapp.local;

    location / {
        proxy_pass http://localhost:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }
}
```

### View Nginx Logs

```bash
# Access log
tail -f /var/log/nginx/access.log

# Error log
tail -f /var/log/nginx/error.log
```

---

## 📦 Profile-by-Profile Summary

### 🔷 Backend Profile

**File:** `profiles/backend.toml`  
**Services enabled:** `docker`, `postgresql`, `redis`, `nginx`

All four services are enabled automatically. After applying this profile:

```bash
# 1. Verify Docker
docker version
docker ps

# 2. Verify PostgreSQL
systemctl status postgresql
sudo -u postgres psql -c "SELECT version();"

# 3. Verify Redis
systemctl status redis
redis-cli ping   # → PONG

# 4. Verify Nginx
systemctl status nginx
curl -I http://localhost
```

> ⚠️ **Note:** PostgreSQL requires manual initialization (`initdb`) before it can start for the first time. See the [PostgreSQL section](#-postgresql) above.

---

### 🔷 DevOps Profile

**File:** `profiles/devops.toml`  
**Services enabled:** `docker`

Only Docker is enabled. After applying this profile:

```bash
# Verify Docker daemon is running
systemctl status docker

# Check Docker version
docker version

# Check running containers (empty on fresh install)
docker ps

# Add your user to the docker group
sudo usermod -aG docker $USER && newgrp docker
```

---

### 🔷 Hacking Profile

**File:** `profiles/hacking.toml`  
**Services enabled:** *(none)*

No services are enabled by this profile. All tools (`nmap`, `wireshark`, `john`, etc.) are standalone CLI or GUI applications that do not require system services.

```bash
# Verify tools are installed
nmap --version
wireshark --version
john --version
aircrack-ng --version
hashcat --version
```

---

### 🔷 Programmer Profile

**File:** `profiles/programmer.toml`  
**Services enabled:** *(none)*

No services are enabled by this profile. All tools are standalone development utilities.

```bash
# Verify tools are installed
git --version
vim --version
nvim --version
node --version
npm --version
python --version
```

---

## 🔧 General Service Management Cheatsheet

Use these `systemctl` commands for any service listed above:

```bash
# Check service status
systemctl status <service>

# Start a service
sudo systemctl start <service>

# Stop a service
sudo systemctl stop <service>

# Restart a service
sudo systemctl restart <service>

# Enable service to start on boot
sudo systemctl enable <service>

# Enable and start immediately
sudo systemctl enable --now <service>

# Disable service from starting on boot
sudo systemctl disable <service>

# View service logs (real-time)
sudo journalctl -u <service> -f

# View last 50 lines of service logs
sudo journalctl -u <service> -n 50
```

---

## 🧩 Docker Compose Quick Guide

Both **Backend** and **DevOps** profiles install `docker-compose`. Here's a quick reference:

```bash
# Check docker-compose version
docker compose version

# Start services defined in docker-compose.yml
docker compose up -d

# List running compose services
docker compose ps

# View logs for all services
docker compose logs -f

# Stop all compose services
docker compose down

# Rebuild and restart
docker compose up -d --build
```

---

*Documentation generated for Tealinux Modularitea — TeaLinuxOS Team*
