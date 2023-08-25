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

nvim relays.txt # Input relay URLs
# Example
<<EOF
wss://nostr.example.com
wss://another-relay.example.com
# wss://invalid-relay.example.com
# The line starting with # is ignored.
wss://third-relay.example.com
EOF

touch content.txt
bash run.sh
# Keep it running.

# In another terminal:
nvim content.txt
```

Edit and `:w` to publish an event of kind 1.

## My Vim script example

```vim
function! s:nost()
  if @% != "content.txt"
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

## Want to do

- Fix an error after `pool.publish(relays, ev);`
