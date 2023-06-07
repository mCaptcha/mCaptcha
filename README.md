<div align="center">
<img width="100px" alt="mcaptcha logo" src="./docs/res/icon-trans.png" />
  <h1>mCaptcha</h1>
  <p>
    <strong>
Proof of work based, privacy respecting CAPTCHA system with a kickass UX. 
</strong>
  </p>

[![Documentation](https://img.shields.io/badge/docs-master-blue?style=flat-square)](https://mcaptcha.github.io/mCaptcha/mCaptcha/)
[![Build](https://github.com/mCaptcha/mCaptcha/actions/workflows/linux.yml/badge.svg)](https://github.com/mCaptcha/mCaptcha/actions/workflows/linux.yml)
[![Docker](https://img.shields.io/docker/pulls/mcaptcha/mcaptcha)](https://hub.docker.com/r/mcaptcha/mcaptcha)
[![dependency status](https://deps.rs/repo/github/mCaptcha/mCaptcha/status.svg?style=flat-square)](https://deps.rs/repo/github/mCaptcha/mCaptcha)
[![codecov](https://codecov.io/gh/mCaptcha/mCaptcha/branch/master/graph/badge.svg?style=flat-square)](https://codecov.io/gh/mCaptcha/mCaptcha)
<br />
[![AGPL License](https://img.shields.io/badge/license-AGPL-blue.svg?style=flat-square)](http://www.gnu.org/licenses/agpl-3.0)
[![Chat](https://img.shields.io/badge/matrix-+mcaptcha:matrix.batsense.net-purple?style=flat-square)](https://matrix.to/#/+mcaptcha:matrix.batsense.net)

**STATUS: ACTIVE DEVELOPMENT**

</div>

</div>

**Skip to [demo](#demo)**

[mCaptcha](https://mcaptcha.org) is a privacy respecting, _free_ CAPTCHA
system with a kickass UX. Your users no longer have to interact with
ridiculous image-based CAPTCHA system, wasting precious mental
bandwidth. Instead, your computer will do the work for you, [see for
yourself!](https://demo.mcaptcha.org/widget/?sitekey=pHy0AktWyOKuxZDzFfoaewncWecCHo23)

## How does it work?

mCaptcha uses SHA256 based proof-of-work (PoW) to rate limit users.

When a user wants to do something on a mCaptcha-protected website,

1. they will have to generate proof-of-work (a bunch of math that will takes
   time to compute) and submit it to mCaptcha.

2. We'll validate the proof:

    - **if validation is unsuccessful**, they will be prevented from
      accessing their target website
    - **if validation is successful**, read on,

3. They will be issued a token that they should submit along
   with their request/form submission to the target website.

4. The target website should validate the user-submitted token with mCaptcha
   before processing the user's request.

The whole process is automated from the user's POV. All they have to do
is click on a button to initiate the process.

mCaptcha makes interacting with websites (computationally) expensive for
the user. A well-behaving user will experience a slight delay (no delay
when under moderate load to 2s when under attack; PoW difficulty is
variable) but if someone wants to hammer your site, they will have to do
more work to send requests than your server will have to do to respond
to their request.

## Why use mCaptcha?

-   [x] **Free software, privacy focused**
-   [x] **Seamless UX** - No more annoying CAPTCHAs!
-   [x] **No tracking:** Our CAPTCHA routes are cookie free!
-   [x] **IP address independent:** your users are behind a NAT? We got you covered!
-   [x] **Resistant to replay attacks:** proof-of-work configurations have
        short lifetimes (30s) and can be used only once. If a user submits a
        PoW to an already used configuration or an expired one, their proof
        will be rejected.

## Demo

## Client-side widget:

mCaptcha's UX is super silent, solving CAPTCHAs have never been more
easier. One click and you are on your way.
To observe mCaptcha in action, open dev tools and
monitor console and network activity.

1. [Link to widget](https://demo.mcaptcha.org/widget/?sitekey=pHy0AktWyOKuxZDzFfoaewncWecCHo23)

2. [Video](https://github.com/mCaptcha/mCaptcha/blob/master/docs/res/widget-in-action.mp4?raw=true):

### Demo servers are available at:

-   https://demo.mcaptcha.org/
-   https://demo2.mcaptcha.org/ (runs on a Raspberry Pi!)

> Core functionality is working but it's still very much
> work-in-progress. Since we don't have a stable release yet, hosted
> demo servers might be a few versions behind `master`. Please check footer for
> build commit.

Feel free to provide bogus information while signing up (project under
development, database frequently wiped).

### Self-hosted:

Clone the repo and run the following from the root of the repo:

```bash
git clone https://github.com/mCaptcha/mCaptcha.git
docker-compose up -d
```

After the containers are up, visit [http://localhost:7000](http://localhost:7000) and login with the default credentials:

-   username: aaronsw
-   password: password

It takes a while to build the image so please be patient :)

See [DEPLOYMENT.md](./docs/DEPLOYMENT.md) for detailed alternate deployment
methods.

## Development:

See [HACKING.md](./docs/HACKING.md)

## Deployment:

See [DEPLOYMENT.md](./docs/DEPLOYMENT.md)

## Configuration:

See [CONFIGURATION.md](./docs/CONFIGURATION.md)

## Funding

### NLnet

<div align="center">
	<img
		height="150px"
		alt="NLnet NGIZero logo"
		src="./docs/third-party/NGIZero-green.hex.svg"
	/>
</div>

<br />

2023 development is funded through the [NGI0 Entrust
Fund](https://nlnet.nl/entrust), via [NLnet](https://nlnet.nl/). Please
see [here](https://nlnet.nl/project/mCaptcha/) for more details.
