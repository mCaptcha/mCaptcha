# Deployment instructions:

See [CONFIGURATION.md](./CONFIGURATION.md) for configuration instructions

There are three ways to deploy mCaptcha:

1. Docker
2. Docker compose
3. Bare metal

## Docker

NOTE: We'll publish pre-built images once we reach `alpha`.

1. Build image:

```bash
$ cd mcaptcha && docker build -t mcaptcha/mcaptcha:latest .
```

2. Set configuration in [configuration file](../config/default.toml)

3. Run image:

If you have already have a Postgres instance running, then:

```bash
docker run -p <host-machine-port>:<port-in-configuration-file> \
	--add-host=database:<database-ip-addrss> \
	-e RUST_LOG=debug \
	-e DATABASE_URL="postgres://<db-user>:<db-password>@database:<db-port>/<db-name>" \
	mcaptcha/mcaptcha:latest
```

If you don't have a Postgres instance running, you can either install
one using a package manager or launch one with docker. A [docker-compose
configuration](../docker-compose.yml) is available that will launch both
a database instance mcaptcha instance.

## With docker-compose

1. Follow steps above to build docker image.

2. Set database password [docker-compose configuration](../docker-compose.yml).

3. Launch network:

```bash
$ docker-compose up -d
```

## Bare metal:

The process is tedious, most of this will be automated with a script in
the future.

### 1. Install postgres if you don't have it already.

### 2. Create new user for running `mcaptcha`:

```bash
$ sudo useradd -b /srv -m -s /usr/bin/zsh mcaptcha
```

### 3. Create new user in Postgres

```bash
$ sudo -iu postgres # switch to `postgres` user
$ psql
postgres=#  CREATE USER mcaptcha WITH PASSWORD 'my super long password and yes you need single quote`;
$  createdb -O mcaptcha mcaptcha # create db 'mcaptcha' with 'mcaptcha' as owner
```

### 4. Install and load [`mCaptcha/cache`](https://github.com/mCaptcha/cache) module:

See [`mCaptcha/cache`](https://github.com/mCaptcha/cache) for more
details.

### 4. Build `mcaptcha`:

To build `mcaptcha`, you need the following dependencies:

1. rust
2. node(`v14.16.0`)
3. yarn(JavaScript package manager)
4. make

## How to build

1. Install Cargo using [rustup](https://rustup.rs/) with:

```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Install node(`v14.16.0`)

3. Install yarn(JavaScript package manager)

4. Build with make:

```bash
$ make dev-env && \
	make release
```

### 5. Install package:

```bash
$ sudo cp ./target/release/mcaptcha /usr/bin/ && \
	mkdir sudo /etc/mcaptcha && \
	sudo cp config/default.toml /etc/mcaptcha/config.toml
```

### 6. Systemd service configuration:

1. Copy the following to `/etc/systemd/system/mcaptcha.service`:

```systemd
[Unit]
Description=mCaptcha: a CAPTCHA system that gives attackers a run for their money

[Service]
Type=simple
User=mcaptcha
ExecStart=/usr/bin/mcaptcha
Restart=on-failure
RestartSec=1
SuccessExitStatus=3 4
RestartForceExitStatus=3 4
SystemCallArchitectures=native
MemoryDenyWriteExecute=true
NoNewPrivileges=true
Environment="RUST_LOG=info"

[Unit]
After=sound.target
Wants=network-online.target
Wants=network-online.target
Requires=postgresql.service
After=syslog.target

[Install]
WantedBy=multi-user.target
```

2. Enable service:

```bash
$ sudo systemctl daemon-reload && \
	sudo systemctl enable mcaptcha && \ # Auto startup during boot
	sudo systemctl start mcaptcha
``
```
