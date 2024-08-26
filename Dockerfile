############################
# Backend build
############################
FROM rust:1.80-alpine3.19 AS backend-build

RUN apk add pkgconfig openssl openssl-dev musl musl-dev

RUN mkdir /app
WORKDIR /app

COPY backend/ /app/

RUN cargo build --release

############################
# Frontend build
############################
FROM node:22.7.0 AS frontend-build

RUN mkdir /app
WORKDIR /app

COPY frontend/ /app/

RUN npm install
RUN npm run build-only

############################
# Executable
############################
FROM alpine:3.19

RUN apk add libc6-compat

RUN mkdir /app
WORKDIR /app

COPY --from=backend-build /app/target/release/japanese-study-tracker-backend /app/
COPY --from=frontend-build /app/dist/ /app/dist

CMD ["./japanese-study-tracker-backend"]
