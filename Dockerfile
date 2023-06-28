############################
# Backend build
############################
FROM rust:1.70 AS rust-build

RUN mkdir /app
WORKDIR /app

# re-write the index file to remove the http://localhost:5173 part from the code

RUN cargo build

############################
# Frontend build
############################
FROM node:20.2.0-bullseye-slim AS frontend-build

RUN mkdir /app
WORKDIR /app

# COPY frontend/package.json .
# COPY frontend/package-lock.json .
COPY frontend/ /app/

RUN npm install
RUN npm run build-only

############################
# Executable
############################
FROM rust:1.70

RUN mkdir /app
WORKDIR /app
COPY backend/ /app/

# copy executable from rust-build
COPY --from=frontend-build /app/dist/ /app/dist
# copy frontend code from frontend-build
# RUN the executable/"cargo run"
RUN cargo build
CMD ["cargo", "run"]
