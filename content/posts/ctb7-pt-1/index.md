---
title: "capture the bitcoin at tab7: pt 1"
date: 2025-11-03
---

Every year, a conference is put on in Atlanta, GA called The Atlanta Bitcoin Conference (TABconf). This year (2025) was TAB7. There is a competition put on each year called Capture the Bitcoin or CTB. The idea is to follow the clues to find a private key and use it to sweep the address holding a bunch of sats generously donated by some of the sponsors of the conference.

Finding the private key is no simple feat. It requires solving a long sequence of puzzles that will take a large amount of Bitcoin knowledge. It also requires writing code to solve some of the more difficult ones. Some teams have even developed special tooling for common scenarios that pop up each year.

I competed this year and got destroyed, but that's okay. I had fun working with my team. Plus it has lit a fire under my ass to really learn some of these concepts used to solve these puzzles. In an effort to keep myself accountable, I'm going to go through the CTB from this past year, documenting how to solve all the puzzles, as a way to grow as a Bitcoiner. Depending on how it goes, maybe I'll go back and do previous years where I've also gotten my ass kicked (a bit of a pattern here).

## the starting line

When you check in to TAB, you're given two things: a challenge coin and a wristband. The wristband is just a (silicone?) band with a bulge in the middle containing an NFC chip. Tapping the chip with your phone will reveal a URL to the schedule for TAB. The coin is a hard, thin, circular piece made of what I'm guessing is wood. The TAB7 logo is on one side; "PERMISSIONLESS INNOVATION" on the other. Around the edge is a series of numbers and letters.

{{< spread >}}
{{< img src="images/coin_logo.jpg" alt="TAB7 logo on challenge coin" height="300" >}}
{{< img src="images/coin_permissionless_innovation.jpg" alt="PERMISSIONLESS INNOVATION text on challenge coin" height="300" >}}
{{< img src="images/coin_edge.jpg" alt="Edge of challenge coin with numbers and letters" height="300" >}}
{{< /spread >}}

It's pretty well known at this point that the beginning of the CTB involves reading the edge of the coin as a transaction ID. This coin has `3e515635c3e319dc04dda918b278c4cdb4b745d59fa1d6993cfaadb86d52e480` on its edge. Inputting this into [our favorite block explorer](https://mempool.space/tx/3e515635c3e319dc04dda918b278c4cdb4b745d59fa1d6993cfaadb86d52e480?mode=details) reveals a transaction with two outputs, one utilizing the dreaded `OP_RETURN` op code to embed a string of text onto the blockchain. This year it reads, "Attacking Bitcoin one puzzle at a time. https://tabctb.com/0x07\".

## and so it begins...

Navigating to that URL takes us to the first page which talks about the challenge and shouts out the sponsors of the CTB. We are also provided with some transaction IDs we can use to track what outputs are still available to sweep. There is usually one large pot of funds that requires solving the whole CTB and some smaller pots along the way you can nab. No use wasting time on outputs that aren't available anymore!

At the bottom of the page, we get our next clue:

> If you're ready to continue, check your wristband for the next appended url path to this website.

This is another riddle that has been used in past years. The NFC chip in the wristband actually has multiple pieces of data on it. Using an app like [NFC Tools](https://play.google.com/store/apps/details?id=com.wakdev.wdnfc) allows you to see all that data on the chip.

{{< img src="wristband_data.png" alt="Wristband with NFC chip" height="500" >}}

You can see our first record is the TABconf schedule, but there is another record! It's a URL path of `/wristybusiness`. Let's [add that](https://tabctb.com/0x07/wristybusiness) to the URL we got earlier.

## the giggler has attacked!

Here, we get the backstory. Every year has a theme, and this year we will be saving Bitcoin (again). We have a few options at the bottom. Obviously, being the heroic bitcoiners we are, we'll go with the "Step up and Save Bitcoin!" option.

Clicking on that button brings us to another page with more info. This is an important page to read as it gives us some hints on important workshops to attend for those competing in the CTB, tells us how to progress (append to the URL path), and of course reminds us to be considerate of the events going on at the same time.

Lastly, it tells us where the next clue is: the TABconf poster. At the bottom of the TABconf poster is a barcode.

{{< img src="poster_barcode.jpg" alt="Barcode on TABconf poster" height="400" >}}

Scanning this, which was not easy and involved what might have been some sketchy websites, gives us this binary: `0111101010110111`. This appears to be two bytes of binary, so let's convert it to hex: `7AB7`. Now, let's append that to our URL and see [where that takes us](https://tabctb.com/0x07/wristybusiness/lore/7AB7)!

## the road to the giggler

We land on a page with a map of Georgia, showing a winding path from TABconf to the Giggler from Jekyll Island. Clicking the button at the bottom to follow the map takes us to a page informing us we have arrived at a wizard's lab.

This is where we will leave it for now. Tune in next time to check out the secret door in the wizard's lab!
