---
title: "deploying a nostr relay"
date: 2022-11-29T18:00:27-05:00
---

I've been looking in to [nostr](https://github.com/nostr-protocol/nostr) recently, and I think it's pretty neato. I wanted to participate in `#nostrnovember` more than I have up to this point though. In an effort to do _something_ before the end of the month, I decided to actually create a nostr presence. After minutes [of research](https://github.com/aljazceru/awesome-nostr#clients), I created a key pair using [Astral](https://astral.ninja/). Very quickly, though, I realized that I wanted to run my own relay to at least store my own...messages?...to be a little more "censorship resistant." It didn't look hard, especially with all [the existing implementations](https://github.com/aljazceru/awesome-nostr#implementations) the community has created already.

In an effort to give back, here are some instructions on how to setup a nostr relay node using [nostr-rs-relay](https://github.com/scsibug/nostr-rs-relay). This should work on any new Ubuntu box. I used an existing one I have on Digital Ocean.

## Step 1: Dependencies

There are a some dependencies we need to install. These include [rustup](https://rustup.rs/) (not explained here) and [certbot](https://certbot.eff.org/) (also not explained here).

Next, there's some stuff we can install with `apt`. So do that. We'll also open up some ports in the firewall (`ufw`), and turn that beast on.

```shell
apt update && apt upgrade -y
apt install build-essential sqlite3 libsqlite3-dev libssl-dev pkg-config nginx -y
ufw allow 'Nginx Full'
ufw allow 'OpenSSH'
ufw enable
```

Now that the dependencies are installed, we should clone the source code, build the relay, and install it on our system.

```shell
git clone https://github.com/scsibug/nostr-rs-relay.git && cd nostr-rs-relay
cargo build --release # go take a nap. this will take a bit.
install target/release/nostr-rs-relay /usr/local/bin
```

Create a user for our relay to eventually run as, and switch to that user to handle the config file.

```shell
adduser --disabled-login nostr
su nostr
```

While we are our `nostr` user, let's download the example config and change it to suit our liking. In my case, I updated the URL and name, changed the address to just listen on `127.0.0.1` since we will use Nginx as a proxy, and added my public key as a whitelisted pubkey since I was making this only as an authoritative backup for my events.

```shell
# as nostr
cd $HOME
wget https://raw.githubusercontent.com/scsibug/nostr-rs-relay/master/config.toml
vim config.toml # change this file as you want
exit # go back to root
```

Now that the relay is configured, let's make sure it runs automatically when the server restart and such and configure it to be accessible using Nginx as a proxy.

First, let's create our `systemd` file so `systemd` will manage the uptime of our relay.

```shell
# as root
cat /lib/systemd/system/nostr-relay.service <<EOF
[Unit]
Description=Nostr Relay
After=network.target

[Service]
Type=simple
User=nostr
WorkingDirectory=/home/nostr
Environment=RUST_LOG=info,nostr_rs_relay=info
ExecStart=/usr/local/bin/nostr-rs-relay
Restart=on-failure

[Install]
WantedBy=multi-user.target
EOF
```

Nothing wild going on there. Next we create our Nginx site config. This currently doesn't have any SSL stuff in it. We will later use `certbot` to handle that for us. You'll want to change this for your particular setup. It mostly would just need to be changed to match the domain you plan on deploying this on.

```shell
cat /etc/nginx/sites-available/nostr-relay <<'EOF'
map $http_upgrade $connection_upgrade {
    default upgrade;
    '' close;
}

upstream websocket {
    server 127.0.0.1:8080;
}

server {
    listen 80;
    server_name nostr.domainname.com;
    location / {
        proxy_pass http://websocket;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection $connection_upgrade;
        proxy_set_header Host $host;
        proxy_set_header X-Forwarded-For $remote_addr;
    }
}
EOF
```

Now we need to "turn on" the nostr-relay site and restart all the things.

```shell
ln -sf /etc/nginx/sites-available/nostr-relay /etc/nginx/sites-enabled/nostr-relay
systemctl start nostr-relay.service
systemctl enable nostr-relay.service
systemctl restart nginx.service
```

Everything should be running now. Update your DNS provider to point your subdomain (assuming it's a subdomain), to your server. For me, this was a simple A record for `nostr` that points to my Digital Ocean box.

Now that the DNS is setup and (hopefully) propagated, run `certbox` to setup the TLS certificates.

```shell
# update DNS before running certbot
certbot --nginx -d nostr.domainname.com
```

Hopefully, I didn't forget a step and you now have a beautiful new Nostr relay working.

If you like what you read, please contribute back to the many excellent projects [building on top of nostr](https://github.com/aljazceru/awesome-nostr), donate to [OpenSats](https://opensats.org/), or leave me a tip at [sats4.tips](https://sats4.tips/w3irdrobot), all of which make their way to OpenSats.
