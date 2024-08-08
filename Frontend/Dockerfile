FROM node

WORKDIR /usr/site/app

COPY . .

RUN npm i

RUN npm update

RUN npm run build

CMD ["node","build/index.js"]