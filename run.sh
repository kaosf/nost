while :; do
  inotifywait -q content.txt -e close_write
  if [ -z "$(cat content.txt)" ]; then
    echo 'Empty!'
    continue
  fi
  if [ -f content-before.txt ] && diff -q content.txt content-before.txt; then
    echo 'Same!'
    continue
  fi
  node index.js
  cp content.txt content-before.txt
done
