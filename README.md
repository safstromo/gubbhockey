# gubbhockey.com

This is the website (gubbhockey.com)[gubbhockey.com]
Used for booking and planning senior hockey training sessions.
It uses OIDC with Auth0 for simple login.


## Create a .env

DATABASE_URL="postgresql://develop:develop@localhost:5432/gubbhockey"
OAUTH_CLIENT_ID="clientid"
OAUTH_CLIENT_SECRET="secred"
OAUTH_AUTH_URL="https://url.com/authorize"
OAUTH_TOKEN_URL="https://url.com/oauth/token"
OAUTH_REDIRECT_URL="http://localhost:3000/auth"
OAUTH_LOGOUT_URL="https://url.com/logout"

## Running your project

```bash
cargo leptos watch
```

## Installing Additional Tools

By default, `cargo-leptos` uses `nightly` Rust. If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
3. ```cargo install cargo-leptos``` - Install cargo leptos
4. Run ```npm i -D daisyui@latest``` to install daisyui

## Compiling for Release

```bash
cargo leptos build --release
```

Will generate your server binary in target/server/release and your site package in target/site


## Quick deploy steps

### Building dockerfile and copy to server

1. Run ```cargo leptos build --release```

2. Add envstuff to dockerfile

3. run ```docker build -t gubbhockey```

4. export image ```docker save -o gubbhockey_image.tar gubbhockey:latest```

5. Move image to server with scp ```scp gubbhockey_image.tar user@ip:gubbhockey/```

6. Import image  ```docker load -i gubbhockey_image.tar```


## TODO:
- Remove hardcoded envs in dockerfile and import from .env, or just create a docker-compose instead.
