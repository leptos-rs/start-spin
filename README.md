<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Leptos Spin SSR Starter Template

This is a template for use with the [Leptos][leptos] web framework and the [Spin][spin-install] WASI platform.

[Click here to see the deployed version of this starter](https://spin-leptos-ssr-49mccqzz.fermyon.app).

## Creating Your Repo

First, ensure that you have Rust 'nightly' with both the `wasm` and `wasm32-wasi` targets, along with `cargo-generate`
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

For the full [Spin CLI install instructions see here][spin-install].

<br/>

To generate your own repo from this starter template, run
```sh
cargo generate --git https://github.com/leptos-rs/start-spin
```

Then

```sh
cd {{project-name}}
```

to go to your newly created project.

<br/>

Feel free to explore the project structure, but the best place to start with your application code is in `src/pages/home.rs`.


Additionally, Cargo.toml may need updating as new versions of the dependencies are released, especially if things are not working after a `cargo update`.

## Running & Developing Your Project

Running
```sh
spin watch
```

will build and run your server as well as recompile your code after making changes.

Using
```sh
spin build --up
```
will also run your app locally.


## Release and Deployment

To deploy your app to [Fermyon Cloud signup here first][spin-signup]. For more information on [Ferymon Cloud see here][spin-cloud-info].

After you have your Fermyon cloud account, running
```sh
spin build
```
will build your application for release and

```sh
spin deploy
```
will publish your app to Fermyon cloud.

If you would prefer to deploy your app on own machine or in a VM with Docker rather than on Fermyon Cloud, see [the Spin in Docker setup instructions here](https://www.fermyon.com/blog/spin-in-docker). 


[leptos]: https://github.com/leptos-rs/leptos
[spin-install]: https://developer.fermyon.com/spin/v2/install
[spin-signup]: https://cloud.fermyon.com/
[spin-cloud-info]: https://www.fermyon.com/cloud
