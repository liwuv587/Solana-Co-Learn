FROM rust:1.77.1
ARG ANCHOR_VERSION=0.29.0
ARG SOLANA_VERSION=1.17.0
WORKDIR /solana
RUN sed -i 's/deb.debian.org/mirrors.ustc.edu.cn/g' /etc/apt/sources.list.d/debian.sources  
RUN apt-get update && apt-get upgrade -y && apt-get install -y pkg-config build-essential libudev-dev   
RUN sh -c "$(curl -sSfL https://release.solana.com/${SOLANA_VERSION}/install)"  
RUN cargo install --git https://github.com/coral-xyz/anchor --tag v${ANCHOR_VERSION} avm --locked  
RUN avm install ${ANCHOR_VERSION}  && \
avm use ${ANCHOR_VERSION}

# docker build -t liwuv587/anchor:0.29.0 .
# docker push liwuv587/anchor:0.29.0