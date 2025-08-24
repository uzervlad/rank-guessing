FROM oven/bun:1.2.20-alpine AS builder
WORKDIR /app

COPY . /app/

RUN apk update
RUN apk add python3 make

ENV DATABASE_URL="file:guess.db"

RUN bun i
RUN bun pm trust --all
RUN bun run build

# COPY --from=builder /app/build /app/build
# COPY start.sh /app/start.sh
EXPOSE 3000

ENTRYPOINT [ "sh", "/app/start.sh" ]