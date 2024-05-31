FROM rust:latest

ARG DATABASE_URL

ENV DATABASE_URL=${DATABASE_URL}

RUN apt-get update && apt-get install -y libpq-dev

WORKDIR /clynelish-backend

COPY . .

RUN cargo build --release

CMD ["./target/release/clynelish-backend"]