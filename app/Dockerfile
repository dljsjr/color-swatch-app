FROM mhart/alpine-node:14 as builder

WORKDIR /app
COPY ccolors-app/ .
RUN npm ci
RUN npm run build

FROM mhart/alpine-node:slim-14 as runtime
WORKDIR /app
COPY --from=builder /app .

CMD ["node", "./build"]