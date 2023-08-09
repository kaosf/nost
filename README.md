# Nostr + Post = Nost

- Neovim
- inotifywait
- asdf

```sh
asdf plugin add nodejs https://github.com/asdf-vm/asdf-nodejs.git
asdf install
npm i

nvim nsec.txt # Input nsec1...
chmod 600 nsec.txt

touch content.txt
bash run.sh
# Keep it running.

# In another terminal:
nvim content.txt
```

Edit and `:w` to publish an event of kind 1.

## Want to do

- Fix hard coded public key
- Fix an error after `pool.publish(relays, ev);`
- Enable to costomize `relays`
