# build
FROM rust:1.67 as build
RUN USER=root cargo new --bin senvy
WORKDIR /senvy
RUN rm -rf /src

# root Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# server
RUN cargo init server
COPY ./server/Cargo.toml /senvy/server/Cargo.toml

# cli - not required for server just placing a dummy empty project
RUN cargo init cli

# commmon
RUN cargo init --lib senvy_common
COPY ./senvy_common/Cargo.toml ./senvy_common/Cargo.toml
RUN rm senvy_common/src/*.rs
COPY ./senvy_common/src ./senvy_common/src

RUN cargo build --release -p senvy
RUN rm server/src/*.rs
COPY ./server/src ./server/src
RUN rm ./target/release/deps/senvy*
RUN cargo build --release

# run
FROM rust:1.67
COPY --from=build /senvy/target/release/senvy .
VOLUME data
EXPOSE 8080
CMD ["./senvy"]
