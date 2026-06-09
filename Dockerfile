# Build frontend
FROM node:20 AS frontend
WORKDIR /app/frontend
COPY frontend/package*.json ./
RUN npm install
COPY frontend/ .
RUN npm run build

# Build backend
FROM rust:1.95 AS backend
WORKDIR /app
COPY . .
COPY --from=frontend /app/frontend/dist ./frontend/dist
RUN cargo build --release

# Final image
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=backend /app/target/release/blind-spot .
COPY --from=backend /app/frontend/dist ./frontend/dist
EXPOSE 3000
CMD ["./blind-spot"]
