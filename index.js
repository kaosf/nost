import { finishEvent, getPublicKey, nip19, SimplePool } from "nostr-tools";
import "websocket-polyfill";
import { readFileSync } from "fs";

const privkey = nip19.decode(readFileSync("./nsec.txt", "utf-8").trim()).data;
const relays = readFileSync("./relays.txt", "utf-8")
  .split("\n")
  .filter((x) => !x.match(/^#/))
  .filter((x) => !(x === ""));
if (relays.length === 0) {
  console.log("No relay!!");
  process.exit(0);
}

const content = readFileSync("./.content-current.txt", "utf-8").trim();
if (content.match(/^\s*$/)) {
  console.log("Empty!!");
  process.exit(0);
}

const pool = new SimplePool();

const ev = finishEvent(
  {
    kind: 1,
    created_at: Math.floor(Date.now() / 1000),
    tags: [],
    content,
  },
  privkey
);

await Promise.allSettled(pool.publish(relays, ev));
pool.close(relays);
