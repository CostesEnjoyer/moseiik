# Base image
FROM rust:latest

# Install dependencies
RUN apt update && apt install -y git wget unzip

# Set the working directory
WORKDIR /home/eii/QLOG

# Clone git repository
RUN git clone https://github.com/CostesEnjoyer/moseiik.git

# Set the working directory
WORKDIR /home/eii/QLOG/moseiik

# Download and unzip test images
RUN mkdir moseiik_test_images \
&& cd moseiik_test_images \
&& wget https://nasext-vaader.insa-rennes.fr/ietr-vaader/moseiik_test_images.zip -P . \
&& unzip moseiik_test_images.zip \
&& rm moseiik_test_images.zip \
&& mv output.png ground-truth-kit.png

# Run tests
ENTRYPOINT [ "cargo", "test", "--release", "--" ]

