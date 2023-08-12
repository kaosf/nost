const relays = [
  // "wss://eden.nostr.land",
  // "wss://nos.lol",
  "wss://nostr-pub.wellorder.net",
  "wss://nostr-relay.nokotaro.com",
  // "wss://nostr.fmt.wiz.biz",
  // "wss://nostr.h3z.jp",
  "wss://nostr.holybea.com",
  "wss://nostr.orangepill.dev",
  // "wss://nostr.wine",
  "wss://nrelay.c-stellar.net",
  "wss://offchain.pub",
  "wss://relay-jp.nostr.wirednet.jp",
  "wss://relay.current.fyi",
  "wss://relay.damus.io",
  "wss://relay.nostr.band",
  "wss://relay.snort.social",
  "wss://yabu.me",
];

import { getEventHash, getSignature, nip19, SimplePool } from "nostr-tools";
import "websocket-polyfill";
import { readFileSync } from "fs";

const pubkey = "a6f1f450080b65ba75da8ac7328f91c94f8314b2cc4aa719c516852a29388f0b";
const privkey = nip19.decode(readFileSync("./nsec.txt", "utf-8").trim()).data;

const content = readFileSync("content.txt", "utf-8").trim();

const pool = new SimplePool();

const ev = {
  kind: 1,
  created_at: Math.floor(Date.now() / 1000),
  tags: [],
  content,
  pubkey
};
ev.id = getEventHash(ev);
ev.sig = getSignature(ev, privkey);

pool.publish(relays, ev);
