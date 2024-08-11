# Rudis Docker Image

This is the Dockerfile of the Docker image for Rudis.

## Image Variants

### `ghcr.io/sleeprite/rudis:latest`

This is the latest released rusdis Docker image.

### `ghcr.io/sleeprite/rudis:<version>`

Rudis Docker image will be builded on each release, [view the package page](https://github.com/sleeprite/rudis/pkgs/container/rudis).

## How to use this image

### Base

```sh
docker run -p 6379:6379 ghcr.io/sleeprite/rudis:latest
```

### With Args

You can add all supported args at the end, like

```sh
docker run -p 6379:8848 ghcr.io/sleeprite/rudis:latest --port 8848
```

### Handle Data

Rudis Docker image's default `WORKDIR` is /rudis, but you can change it with arg `--dir /some/other/path`

So bind /rudis to handle data

```sh
docker run -p 6379:6379 -v /some/path/to/save/data:/rudis ghcr.io/sleeprite/rudis:latest --save 60/1
```

You can use a config file like this

```sh
touch ./config.properties
docker run -p 6379:6379 -v ./:/rudis ghcr.io/sleeprite/rudis:latest --config config.properties
```
