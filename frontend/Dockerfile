# Needed to specifically lock to this version due to an ETXTBUSY issue with ESBuild
# https://stackoverflow.com/questions/76461515/etxtbsy-error-when-installing-esbuild-in-docker-container
FROM node:20.2.0-bullseye-slim

RUN mkdir /app
WORKDIR /app

COPY package.json .
COPY package-lock.json .

RUN npm install

CMD ["npm", "run", "dev"]