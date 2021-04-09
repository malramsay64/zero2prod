
#
# Find project dependencies
#

# Use the latest stable rust release as our base image
FROM lukemathwalker/cargo-chef as planner
# Switch our working directory to app
WORKDIR app
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

#
# Build Project Dependencies
#

FROM lukemathwalker/cargo-chef as cacher
WORKDIR app
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies
RUN cargo chef cook --release --recipe-path recipe.json

#
# Compile our application
#

FROM rust AS builder
WORKDIR app

# Copy all the compiled dependencies from the previous step
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

# Copy the source code
COPY . .
# Use the offline mode of SQLX to verify database connections
ENV SQLX_OFFLINE true

# Build our binary, making use of the cached dependencies
RUN cargo build --release --bin zero2prod

#
# Get a minimal application ready for deployment
#

# Here we can use a minimal runtime
FROM debian:buster-slim AS runtime
WORKDIR app

# Install OpenSSL which is dynamically linked by our dependencies
RUN apt-get update -y \
     && apt-get install -y --no-install-recommends openssl \
     # Clean up
     && apt-get autoeremove -y \
     && apt-get clean -y \
     && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration
ENV APP_ENVIRONMENT production

# When docker run is launched run the binary
ENTRYPOINT ["./zero2prod"]
