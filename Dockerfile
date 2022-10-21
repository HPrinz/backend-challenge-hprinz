FROM rust:1.64 as builder

COPY . . 

RUN cargo test
RUN cargo build --release

# build final slim image
FROM debian:buster-slim

COPY --from=builder /target/release/backend-challenge-hprinz .

CMD ["./backend-challenge-hprinz"]
