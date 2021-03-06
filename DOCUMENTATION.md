# Config file of D-SH format

```
---
download_dir: "dwn"
applications_dir: "app"
dockerfile:
  from: "ubuntu:18.04"
  tag: "d-base-image:v1.0.0"
# This line is optional. By default use /tmp
tmp_dir: "~/.tmp"
```

# Support installation format

D-SH support install file from:
 * `.deb`
 * `.tar.gz`
 * `.tgz`
 * `.tar.bz2`
 * `.tar.xz`
 * native linux distribution repository

# How add application

To add a new application, add file in `applications_dir` folder. Filename (without `.yml`
extension) is the name of application.

In this file, we need some properties:
```
---
download_filename: "...."
url: "..."
image_name: "..."
dependencies:
  - ...
  - ...
cmd_line: "...."
cmd_line_args:
 - ...
 - ...
interactive: true | false
skip_redownload: true | false
```

## Why list of dependencies ?

Linux have dependencies manager like `apt-get`, `rpm`, `yast`... But if you
download a package (here a `.deb` file). You have many versions.

For example for :
```
$ dpkg -I atom.deb
 new Debian package, version 2.0.
 size 89683126 bytes: control archive=550 bytes.
     644 bytes,    12 lines      control              
 Package: atom
 Version: 1.34.0
 Depends: git, gconf2, gconf-service, libgtk-3-0 (>= 3.9.10), libudev0 | libudev1, libgcrypt11 | libgcrypt20, libnotify4, libxtst6, libnss3 (>= 2:3.22), python, gvfs-bin, xdg-utils, libcap2, libx11-xcb1, libxss1, libasound2 (>= 1.0.16), libxkbfile1, libcurl3 | libcurl4
```

You can see `libudev0` or `libudev1`. But `libudev0` is not available for **Ubuntu 18.04**.

Another issue is sometime, a dependencies missing in `.deb` file.

For that, we must list all dependencies that really need.

# Hack D-SH

## Change Ubuntu version or image base

By default, when you initialize D-SH, the file `Dockerfile.hbs` is created.

If you want change Ubuntu version, edit `~/.d-sh/config.yml` file and
change line `from: "ubuntu:18.04"`.

A last file is entrypoint script `entrypoint.sh`.

## D-SH behind proxy

To allow Ubuntu image to download dependencies, edit `Dockerfile.hbs`
file and add just:
```
ENV ALL_PROXY socks://xx.xx.xx.xx:3128/
ENV FTP_PROXY http://xx.xx.xx.xx:3128/
ENV HTTPS_PROXY http://xx.xx.xx.xx:3128/
ENV HTTP_PROXY http://xx.xx.xx.xx:3128/
ENV all_proxy socks://xx.xx.xx.xx:3128/
ENV ftp_proxy http://xx.xx.xx.xx:3128/
ENV http_proxy http://xx.xx.xx.xx:3128/
ENV https_proxy http://xx.xx.xx.xx:3128/

RUN echo 'Acquire::http::Proxy "http://xx.xx.xx.xx:3128";' >> /etc/apt/apt.conf && \
    echo 'Acquire::https::Proxy "http://xx.xx.xx.xx:3128";' >> /etc/apt/apt.conf
```

Rebuild base image and all applications images

## Dockerfile template data

 - `{{dependencies}}` list of dependencies of all applications,
 - `{{dockerfile_from}}` value from config file,
 - `{{#if dockerfile_base}}` if current build docker base image,
 - `{{application_filename}}` filename of binary of application downloaded,
 - `(ends_width application_filename  ".tar.bz2")` check if application filename end with.

## Add a new command

To add new command, you must know `rust`. After, create module in `src/command`.

## In nutshell

### Docker image entrypoint

The entrypoint of base image point to `entrypoint.sh` script. This file create
user, group and home folder of user that launch container. This script set also
owner of home folder. Finally, script run application with a substitute user and
group ID.

### Home mapping

Full home user that launch application are mount in container home user's folder.

### Add tests

For more informations, read [TESTS](TESTS.md) file.
