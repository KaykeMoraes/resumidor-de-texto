FROM rustlang/rust:nightly AS builder 
WORKDIR /usr/src/app 

COPY Cargo.toml Cargo.lock ./ 
COPY src ./src

RUN cargo build --release 

FROM debian:bookworm-slim 
WORKDIR /app

RUN apt-get update && apt-get install -y \
libssl-dev ca-certificates \ 
&& rm -rf /var/lib/apt/lists/* 

COPY --from=builder /usr/src/app/target/release/resumidor-de-pdf . 

COPY .env .env 

CMD ["./resumidor-de-pdf"] 
