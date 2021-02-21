FROM rust:1.49 as builder

WORKDIR /build

COPY . .

RUN cargo build --release

ARG DATABASE_URL
ENV DATABASE_URL $DATABASE_URL

FROM debian:buster-slim

ARG APP=/usr/src/app

EXPOSE 80

ENV APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

RUN apt-get update -y \
    && apt-get install libpq-dev -y

COPY --from=builder /build/target/release/web ${APP}/web

COPY --from=builder /build/static ${APP}/static

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR $APP

CMD ["./web"]
