# Configuration

mCaptcha is highly configurable.

Configuration is applied/merged in the following order:

1. path to configuration file passed in via `MCAPTCHA_CONFIG`
2. `./config/default.toml`
3. `/etc/mcaptcha/config.toml`
4. environment variables.

## Setup

### Environment variables

Setting environment variables are optional. The configuration files have
all the necessary parameters listed. By setting environment variables,
you will be overriding the values set in the configuration files.

### General

| Name                          | Value                                                                                                             |
| ----------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| `MCAPTCHA_CONFIG`             | Path to configuration file                                                                                        |
| `MCAPTCHA_COMMERCIAL`         | Does this instance offer commercial plans? Please consider donating if it does :D                                 |
| `MCAPTCHA_SOURCE_CODE`        | Link to the source code of this instance                                                                          |
| `MCAPTCHA_ALLOW_REGISTRATION` | Is registration allowed on this instance?                                                                         |
| `MCAPTCHA_ALLOW_DEMO`         | Allow demo access to the server? If registration(previous option) is disabled then demo users will not be allowed |

#### Database

| Name                                 | Value                                                         |
| ------------------------------------ | ------------------------------------------------------------- |
| `MCAPTCHA_DATEBASE_PASSWORD`         | Postgres password                                             |
| `MCAPTCHA_DATEBASE_NAME`             | Postgres database name                                        |
| `MCAPTCHA_DATEBASE_PORT`             | Postgres port                                                 |
| `MCAPTCHA_DATEBASE_HOSTNAME`         | Postgres hostname                                             |
| `MCAPTCHA_DATEBASE_USERNAME`         | Postgres username                                             |
| `MCAPTCHA_DATEBASE_POOL`             | Postgres database connection pool size                        |
| `DATABSE_URL` (overrides above vars) | databse URL in `postgres://user:pass@host:port/dbname` format |

#### Redis

| Name                  | Value                      |
| --------------------- | -------------------------- |
| `MCAPTCHA_REDIS_URL`  | Redis URL                  |
| `MCAPTCHA_REDIS_POOL` | Redis connection pool size |

#### Server

| Name                                     | Value                                                                              |
| ---------------------------------------- | ---------------------------------------------------------------------------------- |
| `MCAPTCHA_SERVER_PORT`                   | The port on which you want mCaptcha to listen to                                   |
| `PORT`(overrides `MCAPTCHA_SERVER_PORT`) | The port on which you want mCaptcha to listen to                                   |
| `MCAPTCHA_SERVER_IP`                     | The IP address on which you want mCaptcha to listen to                             |
| `MCAPTCHA_SERVER_DOMAIN`                 | Domain under which mCaptcha will be\*                                              |
| `MCAPTCHA_SERVER_COOKIE_SECRET`          | Cookie secret, must be long and random                                             |
| `MCAPTCHA_SERVER_PROXY_HAS_TLS`          | Is mCaptcha behind a proxy? If yes, mCaptcha can send additional headers like HSTS |

\* Authentication doesn't work without `MCAPTCHA_DOMAIN` set to the correct domain

### Proof of work

| Name                | Value                                                                                   |
| ------------------- | --------------------------------------------------------------------------------------- |
| `MCAPTCHA_POW_SALT` | Salt has to be long and random                                                          |
| `MCAPTCHA_POW_GC`   | Garbage collection duration in seconds, requires tuning but 30 is a good starting point |

### SMTP

| Name                     | Value                                           |
| ------------------------ | ----------------------------------------------- |
| `MCAPTCHA_SMTP_FROM`     | email address from which the email will be sent |
| `MCAPTCHA_SMTP_REPLY_TO` | email address to which reply can be sent        |
| `MCAPTCHA_URL`           | SMTP server URL                                 |
| `MCAPTCHA_SMTP_PORT`     | SMTP server port                                |
| `MCAPTCHA_SMTP_USERNAME` | SMTP username                                   |
| `MCAPTCHA_SMTP_PASSWORD` | SMTP password                                   |
