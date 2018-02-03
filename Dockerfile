FROM ubuntu:16.04
WORKDIR /opt
RUN apt-get update \
    && apt-get -y install build-essential \
    && apt-get -y install tesseract-ocr \
    && apt-get -y install libssl-dev
RUN curl https://sh.rustup.rs -sSf | sh
ADD target/release/i2eye /opt
ARG LOGVL=info
ENV RUST_LOG=$LOGVL
ENTRYPOINT [ "/opt/i2eye", "-a", "0.0.0.0:3890" ]