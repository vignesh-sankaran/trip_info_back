FROM postgres:9.6.1
ENV POSTGRES_USER user_db 
ENV POSTGRES_PASSWORD user_db
ENV POSTGRES_DB user_info
COPY init.sql /docker-entrypoint-initdb.d/