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

docker container run -d --restart=always \
-v $PWD/config:/workspace/config:ro \
-v $PWD/data:/workspace/data \
-u $UID \
-e RUST_LOG=info \
kaosf/nost:latest

nvim data/content.txt
```

Edit and `:w` to publish an event of kind 1.

The container detects your `WatchMask::MODIFY` event of `data/content.txt`, and post it to Nostr.

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

Lua version:

```lua
local function nost()
	if vim.fn.expand("%:p") ~= "/path/to/data/content.txt" then
		return
	end
	vim.cmd("w")
	vim.cmd("sleep 1")
	vim.cmd("goto 1")
	vim.cmd("d 9999")
	vim.cmd("redraw")
end
vim.keymap.set("n", "sn", nost, { silent = true })
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

## License

This project is licensed under the [GNU Affero General Public License v3.0](https://www.gnu.org/licenses/agpl-3.0.html) – see the [LICENSE](./LICENSE.txt) file for details.
