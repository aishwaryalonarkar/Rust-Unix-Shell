FROM ubuntu:20.04

WORKDIR /home
RUN apt-get update
RUN DEBIAN_FRONTEND=noninteractive TZ=Etc/UTC apt-get -y install tzdata
RUN apt-get install -y curl gcc
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN curl https://getmic.ro | sh -s -- -y && mv micro /usr/bin/

COPY src ./src
COPY dummy-files ./dummy-files
COPY Cargo.toml .
COPY Cargo.lock .
COPY init-fs.sh .
COPY input.json .
RUN chmod +x init-fs.sh
RUN ./init-fs.sh
RUN cargo build

CMD ["/bin/bash"]
