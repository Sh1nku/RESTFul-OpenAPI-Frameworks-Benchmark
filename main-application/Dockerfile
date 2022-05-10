FROM python:3.8-alpine

COPY ./ /app
WORKDIR /app
RUN apk --no-cache add zip
RUN pip install --no-cache-dir -r requirements.txt
RUN wget https://github.com/codesenberg/bombardier/releases/download/v1.2.5/bombardier-linux-amd64
RUN chmod +x bombardier-linux-amd64
CMD ["./main.py"]
