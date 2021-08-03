FROM ubuntu

RUN apt-get update && \
    apt-get install -y tor

RUN echo "HiddenServiceDir /var/lib/tor/onion_service/" >> /etc/tor/torrc && \
    echo "HiddenServicePort 1234 127.0.0.1:1234" >> /etc/tor/torrc

RUN mkdir data && mkdir circuits
COPY target/release/torzkp .
COPY data/provingkey.params data/
COPY circuits/* circuits/

ENTRYPOINT service tor restart && cat /var/lib/tor/onion_service/hostname && RUST_MIN_STACK=8388608 ONION_SERVICE=1 ./torzkp