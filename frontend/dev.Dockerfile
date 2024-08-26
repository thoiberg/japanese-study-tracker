FROM node:22.7.0

RUN mkdir /app
WORKDIR /app

COPY package.json .
COPY package-lock.json .

RUN npm install

CMD ["./node_modules/vite/bin/vite.js", "--host", "0.0.0.0"]