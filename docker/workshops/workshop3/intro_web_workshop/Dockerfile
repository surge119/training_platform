FROM python:3.9

WORKDIR /app
RUN apt update -y 
RUN apt install iputils-ping -y
COPY . /app

RUN pip install --no-cache-dir --upgrade -r /app/requirements.txt
USER www-data
CMD ["gunicorn", "-w", "8","--bind", "0.0.0.0:80", "app:app"]