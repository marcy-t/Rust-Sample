FROM rust:latest 
ENV USER=marcy
WORKDIR /src
RUN apt-get update

