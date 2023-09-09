This is mostly intended as a personal project for hosting my website. You're welcome to use the code as you see fit in your own projects. However, all files under `frontend/static` should be replaced with your own creation as they are of mixed ownership.

## Setup Dev

1. `rustup target add wasm32-unknown-unknown`
1. `cargo install trunk`
1. `cargo install cargo-watch`
1. `trunk build`
1. `cargo build`
1.  open another terminal
1. Terminal 1 `trunk watch`
1. Terminal 2 `cargo watch -x "run --bin server"`
1. If testing the server side rendering be sure to disable wasm in firefox. `javascript.options.wasm`
    - Bootstrap uses pure javascript and can be disabled with `javascript.enabled`