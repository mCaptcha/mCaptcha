<div align="center">
  <h1>mCaptcha Guard</h1>
  <p>
    <strong>Back-end component of mCaptcha</strong>
  </p>

[![Documentation](https://img.shields.io/badge/docs-master-blue)](https://mcaptcha.github.io/guard/guard/)
![CI (Linux)](<https://github.com/mCaptcha/guard/workflows/CI%20(Linux)/badge.svg>)
[![dependency status](https://deps.rs/repo/github/mCaptcha/guard/status.svg)](https://deps.rs/repo/github/mCaptcha/guard)
[![codecov](https://codecov.io/gh/mCaptcha/guard/branch/master/graph/badge.svg)](https://codecov.io/gh/mCaptcha/guard)
<br />
[![AGPL License](https://img.shields.io/badge/license-AGPL-blue.svg)](http://www.gnu.org/licenses/agpl-3.0)

</div>

</div>

Guard is the back-end component of [mCaptcha](https://mcaptcha.org)
system.

**STATUS: UNUSABLE BUT ACTIVE DEVELOPMENT** 

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

Guard is highly configurable.
Configuration is applied/merged in the following order:

1. `config/default.toml`
2. environment variables.


#### Setup

##### Environment variables:

Setting environment variables are optional. The configuration files have
all the necessary parameters listed. By setting environment variables,
you will be overriding the values set in the configuration files.

###### Database:

| Name                      | Value                                  |
| ------------------------- | -------------------------------------- |
| `GUARD_DATEBASE_PASSWORD` | Postgres password                      |
| `GUARD_DATEBASE_NAME`     | Postgres database name                 |
| `GUARD_DATEBASE_PORT`     | Postgres port                          |
| `GUARD_DATEBASE_HOSTNAME` | Postgres hostmane                      |
| `GUARD_DATEBASE_USERNAME` | Postgres username                      |
| `GUARD_DATEBASE_POOL`     | Postgres database connection pool size |

###### Server:

| Name                                | Value                                               |
| ----------------------------------- | --------------------------------------------------- |
| `GUARD_SERVER_PORT` (or) `PORT`\*\* | The port on which you want wagon to listen to       |
| `GUARD_SERVER_IP`                   | The IP address on which you want wagon to listen to |
| `GUARD_SERVER_STATIC_FILES_DIR`     | Path to directory containing static files           |
