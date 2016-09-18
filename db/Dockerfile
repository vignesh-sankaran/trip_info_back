FROM mongo:3.2.9
RUN apt-get update && apt-get install netcat -y
RUN mkdir /script
WORKDIR /script
ADD ./init.sh /script/
ADD ./initdb.js /script/
RUN chmod 755 init.sh
RUN ./init.sh
RUN rm -rf /tmp/mongodb-27017.sock
