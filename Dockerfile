FROM rust:1.91-alpine3.20 AS builder

# Dependências essenciais de build no Alpine
RUN apk add --no-cache musl-dev build-base

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Compilar diretamente no Alpine (já usa musl)
RUN cargo build --release

FROM alpine:3.20

WORKDIR /app

# Copiar binário compilado no Alpine (musl)
COPY --from=builder /usr/src/app/target/release/resumidor-de-pdf .

COPY .env .env

CMD ["./resumidor-de-pdf"]
