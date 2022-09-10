FROM ubuntu
RUN mkdir /opt/demo
WORKDIR /opt/demo
RUN apt update
RUN apt install python3 pip -y
COPY . .
RUN python3 -m pip install -r requirements.txt
RUN chmod +x run.sh
EXPOSE 5000
ENTRYPOINT [ "/opt/demo/run.sh" ]