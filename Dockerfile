FROM rust:1.73.0

WORKDIR /app

COPY . /app

ENV ROCKET_ADDRESS=0.0.0.0

ENV ROCKET_PORT=8000

EXPOSE 8000

RUN cargo build

CMD [ "cargo", "run" ]