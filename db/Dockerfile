FROM postgres:9.6.0
WORKDIR  /docker-entrypoint-initdb.d/
COPY initdb.sql /docker-entrypoint-initdb.d/initdb.sql
ENV POSTGRES_DB=user_info
ENV POSTGRES_USER=tripinfo
ENV POSTGRESS_PASSWORD=trip_info_asdfqwer
