# Rudis Docker Image

This is the Dockerfile of the Docker image for Rudis.

## Image Variants

### `ghcr.io/lunar-landing/rudis:latest`

This is the latest released rusdis Docker image.

### `ghcr.io/lunar-landing/rudis:<version>`

Rudis Docker image will be builded on each release, [view the package page](https://github.com/lunar-landing/rudis/pkgs/container/rudis).

## How to use this image

### Base

```sh
docker run --rm -p 6379:6379 -p 7379:7379 ghcr.io/lunar-landing/rudis:latest
```

### With Args

You can add all supported args at the end, like

```sh
docker run --rm -p 6379:8848 -p 7379:7379 ghcr.io/lunar-landing/rudis:latest --port 8848
```

### Handle Data

Rudis Docker image's default `WORKDIR` is /rudis, but you can change it with arg `--dir /some/other/path`

So bind /rudis to handle data

```sh
docker run --rm -p 6379:6379 -v /some/path/to/save/data:/rudis ghcr.io/lunar-landing/rudis:latest --save 60/1
```

You can use a config file like this

```sh
touch ./config.properties
docker run --rm -p 6379:6379 -v ./:/rudis ghcr.io/lunar-landing/rudis:latest --config config.properties
```
