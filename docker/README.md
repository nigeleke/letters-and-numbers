# Letters and Numbers

## Build
```bash
> sbt
$sbt> clean
$sbt> test
$sbt> assembly
$sbt> exit
> cd ui
./ui> trunk build index-dev.html
./ui> cd ..
> docker-compose build
> docker-compose up -d 
>
```

## Release
```bash
> cd ui
./ui> trunk build --release index-prod.html
./ui> cd ..
> docker-compose build
> docker push nigeleke/lettersandnumbers-api
> docker push nigeleke/lettersandnumbers-ui
```
