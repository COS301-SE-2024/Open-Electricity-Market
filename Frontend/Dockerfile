FROM node:18.18.0-alpine AS builder
LABEL authors="Christiaan George Strydom"
WORKDIR /Frontend
COPY package.json .

RUN npm i

COPY . .
CMD ["node", "build"]