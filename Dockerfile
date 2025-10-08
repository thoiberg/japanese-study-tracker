############################
# Backend build
############################
FROM rust:1.90-alpine3.22 AS backend-build

RUN apk add pkgconfig openssl openssl-dev musl musl-dev

RUN mkdir /app
WORKDIR /app

COPY backend/ /app/

RUN cargo build --release

############################
# Executable
############################
FROM alpine:3.22

RUN apk add libc6-compat

RUN mkdir /app
WORKDIR /app

COPY --from=backend-build /app/target/release/japanese-study-tracker-backend /app/
COPY --from=backend-build /app/dist /app/dist

CMD ["./japanese-study-tracker-backend"]
