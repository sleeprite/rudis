FROM rust:latest
WORKDIR /rudis
RUN cargo install --git https://github.com/sleeprite/rudis.git
EXPOSE 6379
ENTRYPOINT ["rudis-server"]
