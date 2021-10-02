FROM rust:1.55 as build

RUN USER=root cargo new --bin webapp
WORKDIR /webapp
COPY apps/webapp/Cargo.* ./
RUN cargo build --release
RUN rm src/*.rs

COPY . .

# RUN rm /webapp/target/release/deps/webapp*
RUN cargo build --release


FROM gcr.io/distroless/cc-debian10


COPY --from=build /webapp/target/release/lexmex-webapp /usr/local/bin/lexmex-webapp

CMD ["lexmex-webapp"]
