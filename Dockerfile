# Step 1: Build the application
FROM rust:1.73 AS builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the source code
COPY . .

# Build the application in release mode
RUN cargo build --release

# Step 2: Create a minimal runtime image
FROM debian:buster-slim

# Install necessary system dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/aws-ips-opnsense-rs- /app/app

# Expose the application port
EXPOSE 3030

# Command to run the application
CMD ["./app"]

