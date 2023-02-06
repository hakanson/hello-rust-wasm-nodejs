FROM amazonlinux:latest
USER root
RUN yum update -y
RUN yum install -y tar which gzip gcc openssl openssl-devel perl git
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH=/root/.cargo/bin:$PATH
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN yum install -y rust-src
RUN curl -fsSL https://rpm.nodesource.com/setup_16.x | bash -
RUN yum install -y nodejs
CMD ["/bin/bash"]