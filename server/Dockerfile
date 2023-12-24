# Base image with cargo-chef for caching dependencies
FROM lukemathwalker/cargo-chef:latest-rust-latest AS chef
WORKDIR /app

# Prepare the build, creating a recipe file
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Build only the dependencies to cache them
FROM chef AS cacher
COPY --from=chef /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Final build stage with cargo-watch for development
FROM rust:latest AS development
WORKDIR /app

# Copy only the cached dependencies
COPY --from=cacher /usr/local/cargo /usr/local/cargo

# Install cargo-watch for live reloading
RUN cargo install cargo-watch

# Expose the necessary port
EXPOSE 8000

# Command to run the application using cargo-watch for live reloading
# Note: The actual source code will be mounted, not copied
CMD ["cargo", "watch", "-x", "run"]

