# Development Setup

## To quickly make changes:

We have a docker-compose config that you can use to quickly spin up dev
environment.

From the root of the repo, run:

```bash
$ docker-compose -d up
```

### Logs from docker:

- Logs from database and web server as they are generated:

```bash
$ docker-compose logs -f
```

- from just webserver:

```bash
$ docker-compose logs -f mcaptcha
```

## Setting up elaborate development environment

### Toolchain

You'll have to install before you can start writing code.

1. Install Rust:
   Install Cargo(Rust toolchain) using [rustup](https://rustup.rs/) with:

```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Install Node `v14.16.0`:
   Please refer to [official instructions](https://nodejs.org/en/download/)

3. Install yarn:
   `npm install -g yarn`
   For more details, refer to [official
   instructions](https://yarnpkg.com/getting-started/install)

4. GNU Make:
   If you are on Linux, it's probably already installed on your machine.

You can check it's existence by running:

```bash
$ make --version
```

If it's not available, you download it with your package manager or
refer to [official instructions](https://www.gnu.org/software/make/)

### External Dependencies:

### Postgres databse:

The backend requires a Postgres database. We have
compiletime SQL checks so without a database available, you won't be
able to build the project.

I use Postgres in Docker.

1. To install Docker, please refer to [official
   instructions](https://docs.docker.com/engine/install/].

2. Create create database user:

```bash
$ docker create --name mcaptcha-postgres \
	-e POSTGRES_PASSWORD=password \
	-p 5432:5432    postgres
```

3. Start database container:

```bash
$ docker start mcaptcha-postgres
```

4. Set configurations:

```bash
$ cd mcaptcha # your copy of https://github.com/mCaptcha/mcaptcha
$ echo 'export DATABASE_URL="postgres://postgres:password@localhost:5432/postgres"' > .env
```

**NOTE: Don't use this database for other projects**

5. Run migrations:
   This step is only required when migrations are updated. The server
   binary has inbuilt migrations manager but that can only be used after
   the server is compiled. Since we are trying to compile the server here,
   we can't use that.

However, this project ships with a utility to run migrations!

```bash
$ cd mcaptcha # your copy of https://github.com/mCaptcha/mcaptcha
$ cargo run --bin tests-migrate
```

That's it, you are all set!

## Build commands:

### Compile:

```bash
$ cd mcaptcha # your copy of https://github.com/mCaptcha/mcaptcha
$ make
```

### Additional commands:

```bash
➜  mcaptcha git:(master) ✗ make help
  docs      	- build documentation
  run       	- run developer instance
  test 		- run unit and integration tests
  migrate   	- run database migrations
  dev-env 	- download dependencies
  clean     	- drop builds and environments
  coverage 	- build test coverage in HTML format
  xml-coverage 	- build test coverage in XML for upload to codecov
```
