# Configuration

Guard is highly configurable.

Configuration is applied/merged in the following order:

1. path to configuration file passed in via `GUARD_CONFIG`
2. `./config/default.toml`
3. `/etc/guard/config.toml`
4. environment variables.

## Setup

### Environment variables:

Setting environment variables are optional. The configuration files have
all the necessary parameters listed. By setting environment variables,
you will be overriding the values set in the configuration files.

#### Database:

| Name                                 | Value                                                         |
| ------------------------------------ | ------------------------------------------------------------- |
| `GUARD_DATEBASE_PASSWORD`            | Postgres password                                             |
| `GUARD_DATEBASE_NAME`                | Postgres database name                                        |
| `GUARD_DATEBASE_PORT`                | Postgres port                                                 |
| `GUARD_DATEBASE_HOSTNAME`            | Postgres hostmane                                             |
| `GUARD_DATEBASE_USERNAME`            | Postgres username                                             |
| `GUARD_DATEBASE_POOL`                | Postgres database connection pool size                        |
| `DATABSE_URL` (overrides above vars) | databse URL in `postgres://user:pass@host:port/dbname` format |

#### Server:

| Name                                  | Value                                               |
| ------------------------------------- | --------------------------------------------------- |
| `GUARD_SERVER_PORT`                   | The port on which you want guard to listen to       |
| `PORT`(overrides `GUARD_SERVER_PORT`) | The port on which you want guard to listen to       |
| `GUARD_SERVER_IP`                     | The IP address on which you want guard to listen to |
| `GUARD_SERVER_DOMAIN`                 | Domain under which guard will be\*                  |
| `GUARD_SERVER_COOKIE_SECRET`          | Cookie secret, must be long and random              |
| `GUARD_SERVER_ALLOW_REGISTRATION`     | `bool` that controls                                |  | registration |

\* Authentication doesn't work without `GUARD_DOMAIN` set to the correct
domain

### Configuration file location:

| Name           | Value               |
| -------------- | ------------------- |
| `GUARD_CONFIG` | Path to config file |

### Proof of work:

| Name             | Value                                                                                   |
| ---------------- | --------------------------------------------------------------------------------------- |
| `GUARD_POW_SALT` | Salt has to be long and random                                                          |
| `GUARD_POW_GC`   | Garbage collection duration in seconds, requires tuning but 30 is a good starting point |
