# Letters and Numbers

## Build
```bash
> sbt
$sbt> clean
$sbt> test
$sbt> assembly
$sbt> exit
> cd ui
./ui> cargo build --features=config-dev
./ui> trunk build index-dev.html
./ui> cd ..
> docker-compose build
> docker-compose up -d 
```

## Release
```bash
> cd ui
./ui> cargo build --features=config-prod
./ui> trunk build --release --public-url lettersandnumbers index-prod.html
./ui> cd ..
> docker buildx bake --push --set *.platform=linux/amd64,linux/arm
```
