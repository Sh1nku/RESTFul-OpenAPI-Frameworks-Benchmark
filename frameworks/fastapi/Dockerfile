FROM python:3.10
WORKDIR /app
COPY requirements.txt ./
RUN pip install --no-cache-dir --upgrade -r requirements.txt
COPY main.py ./

#
CMD ["uvicorn", "main:app", "--host", "0.0.0.0", "--port", "80", "--workers", "16", "--no-access-log", "--log-level=critical"]