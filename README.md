<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Leptos Spin SSR Starter Template

This is a template for use with the [Leptos][leptos] web framework and the [Spin][spin-install] WASI platform.

## Creating Your Repo

First, ensure that you have Rust 'nightly with both the `wasm` and `wasm32-wasi` targets, along with `cargo-generate`
- `rustup toolchain install nightly --allow-downgrade`
- `rustup target add wasm32-unknown-unknown`
- `rustup target add wasm32-wasi`
- `cargo install cargo-generate`

If you don't have `spin` installed you can install it with

```bash
curl -fsSL https://developer.fermyon.com/downloads/install.sh | bash
```

Once you have the Spin CLI tool downloaded, we recommend putting the binary into a folder already on your path, eg

```sh
sudo mv spin /usr/local/bin/
```

for the full [Spin CLI install instructions see here.][spin-install]


Then to generate your own repo from this starter template, run
```sh
cargo generate rust-wasm-wasi/spin-leptos-ssr-starter
```

to generate a new project template. Then

```sh
cd {{project-name}}
```

to go to your newly created project.

Feel free to explore the project structure, but the best place to start with your application code is in `src/pages/home.rs`.


Additionally, Cargo.toml may need updating as new versions of the dependencies are released, especially if things are not working after a `cargo update`.

## Running / Developing Your Project

Run
```sh
spin watch
```

this will build and run your server, and recompile after you make changes to your code.


## Release and Deployment

Running
```sh
spin build
```
will build your application for release and

```sh
spin deploy
```
will publish your app to Fermyon cloud.


[leptos]: https://github.com/leptos-rs/leptos
[spin-install]: https://developer.fermyon.com/spin/v2/install