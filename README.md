# shokuzai

Expiry date list of my food

![index image](./img/index.png)

## Usage

First, you have to install SQLite. According to [Rocket repository](https://github.com/SergioBenitez/Rocket/tree/master/examples/todo), for example:

- OS X: `brew install sqlite`
- Debian/Ubuntu: `apt-get install libsqlite3-dev`

Then, clone this repository and run Rust program.
If you don't have **nightly** Rust, please install it with `rustup install nightly`.

```bash
git clone https://github.com/yammmt/shokuzai.git
cd shokuzai
rustup override set nightly
cargo run # or `cargo run --release`
```

You can access your site by accessing `http://localhost:8000`.

## Links

- [Rocket todo example](https://github.com/SergioBenitez/Rocket/tree/master/examples/todo)
    - This app is based on this example :bow:
- CSS framework [Bulma](https://bulma.io/)
