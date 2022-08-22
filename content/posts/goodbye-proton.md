---
title: "goodbye, proton (the great migration)"
date: 2022-08-20T18:13:08-04:00
---

After using [Proton](https://proton.me/) for many years, I've made the decision to move away from their service and migrate my email to [Migadu](https://migadu.com).

I'm sorry, Proton. It's me. Not you.

## "But things were going so well!"

Don't get me wrong. Proton is a great service. I'd recommend it to anyone looking to move away from a service like Gmail or Yahoo Mail. I've been paying for it for years to be able to use a custom domain name and support their growth, and I've never really had any problems with them. I think the addition of a calendar and cloud storage as services are excellent touches. It's becoming a great all-in-one alternative to the Google ecosystem. Plus the very few times I've contacted support have always been quick and pleasant.

## We've just grown apart.

As I've grown as a developer and learned more about the technologies we use everyday, I've realized that the workflow I want for myself doesn't work well with something like Proton's email service. I want to manage the GPG keys locally myself and not have to upload them to Proton's servers to be able to use the same ones everywhere. I don't need the fancy (and slow) webmail UI that for some reason doesn't support the Dracula theme yet. I want to be able to use a single email client on my phone/computer that can talk to all my email accounts using IMAP (or POP3) without needing a "bridge." I don't need cloud storage. I use [Nextcloud](https://nextcloud.com/). I don't need the calendar. I *will be* moving that to Nextcloud. Nowadays, those are easy to host yourself or use [your Uncle Jim](https://diverter.hostyourown.tools/becoming-uncle-jim/).

However, hosting and maintaining your own email server that doesn't just get all its messages sent to spam is something I don't want to worry about. What I really want is a service that just does email, doesn't worry much about a webmail client, and focuses on providing an excellent email hosting platform that I can talk to using IMAP. I'm not afraid of something a little more developer-centric. I just want to be more in-control of what my email is doing.

## "What do they have that we don't have?"

I actually just heard of Migadu recently from [Drew Devault](https://drewdevault.com/) when inquiring about why he seems to dislike Protonmail. I had already been thinking about leaving their service anyways. When asked what he uses, he said Migadu. Despite his [butthurt opinion on Bitcoin](https://drewdevault.com/2021/04/26/Cryptocurrency-is-a-disaster.html)[^1] and unwillingness to accept Bitcoin at [sourcehut](https://sourcehut.org/), he seems like an intelligent person who knows technology. So I decided to check it out. The service seems to be just what I was looking for. Relatively minimalistic in terms of web interface. There is [a webmail UI](https://webmail.migadu.com/) for those who want that. Offers IMAP/SMTP access to the email servers. Unlimited domain names, which is great because I tend to hoard them. All I really needed.

The thing I'll miss the most is [automated encryption with a public key](https://www.migadu.com/procon/#not-encrypted). However, I think this will force me to come up with a better strategy for managing personal emails instead of it being done for me. We'll see. Also, their pricing page used to say they accepted "crypto," but you had to contact them. I wanted to pay in BTC; so I asked. They told me they don't do that anymore and removed it from the pricing page. So that's some shit.

## Getting over an ex

Once I made the decision to move, the next step was to migrate all my emails. I thought this would be as simple as outputting them from the Protonmail UI and then importing them into a mailbox in the Migadu admin area. Oh boy, was I naive. A few things got in the way. Protonmail doesn't have a good way in the UI to export emails. They have an import-export tool that's pretty neat. I tried it out, and it worked well, but I didn't end up using it. Also, Migadu doesn't have a way to import emails in their service. Instead, since they are built on being super standards-compliant, the expected way to import is to just use IMAP.

After digging [into the Migadu docs](https://www.migadu.com/guides/imapsync/), it turns out there is an excellent tool called [imapsync](https://imapsync.lamiral.info/) developed exactly for this usecase. So here's the new plan: download the Proton Mail Bridge, sign in, setup `imapsync`, configure the script, run the script. Below I'll list these in order with links for those following behind me.

1. Download [the Proton Mail Bridge](https://proton.me/mail/bridge).
1. Sign in to the bridge with the account you are moving emails from.
1. Install [imapsync](https://imapsync.lamiral.info/INSTALL.d/INSTALL.ANY.txt).

    ```shell
    # i was installing this on Pop!_OS. so i used the debian instructions
    # install dependencies
    ₿ sudo apt update && sudo apt install -y \
        libauthen-ntlm-perl \
        libcgi-pm-perl \
        libcrypt-openssl-rsa-perl \
        libdata-uniqid-perl \
        libencode-imaputf7-perl \
        libfile-copy-recursive-perl \
        libfile-tail-perl \
        libio-socket-inet6-perl \
        libio-socket-ssl-perl \
        libio-tee-perl \
        libhtml-parser-perl \
        libjson-webtoken-perl \
        libmail-imapclient-perl \
        libparse-recdescent-perl \
        libproc-processtable-perl \
        libmodule-scandeps-perl \
        libreadonly-perl \
        libregexp-common-perl \
        libsys-meminfo-perl \
        libterm-readkey-perl \
        libtest-mockobject-perl \
        libtest-pod-perl \
        libunicode-string-perl \
        liburi-perl \
        libwww-perl \
        libtest-nowarnings-perl \
        libtest-deep-perl \
        libtest-warn-perl \
        make \
        time \
        cpanminus

    # download imapsync
    ₿ wget -N https://raw.githubusercontent.com/imapsync/imapsync/master/imapsync
    ₿ chmod +x imapsync

    # perform a livetest to make sure everything is working correctly with the tool
    ₿ ./imapsync --testslive
    ```

1. Create a script for running `imapsync` (adapted from [the example script](https://github.com/imapsync/imapsync/blob/master/examples/imapsync_example.sh)).

    ```shell
    # update these to match your credentials
    ₿ export PROTON_USER=myuser PROTON_PASS=protonpass123 MIGADU_USER=mymigaduuser MIGADU_PASS=migadupass123
    ₿ cat <<EOF > run_imapsync.sh
    #!/bin/sh
    ./imapsync \\
        --host1 127.0.0.1 --port1 1143 --user1 $PROTON_USER --password1 $PROTON_PASS  \\
        --host2 imap.migadu.com --user2 $MIGADU_USER --password2 $MIGADU_PASS \\
        --automap "\$@"
    EOF

    ₿ chmod +x run_imapsync.sh
    ```

1. Test the script is able to connect to both hosts by kicking off a dry run.

    ```shell
    ₿ ./run_imapsync.sh --justfolders --dry
    ```

If this completes fine, then the hard work is done. `imapsync` takes alot of different flags to customize what the migration does, including mapping one folder to another, excluding folders and much more. For example, I added `--exclude 'Labels|All Mail|Trash'` to ignore many emails that would have been moved over that I don't want.

Once you run it with a dry run and all the output looks like it's what you want it to do, then remove the `--dry` flag and let it create the folders. If that runs fine and it looks right, then remove the `--justfolders` flag and run the large email migration. This could take a while; it all depends on how many emails you are wanting to migrate. It will give you logs showing you its progress so you can keep track. By default, it will also send you an email at the new account when it's complete.

So walk away and let it do its thing.

## There's other fish in the sea

Email has been around a long time and fortunately has many standards in use around the internet. These standards, when implemented correctly, allow alot of mobility for end-users just looking for a good email service. Migadu is that for me for now, but who knows? At least now I know migrating manually using these standards and the wonderful tooling available doesn't have to suck.

You don't have to be stuck in a horrible email relationship.

## Support `imapsync`

If you found `imapsync` helpful, consider [donating to the maintainer](https://imapsync.lamiral.info/#SUPPORT). He seems like a cool dude, and [the license is legit OSS](https://imapsync.lamiral.info/LICENSE).

[^1]: My stake is none of your fucking business.
