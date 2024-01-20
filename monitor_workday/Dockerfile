# Use an image with muslrust for building a static binary
FROM clux/muslrust:stable AS chef
USER root
RUN cargo install cargo-chef

WORKDIR /app
COPY ./shared_lib ./shared_lib

# Prepare the build, creating a recipe file
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Build only the dependencies to cache them
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

# Build application
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl --bin monitor_workday

# Build runtime image
FROM scratch AS final
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/monitor_workday /bin/monitor_workday

# Set the entrypoint to the application binary
ENTRYPOINT ["/bin/monitor_workday"]


