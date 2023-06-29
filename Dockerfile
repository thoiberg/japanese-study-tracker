############################
# Backend build
############################
FROM rust:1.70 AS backend-build

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
FROM rust:1.70

RUN mkdir /app
WORKDIR /app

COPY --from=backend-build /app/target/release/ /app/
COPY --from=frontend-build /app/dist/ /app/dist

CMD ["./japanese-study-tracker-backend"]
