FROM williamofockham/sandbox:nightly-2019-07-03
MAINTAINER Ben Simms <ben@bensimms.moe>

WORKDIR /opt/netbricks
RUN git clone https://github.com/williamofockham/NetBricks /opt/netbricks
RUN make build

WORKDIR /opt/netbricks_test
ADD . /opt/netbricks_test
RUN cargo build
