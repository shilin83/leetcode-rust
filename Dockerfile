FROM rust:1.84-alpine

WORKDIR /app

COPY . .

CMD ["cargo", "test"]
