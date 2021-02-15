FROM rust:1.47 as builder

WORKDIR /build

COPY . .

RUN cargo build --release
RUN cargo install diesel_cli

ARG DATABASE_URL
ENV DATABASE_URL $DATABASE_URL

RUN diesel setup

FROM alpine:latest

ARG APP=/usr/src/app

EXPOSE 8080

ENV APP_USER=appuser

RUN addgroup -S $APP_USER \
    && adduser -S -g $APP_USER $APP_USER

COPY --from=builder /build/target/release/web ${APP}/web

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./web"]
