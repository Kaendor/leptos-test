# Install

Rust needs to be installed on your system. You can install it by following the instructions on the [official website](https://www.rust-lang.org/tools/install).

Once rust is installed, you can follow the leptos documentation to install and run the project here : [leptos](https://book.leptos.dev/getting_started/index.html#hello-world-getting-set-up-for-leptos-csr-development)

Once everything is set up (rust toolchain, trunk and leptos) you can run this pet project with `trunk serve --open`

# Further down the road

If the need arise, we can use (Gloo)[https://gloo-rs.web.app/] to have an ergonomic API to the various browser APIs instead of having to use bindgen-wasm ourselves and be prone to mistakes.
