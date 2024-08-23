FROM oven/bun

WORKDIR /usr/site/app

COPY . .

RUN bun install

RUN bun run build

CMD ["bun","build/index.js"]