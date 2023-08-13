
FROM ubuntu:latest

# 必要なパッケージをインストールするための準備
RUN apt-get update && \
    apt-get upgrade -y
RUN apt-get install -y \
        build-essential \
        clang \
        curl \
        git \
        wget \
        gawk \
        texinfo \
        bison \
        autoconf \
        cmake automake autotools-dev \
        python3 python3-pip \
        libmpc-dev libmpfr-dev libgmp-dev flex texinfo gperf libtool patchutils bc zlib1g-dev libexpat-dev ninja-build libglib2.0-dev

RUN apt-get install -y \
        lld \
        llvm \
        qemu-system-riscv32


# RISC-VのGCCツールチェーンをビルドするためのスクリプト等をダウンロード
RUN git clone --recursive https://github.com/riscv/riscv-gnu-toolchain

# RISC-V 32ビット用のツールチェーンをビルド
WORKDIR /riscv-gnu-toolchain
RUN ./configure --prefix=/opt/riscv32 --with-arch=rv32i --with-abi=ilp32 && \
    make

# 環境変数を設定して、ツールチェーンが見つかるようにする
ENV PATH="/opt/riscv32/bin:${PATH}"

RUN mkdir -p work && \
    cd work && \
    curl -LO https://github.com/qemu/qemu/raw/v8.0.4/pc-bios/opensbi-riscv32-generic-fw_dynamic.bin

WORKDIR /work



