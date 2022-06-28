+++
date = 2022-05-12T17:43:02-04:00
title = "Bisq API Rust Project Setup"
description = "Basic setup for a Rust project talking to the Bisq API"
tags = ["rust", "rustlang", "api"]
categories = ["programming"]
+++

Bisq has a new API in the [v1.9.1 release](https://github.com/bisq-network/bisq/releases/tag/v1.9.1). They have provided [some really good documentation](https://github.com/bisq-network/bisq-api-reference) on how to work with it, including some sample code in Python and Java. However, for those of us who like good programming languages, we have to figure it out on our own. Here's a basic setup to start working with it in Rust.

> Completed code for this post can be found [on my GitHub page](https://github.com/w3irdrobot/bisq-api-rust).


## New Project

Let's create the new project and install some dependencies we will need. I will be using [`cargo-edit`](https://crates.io/crates/cargo-edit) below. So make sure to install it if you run the below commands and they don't work.

```shell
# `cargo install cargo-edit` if necessary
# create the new project
cargo new bisq-api
# move into the project directory
cd bisq-api
# add the deps we'll need
cargo add tokio --features rt-multi-thread
cargo add prost tonic
cargo add --build tonic-build
```

Here we are bringing in `tokio` for async support, `tonic` and `prost` for gRPC support with code generation (since the API is a gRPC API), and `tonic-build` to help automate some of the gRPC protobuf client creation.

Now we need to download the proto files used by Bisq to create its server. We can use these same files to generate a Rust client for us to use. Let's plop them into a `proto` directory.

```shell
mkdir proto
(
    cd proto
    curl -LO https://raw.githubusercontent.com/bisq-network/bisq/v1.9.1/proto/src/main/proto/grpc.proto
    curl -LO https://raw.githubusercontent.com/bisq-network/bisq/v1.9.1/proto/src/main/proto/pb.proto
)
```

This creates our proto directory and downloads those proto files into there. We will then use `tonic-build` as part of the build process to create the client for us auto-magically. Place the following in a file called `build.rs` in the root of the project.

```rust
fn main() {
    tonic_build::compile_protos("proto/grpc.proto")
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
}
```

For more information on this file, check out [the cargo book](https://doc.rust-lang.org/cargo/reference/build-scripts.html).


## Get Bisq Version

Now that's all set up, let's start working on our actual `main.rs` file. First, let's setup our proto files using `tonic`.

```rust
mod bisq {
    tonic::include_proto!("io.bisq.protobuffer");
}
```

This can go at the very top of the file. It's a macro that includes the necessary generated Rust code into the `bisq` module. This means our generated gRPC client code is available inside `bisq::`.

To start off simple, let's query the current Bisq version through the API. Let's setup our imports and our `main` function:

```rust
use bisq::get_version_client::GetVersionClient;
use bisq::GetVersionRequest;
use std::time::Duration;
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
}
```

This is the basic setup for a simple API call. We bring in the `GetVersionClient` struct, as well as the structs we will need for creating the request we will send.

I'll talk through how to setup the API locally a bit later, but for now just assume there will be an API running on port `http://localhost:9998`. We will also accept a password through an environment variable called `BISQ_PASSWORD` that will eventually be passed in the request we make to the API as authentication. Add the following to the top of the `main` function.

```rust
let password = std::env::var("BISQ_PASSWORD")?;

let mut client = GetVersionClient::connect("http://localhost:9998").await?;
```

Here we are getting the API password from the environment and then setting up a client using the `connect` function. Notice we are using `.await` at the end of creating the client. This is because the client `tonic` generates use async Rust.

Next let's create our request struct.

```rust
let mut req = Request::new(GetVersionRequest {});
req.set_timeout(Duration::from_secs(5));
```

We wrap the `GetVersionRequest`, which takes no attributes, in a `tonic::Request` and then set the timeout on the request to five seconds.

Now add authentication to the request. Based on the Python and Java code examples in the documentation, we can see the authentcation is handled by adding a `password` field to the metadata of the request and setting that equal to the password we received from the environment.

```rust
let metadata = req.metadata_mut();
metadata.insert("password", password.parse()?);
```

Now we just need to send the request! Let's do that and output the current version to our terminal.

```rust
let response = client.get_version(req).await?.into_inner();
println!("bisq version: {}", response.version);

Ok(())
```

## Run the API

The Bisq API is actually a separate process from the Bisq desktop client. From my understanding, you can either run the Bisq API process **or** the Bisq desktop client at one time, not both. So assuming the Bisq desktop is not running, let's download and get the `v1.9.1` Bisq API process running locally.

```shell
# move to Downloads to put everything in there
cd ~/Downloads
# import the gpg public key used to sign the API zip file
curl -L https://github.com/bisq-network/bisq/releases/download/v1.9.1/29CDFD3B.asc | gpg --import
# download the Bisq API daemon zip
curl -LO https://github.com/bisq-network/bisq/releases/download/v1.9.1/bisq-daemon-1.9.1.zip
# download the signature file
curl -LO https://github.com/bisq-network/bisq/releases/download/v1.9.1/bisq-daemon-1.9.1.zip.asc
# verify the signature
gpg --verify bisq-daemon-1.9.1.zip.asc
# unzip the daemon archive and move into the outputted directory
unzip bisq-daemon-1.9.1.zip
cd bisq-daemon-1.9.1
# RUN THE API!
java -jar daemon.jar --apiPassword=thisisastrongpassword
```

Notice that when we run the API, we are passing a password using the `apiPassword` flag. You can make this whatever you want. We will need to pass this to our Rust program when we run it.

Bisq will output a bunch of stuff. Give it a little time to run. It does alot of stuff to get itself up and running, just like the desktop does.


## Run the program

Well, let's see how we did. Set the password we passed when we started the API as an environment variable called `BISQ_PASSWORD` and then run the program!

```shell
export BISQ_PASSWORD="thisisastrongpassword"
cargo run
```

On my computer, this is the output.

```shell
bisq-api on  master [?] is 📦 v0.1.0 via 🦀 v1.60.0
₿ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 8.92s
     Running `target/debug/bisq-api`
bisq version: 1.9.1
```

Woooooo! It works. Super cool...okay let's doing something a little more interesting.


## Get all closed transactions

I think it would be more interesting to get all the closed trades. The process is pretty similar to getting the version. The one extra thing we need to do is unlock the wallet. It appears many of the things that interact with the wallet first require the wallet to be unlocked, which makes sense. One super cool thing is when we unlock it via the API, we can tell it how long we want it to be unlocked for, and Bisq will automatically lock the wallet for us. Thanks, Bisq!

First, add some imports to the top of our `main.rs` file.

```rust
use bisq::get_trades_request::Category;
use bisq::trades_client::TradesClient;
use bisq::wallets_client::WalletsClient;
use bisq::{GetTradesRequest, UnlockWalletRequest};
```

Next, add a line to pull in another environment variable representing the password to unlock the Bisq wallet.

```rust
let wallet_password = std::env::var("WALLET_PASSWORD")?;
```

To unlock the wallet, create a `WalletsClient` instance and call `unlock_wallet` on it.

```rust
let mut client = WalletsClient::connect("http://localhost:9998").await?;

let mut req = Request::new(UnlockWalletRequest {
    password: wallet_password,
    timeout: 10,
});
req.set_timeout(Duration::from_secs(5));

let metadata = req.metadata_mut();
metadata.insert("password", password.parse()?);

client.unlock_wallet(req).await?.into_inner();
```

Now that our wallet is unlocked, we can make a `TradesClient` and request to get all of our trades.

```rust
let mut client = TradesClient::connect("http://localhost:9998").await?;

let mut req = Request::new(GetTradesRequest {
    category: Category::Closed.into(),
});
req.set_timeout(Duration::from_secs(5));

let metadata = req.metadata_mut();
metadata.insert("password", password.parse()?);

let response = client.get_trades(req).await?.into_inner();
println!("trades: {:?}", response.trades);
```

That's it! Add an environment variable for our wallet password, and run the program again. You should see some ugly output. Peruse through it, and you'll see all the info is there.

```shell
export WALLET_PASSWORD="mywalletpasswordissecure"
cargo run
```

Here's some output:

```shell
bisq-api on  master [?] is 📦 v0.1.0 via 🦀 v1.60.0
₿ cargo run
   Finished dev [unoptimized + debuginfo] target(s) in 6.64s
     Running `target/debug/bisq-api`
bisq version: 1.9.1
trades: [TradeInfo { offer: ...
```


## Wrap up

That's really it. Not too much to it. I want to thank the Bisq team for all their hardwork. If you enjoy the project and want to give back, check out the [contributor checklist](https://bisq.wiki/Contributor_checklist) and get involved. They're a good group over there. If you're interested in creating an integration with the API, check out the [API documentation](https://bisq-network.github.io/slate/).
