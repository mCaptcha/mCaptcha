<div align="center">
  <h1>mCaptcha Guard</h1>
  <p>
    <strong>Back-end component of mCaptcha</strong>
  </p>

[![Documentation](https://img.shields.io/badge/docs-master-blue)](https://mcaptcha.github.io/mCaptcha/guard/index.html)
![CI (Linux)](<https://github.com/mCaptcha/guard/workflows/CI%20(Linux)/badge.svg>)
[![dependency status](https://deps.rs/repo/github/mCaptcha/guard/status.svg)](https://deps.rs/repo/github/mCaptcha/guard)
[![codecov](https://codecov.io/gh/mCaptcha/guard/branch/master/graph/badge.svg)](https://codecov.io/gh/mCaptcha/guard)
<br />
[![AGPL License](https://img.shields.io/badge/license-AGPL-blue.svg)](http://www.gnu.org/licenses/agpl-3.0)
</div>

</div>

**placeholder-repo** is an placeholder-repo and access management platform built for the
[IndieWeb](indieweb.org)

### How to build

- Install Cargo using [rustup](https://rustup.rs/) with:

```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- Clone the repository with:

```
$ git clone https://github.com/mCaptcha/guard
```

- Build with Cargo:

```
$ cd guard && cargo build
```

### Configuration:

placeholder-repo is highly configurable.
Configuration is applied/merged in the following order:

1. `config/default.toml`
2. environment variables.

To make installation process seamless, placeholder-repo ships with a CLI tool to
assist in database migrations.

#### Setup

##### Environment variables:

Setting environment variables are optional. The configuration files have
all the necessary parameters listed. By setting environment variables,
you will be overriding the values set in the configuration files.

###### Database:

| Name                            | Value                                  |
| ------------------------------- | -------------------------------------- |
| `PLACEHOLDER_DATEBASE_PASSWORD` | Postgres password                      |
| `PLACEHOLDER_DATEBASE_NAME`     | Postgres database name                 |
| `PLACEHOLDER_DATEBASE_PORT`     | Postgres port                          |
| `PLACEHOLDER_DATEBASE_HOSTNAME` | Postgres hostmane                      |
| `PLACEHOLDER_DATEBASE_USERNAME` | Postgres username                      |
| `PLACEHOLDER_DATEBASE_POOL`     | Postgres database connection pool size |

###### Redis cache:

| Name                         | Value          |
| ---------------------------- | -------------- |
| `PLACEHOLDER_REDIS_PORT`     | Redis port     |
| `PLACEHOLDER_REDIS_HOSTNAME` | Redis hostmane |

###### Server:

| Name                                      | Value                                               |
| ----------------------------------------- | --------------------------------------------------- |
| `PLACEHOLDER_SERVER_PORT` (or) `PORT`\*\* | The port on which you want wagon to listen to       |
| `PLACEHOLDER_SERVER_IP`                   | The IP address on which you want wagon to listen to |
| `PLACEHOLDER_SERVER_STATIC_FILES_DIR`     | Path to directory containing static files           |
