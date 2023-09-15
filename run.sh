while :; do
  inotifywait -q data/content.txt -e close_write
  cp data/content.txt data/.content-current.txt
  if [ -z "$(cat data/.content-current.txt)" ]; then
    echo 'Empty!'
    continue
  fi
  if [ -f data/.content-before.txt ] && diff data/.content-current.txt data/.content-before.txt > /dev/null; then
    echo 'Same!'
    continue
  fi
  date
  echo "Before node index.js"
  node index.js
  date
  echo "After node index.js"
  cp data/.content-current.txt data/.content-before.txt
done
