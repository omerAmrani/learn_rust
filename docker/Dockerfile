FROM rust:latest

WORKDIR /usr/app
# Copy the Rust project files to the working directory
COPY .. .

# Build the Rust app
RUN cargo build --release

EXPOSE 3000


# Set the command to run the Rust app
CMD cargo run
