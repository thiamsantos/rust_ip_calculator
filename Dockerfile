FROM rust:1.24

WORKDIR /usr/src/myapp
COPY . .

CMD ["cargo", "run"]
