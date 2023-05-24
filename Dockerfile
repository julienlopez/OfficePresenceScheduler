FROM rust:latest as build

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli

WORKDIR /usr/src/OfficePresenceScheduler
COPY . .

RUN cd frontend && trunk build --release
RUN cargo build --release

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/src/OfficePresenceScheduler/target/release/backend /usr/local/bin/backend
COPY --from=build /usr/src/OfficePresenceScheduler/frontend/dist /usr/local/bin/dist

WORKDIR /usr/local/bin
CMD ["backend"]
