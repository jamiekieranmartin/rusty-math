FROM rustlang/rust:nightly

ARG PORT

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_ENV=prod

WORKDIR /app
COPY . .

RUN cargo test
RUN cargo build --release

CMD ROCKET_PORT=$PORT ./target/release/rusty-math
