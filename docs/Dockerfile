FROM rust:1-slim AS builder

RUN CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse cargo install mdbook --locked
RUN mkdir /opt/docs/

COPY ./src /opt/docs/src
COPY ./book.toml /opt/docs/

WORKDIR /opt/docs/
RUN mdbook build

FROM nginx:stable-alpine
COPY --from=builder /opt/docs/book /usr/share/nginx/html

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]