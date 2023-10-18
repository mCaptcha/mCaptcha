## 0.1.0(unreleased)

### Changed

-   2023-10-18: Environment variable names have changed, please see
    [CONFIGURATION.md](docs/CONFIGURATION.md) for the names of environment
    variables.
-   ([`7d0e4c6`](https://github.com/mCaptcha/mCaptcha/commit/7d0e4c6be4b0769921cda7681858ebe16ec9a07b)) Add `secret` parameter to token verification request payload(`/api/v1/pow/siteverify`) to mitigate a security issue that @gusted found:
    > ...A malicious user could grab the sitekey
    > and use that sitekey with mcaptcha to use it for their own server.
    > While they can now go abuse it for illegal stuff or other stuff.
    > You might decide, oh I don't want this! and terminate a legitimate
    > siteKey.
    > New request payload:
    ```json
    {
    	"secret": "<your-users-secret>", // found in /settings in the dashbaord
    	"token": "<token-presented-by-the-user>",
    	"key": "<your-sitekey>"
    }
    ```
-   ([`42544ec42`](https://github.com/mCaptcha/mCaptcha/commit/42544ec421e0c3ec4a8d132e6101ab4069bf0065)) Rename pow section in settings to captcha and add options to configure
