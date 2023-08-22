FROM python:latest
WORKDIR /app
COPY . .
RUN pip install django
EXPOSE 8080
CMD ["python", "manage.py", "runserver", "0.0.0.0:8080"]
