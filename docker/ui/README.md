# Letters and Numbers - Rust implementation

Environments are:

* dev
* prod

Requires `api` running in `url` defined by `Config.default().api_url`.

## Develop

```trunk serve index-dev.html```

Requires `api` running and exposed in `http://localhost:8088/`

## Release

```trunk build --release index-prod.html```

Requires `api` running and exposed in `http://lettersandnumbers.pi.local/`
