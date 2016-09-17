FROM mongo:3.2.9
RUN mkdir /script
WORKDIR /script
ADD ./init.sh /script/
RUN chmod 755 init.sh
RUN apt-get update && apt-get install netcat -y
RUN ./init.sh
