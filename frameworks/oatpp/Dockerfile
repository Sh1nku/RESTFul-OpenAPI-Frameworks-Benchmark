FROM ubuntu:20.04

WORKDIR /app/build
ENV PYTHONUNBUFFERED=1
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    python3 \
    python3-pip \
    ninja-build \
    cmake

RUN pip3 install --no-cache --upgrade pip setuptools conan==1.*

COPY conanfile.txt ../
RUN conan profile new default --detect && conan profile update settings.compiler.libcxx=libstdc++11 default
RUN conan install .. --build=missing

COPY CMakeLists.txt ../
COPY src/ ../src
RUN cmake -DCMAKE_BUILD_TYPE=Release -G Ninja ..
RUN ninja

CMD ["bin/App"]
