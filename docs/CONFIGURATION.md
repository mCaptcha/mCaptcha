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
| `MCAPTCHA_debug`              | Enable debug logging                                                                                              |
| `MCAPTCHA_config`             | Path to configuration file                                                                                        |
| `MCAPTCHA_commercial`         | Does this instance offer commercial plans? Please consider donating if it does :D                                 |
| `MCAPTCHA_source_code`        | Link to the source code of this instance                                                                          |
| `MCAPTCHA_allow_registration` | Is registration allowed on this instance?                                                                         |
| `MCAPTCHA_allow_demo`         | Allow demo access to the server? If registration(previous option) is disabled then demo users will not be allowed |

### Database

| Name                                  | Value                                                          |
| ------------------------------------- | -------------------------------------------------------------- |
| `MCAPTCHA_database_DATEBASE_POOL`     | database connection pool size                                  |
| `DATABASE_URL` (overrides above vars) | database URL in `postgres://user:pass@host:port/dbname` format |

### Redis

| Name                  | Value                      |
| --------------------- | -------------------------- |
| `MCAPTCHA_redis_URL`  | Redis URL                  |
| `MCAPTCHA_redis_POOL` | Redis connection pool size |

### Server

| Name                            | Value                                                                              |
| ------------------------------- | ---------------------------------------------------------------------------------- |
| `PORT`                          | The port on which you want mCaptcha to listen to                                   |
| `MCAPTCHA_server_IP`            | The IP address on which you want mCaptcha to listen to                             |
| `MCAPTCHA_server_DOMAIN`        | Domain under which mCaptcha will be\*                                              |
| `MCAPTCHA_server_COOKIE_SECRET` | Cookie secret, must be long and random                                             |
| `MCAPTCHA_server_PROXY_HAS_TLS` | Is mCaptcha behind a proxy? If yes, mCaptcha can send additional headers like HSTS |

\* Authentication doesn't work without `MCAPTCHA_DOMAIN` set to the correct domain

### Captcha

| Name                                                                               | Value                                                                                                                                 |
| ---------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| `MCAPTCHA_captcha_SALT`                                                            | Salt has to be long and random                                                                                                        |
| `MCAPTCHA_captcha_GC`                                                              | Garbage collection duration in seconds, requires tuning but 30 is a good starting point                                               |
| `MCAPTCHA_captcha_RUNNERS`                                                         | [Performance] Number of runners to use for PoW validation. Defaults to number of CPUs available                                       |
| `MCAPTCHA_captcha_QUEUE_LENGTH`                                                    | [Performance] PoW Validation queue length, controls how many pending validation jobs can be held in queue                             |
| `MCAPTCHA_captcha_ENABLE_STATS`                                                    | Record for CAPTCHA events like configuration fetch, solves and authentication of validation token. Useful for commercial deployments. |
| `MCAPTCHA_captcha_DEFAULT_DIFFICULTY_STRATEGY_avg_traffic_difficulty`              | Default difficulty factor to use in easy mode CAPTCHA configuration estimation for average traffic metric                             |
| `MCAPTCHA_captcha_DEFAULT_DIFFICULTY_STRATEGY_avg_traffic_time`                    | This difficulty factor is used in to use in easy mode CAPTCHA configuration estimation for average traffic metric                     |
| `MCAPTCHA_captcha_DEFAULT_DIFFICULTY_STRATEGY_peak_sustainable_traffic_difficulty` | Default difficulty factor to use in easy mode CAPTCHA configuration estimation for peak traffic metric                                |
| `MCAPTCHA_captcha_DEFAULT_DIFFICULTY_STRATEGY_peak_sustainable_traffic_time`       | This difficulty factor is used in to use in easy mode CAPTCHA configuration estimation for peak traffic metric                        |
| `MCAPTCHA_captcha_DEFAULT_DIFFICULTY_STRATEGY_broke_my_site_traffic_difficulty`    | Default difficulty factor to use in easy mode CAPTCHA configuration estimation for traffic that took the website down                 |
| `MCAPTCHA_captcha_DEFAULT_DIFFICULTY_STRATEGY_broke_my_site_traffic_time`          | Default time (in seconds) to use to compute difficulty factor using stored PoW performance records.                                   |
| `MCAPTCHA_captcha_DEFAULT_DIFFICULTY_STRATEGY_duration`                            | Default duration to use in CAPTCHA configuration in easy mode                                                                         |

See commits [`54b14291ec140e`](https://github.com/mCaptcha/mCaptcha/commit/54b14291ec140ea4cbbf73462d3d6fc2d39f2d2c) and [`42544ec421e0`](https://github.com/mCaptcha/mCaptcha/commit/42544ec421e0c3ec4a8d132e6101ab4069bf0065) for more info.

### SMTP

| Name                     | Value                                           |
| ------------------------ | ----------------------------------------------- |
| `MCAPTCHA_smtp_FROM`     | email address from which the email will be sent |
| `MCAPTCHA_smtp_URL`      | SMTP server URL                                 |
| `MCAPTCHA_smtp_PORT`     | SMTP server port                                |
| `MCAPTCHA_smtp_USERNAME` | SMTP username                                   |
| `MCAPTCHA_smtp_PASSWORD` | SMTP password                                   |
