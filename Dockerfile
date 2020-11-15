FROM rust:1.47 as builder

WORKDIR /usr/src/web

COPY . .

RUN cargo install --path .
RUN cargo install diesel_cli

RUN chmod +x entrypoint.sh

ENTRYPOINT ["./entrypoint.sh"]