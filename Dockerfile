FROM ubuntu:16.04
RUN apt-get update \
    && apt-get -y install build-essential \
    && apt-get -y install tesseract-ocr
