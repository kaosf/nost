while :; do
  inotifywait -q content.txt -e close_write
  cp content.txt .content-current.txt
  if [ -z "$(cat .content-current.txt)" ]; then
    echo 'Empty!'
    continue
  fi
  if [ -f .content-before.txt ] && diff .content-current.txt .content-before.txt > /dev/null; then
    echo 'Same!'
    continue
  fi
  date
  echo "Before node index.js"
  node index.js
  date
  echo "After node index.js"
  cp .content-current.txt .content-before.txt
done
