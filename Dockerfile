FROM rust:latest

WORKDIR /app

# 1. Copy only the dependency configurations
COPY Cargo.toml Cargo.lock ./

# 2. Create a fake src/main.rs so cargo fetch doesn't complain
RUN mkdir src && echo "fn main() {}" > src/main.rs

# 3. Fetch the dependencies and cache them
RUN cargo fetch

# 4. Copy your REAL source code (this overwrites the fake main.rs)
COPY . .

# 5. Build the application for release
RUN cargo build --release

EXPOSE 3000

CMD ["cargo", "run", "--release"]