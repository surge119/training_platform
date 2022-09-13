# sudo docker build -t zip_parser .
# sudo docker run -d -p 7293:7293 --rm -it zip_parser

FROM ubuntu:20.04

RUN apt-get update

RUN useradd -d /home/ctf/ -m -p ctf -s /bin/bash ctf
RUN apt-get install patchelf -y
RUN echo "ctf:ctf" | chpasswd

WORKDIR /home/ctf

COPY challenge/vuln .
COPY challenge/flag.txt .
COPY challenge/ynetd .

RUN chown -R root:root /home/ctf

USER ctf
EXPOSE 7293
CMD ./ynetd -p 7293 ./vuln
