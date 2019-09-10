FROM williamofockham/sandbox:nightly-2019-07-03
MAINTAINER Ben Simms <ben@bensimms.moe>

WORKDIR /opt/netbricks
RUN git clone https://github.com/williamofockham/NetBricks /opt/netbricks
RUN make build

WORKDIR /opt/netbricks_test
ADD Cargo.toml ./Cargo.toml
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm src/*.rs
RUN rm target/release/deps/netbricks_test*

ADD src/ src/
RUN cargo build --release

ADD test.toml run.sh ./
