# gleam

```sh
cd ./shared
gleam build
cd ../client
gleam run -m lustre/dev build --minify --outdir=../server/priv/static
cd ../server
gleam run
```
