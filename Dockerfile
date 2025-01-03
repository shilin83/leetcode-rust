FROM rust:1.83.0-alpine3.21

WORKDIR /app

COPY . .

CMD ["cargo", "test"]
