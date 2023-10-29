# Nostr + Post = Nost

Only write and save on Neovim (or Vim). You can post an event to Nostr!

## Preparation

```sh
nvim config/nsec.txt # Input nsec1...
chmod 600 config/nsec.txt

nvim config/relays.txt # Input relay URLs
# Example
<<EOF
wss://nostr.example.com
wss://another-relay.example.com
# wss://invalid-relay.example.com
# The line starting with # is ignored.
wss://third-relay.example.com
EOF

# Version 2 style
cargo run

# or version 1 with Docker
docker container run -d --restart=always \
-v $PWD/config:/workspace/config:ro \
-v $PWD/data:/workspace/data \
-u $UID \
kaosf/nost:latest

nvim data/content.txt
```

Edit and `:w` to publish an event of kind 1.

The container detects your `close_write` event of `data/content.txt`, and post it to Nostr.

## My Vim script example

```vim
function! s:nost()
  if expand("%:p") != "/path/to/data/content.txt"
    return
  endif
  w
  sleep 1
  goto 1
  d 9999
  redraw
endfunction
nnoremap <silent> sn :call <SID>nost()<CR>
```

## Log viewer

```sh
# Version 1 style
docker run -d ... --name nost kaosf/nost:latest
# Run with --name option.

docker logs -f nost
```

## Development

Install Rust https://rustup.rs/

```sh
cargo build
```
