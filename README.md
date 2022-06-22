# TOTP

## Introduction

This Rust application was created to save few clicks when generating time-based one-time passwords 
using Google Authenticator.

## Usage

First you need to initialize your configuration file.

```shell
./totp config append --key your_key --secret your_secret
```

Configuration file will be saved in:

| OS      | PATH                                                    |
|---------|---------------------------------------------------------|
| Linux   | /home/pfigwer/.config/totp/config/totp.tom              |
| macOS   | /Users/pfigwer/Library/Preferences/totp/config/totp.tom |
| Windows | C:\Users\pfigwer\AppData\Roaming\totp\config\totp.toml  |

Now you can generate you one time password using:

```shell
./totp generate
your_key: 787412
```
