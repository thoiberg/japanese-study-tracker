############################
# Backend build
############################
FROM rust:1.71-alpine3.18 AS backend-build

RUN apk add pkgconfig openssl openssl-dev musl musl-dev

RUN mkdir /app
WORKDIR /app

COPY backend/ /app/

RUN cargo build --release

############################
# Frontend build
############################
FROM node:20.2.0-bullseye-slim AS frontend-build

RUN mkdir /app
WORKDIR /app

COPY frontend/ /app/

RUN npm install
RUN npm run build-only

############################
# Executable
############################
FROM alpine:3.18

RUN apk add libc6-compat

RUN mkdir /app
WORKDIR /app

COPY --from=backend-build /app/target/release/japanese-study-tracker-backend /app/
COPY --from=frontend-build /app/dist/ /app/dist

CMD ["./japanese-study-tracker-backend"]
