ARG NODE_VERSION=22.6.0

FROM node:${NODE_VERSION} AS amplify-website
ENV NODE_ENV production

WORKDIR /usr/src/app

COPY ./package.json .

RUN npm i

COPY . .

CMD node build/index.js