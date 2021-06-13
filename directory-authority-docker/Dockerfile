FROM ubuntu

RUN mkdir data && mkdir circuits
COPY target/release/torzkp .
COPY data/verifyingkey.params data/
COPY circuits/* circuits/

EXPOSE 1234

ENTRYPOINT DIRECTORY_AUTHORITY=1 ./torzkp