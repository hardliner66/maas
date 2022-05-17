# MAAS (Mutex-As-A-Service)

I honestly don't know why I made this, but here it is.

Here are some docs:
[Api docs](openapi/maas.md)

Api docs were generated with this: https://mermade.github.io/widdershins/ConvertingFilesBasicCLI.html

## Running with docker
```sh
docker pull hardliner66/maas
docker run --rm -p 80:8000 --name maas hardliner66/maas
```

## Running local docker build
```sh
docker build -t maas .
docker run --rm -p 80:8000 --name maas maas
```

## Heroku Instance
https://mutex-as-a-service.herokuapp.com/
