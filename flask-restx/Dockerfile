FROM python:3.8-alpine

COPY ./ /app
WORKDIR /app
RUN pip install --no-cache-dir -r requirements.txt
CMD ["gunicorn", "--bind", "0.0.0.0:80", "--workers", "16", "wsgi:app"]
