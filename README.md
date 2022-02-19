# Random Donald Trump's tweet
![Build](https://github.com/nbittich/maga-rs/actions/workflows/rust.yml/badge.svg)

🚀 blazing fast!! 🚀 barely unsafe !! 🔥 generates random tweets in 10e8 ns!! 🚀
## How to
- `cargo install maga`
- `maga # prints one tweet to the terminal`
- `maga --count=2 # prints 2 tweets to the terminal`
- `maga-serve # small rest endpoint -- Default port is 8000`
- `maga-serve --port=<port> # change default port`
    - `http://localhost:8000?count=1`


## Docker - Released (Recommended)
- `docker build -t relz -f Dockerfile.stable .`
- `docker run -it relz maga -c 1`
- `docker run -it -p 9999:9999 relz maga-serve -p 9999`

## Docker - Dev
- `docker build -t dev -f Dockerfile.dev .`
- `docker run -it dev maga -c 1`
- `docker run -it -p 9999:9999 dev maga-serve -p 9999`