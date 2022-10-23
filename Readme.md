# Backend Challenge for DigitalService

This is my solution for the [backend challenge of DigitalService](https://github.com/digitalservicebund/backend-challenge), written in Rust language.

## Run

Prerequisites:

- [Docker](https://www.docker.com/products/docker-desktop/) installed

To run the app

- run time docker image by executing `docker run -p 8080:8000 backend-challenge-hprinz`
- access the app at http://localhostl:8000

## Development

- Install rustup by following the steps on https://rustup.rs/
- execute `rustup default stable` 
- run the app with `cargo run`
- open http://127.0.0.1:8000

## Execute Tests

- run `cargo test`

## Possible improvements

- improve error handling
    - render error pages instead of causing panics in http controller
- add tests for unhappy cases (e.g. provoke errors)
- add html metadata (title, etc.)
- make dashboard _more_ pretty