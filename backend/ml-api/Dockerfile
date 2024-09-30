FROM python:3.10

WORKDIR /usr/src/ml_api

COPY requirements.txt /usr/src/ml_api/requirements.txt

RUN pip install --no-cache-dir --upgrade -r /usr/src/ml_api/requirements.txt

COPY ./app /usr/src/ml_api/app

CMD ["fastapi", "run", "app/main.py", "--port", "80"]