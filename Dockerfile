FROM node:18.16.1
RUN apt-get update && apt-get -y install inotify-tools
WORKDIR /workspace/
COPY ["package.json", "package-lock.json", "/workspace/"]
RUN npm i
COPY ["run.sh", "index.js", "/workspace/"]
CMD bash run.sh
