ARG GLEAM_VERSION=v1.14.0

FROM ghcr.io/gleam-lang/gleam:${GLEAM_VERSION}-erlang-alpine AS builder

COPY ./shared /build/shared
COPY ./client /build/client
COPY ./server /build/server

RUN cd /build/shared && gleam deps download
RUN cd /build/client && gleam deps download
RUN cd /build/server && gleam deps download

RUN cd /build/client \
  && gleam add --dev lustre_dev_tools \
  && gleam run -m lustre/dev build --minify --outdir=../server/priv/static

RUN cd /build/server \
  && gleam export erlang-shipment

FROM ghcr.io/gleam-lang/gleam:${GLEAM_VERSION}-erlang-alpine

COPY --from=builder /build/server/build/erlang-shipment /app

WORKDIR /app
RUN echo -e '#!/bin/sh\nexec ./entrypoint.sh "$@"' > ./start.sh \
  && chmod +x ./start.sh

ENV HOST=0.0.0.0
ENV PORT=8080

EXPOSE $PORT

CMD ["./start.sh", "run"]
