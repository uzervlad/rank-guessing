FROM oven/bun:1.2.22-alpine AS builder
WORKDIR /app

COPY web/ /app/

RUN bun i
RUN bun pm trust --all
RUN bun run build

EXPOSE 3000

ENTRYPOINT [ "bun", "run", "build/index.js" ]