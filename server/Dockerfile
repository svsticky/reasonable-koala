FROM rust:1-slim AS BUILDER

RUN apt update -qq && apt install -y -qq --no-install-recommends \
    musl-tools

RUN rustup set profile minimal
RUN rustup target add x86_64-unknown-linux-musl

COPY ./database /opt/project/database
COPY ./wilford /opt/project/wilford
COPY ./Cargo.toml /opt/project/
COPY ./Cargo.lock /opt/project/

WORKDIR /opt/project
ENV RUSTFLAGS='-C link-args=-s'

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine
RUN apk add --no-cache ca-certificates
COPY --from=BUILDER /opt/project/target/x86_64-unknown-linux-musl/release/wilford /usr/local/bin/wilford

RUN chmod a+rx /usr/local/bin/*
RUN adduser wilford -s /bin/false -D -H
USER wilford

EXPOSE 8080
WORKDIR /usr/local/bin
ENTRYPOINT [ "/usr/local/bin/wilford" ]