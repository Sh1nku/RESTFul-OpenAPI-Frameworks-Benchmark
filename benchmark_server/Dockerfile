FROM python:3.8-alpine

COPY ./ /app
WORKDIR /app
RUN apk --no-cache add zip
RUN pip install --no-cache-dir -r requirements.txt
CMD ["./main.py"]