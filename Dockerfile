from rust:1-stretch


WORKDIR /usr/src/trader
COPY ./ /usr/src/trader/
RUN cargo build --release
RUN mkdir /app
RUN cp /usr/src/trader/target/release/alpaca-bot-trader /app
WORKDIR /app
CMD ./alpaca-bot-trader