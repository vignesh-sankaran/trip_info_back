sudo security delete-certificate -c localhost
rm -rf ./ssl/cert.pem ./ssl/key.pem ./ssl/dec.pem
openssl req -x509 -newkey rsa:4096 -keyout ./ssl/key.pem -out ./ssl/cert.pem -days 60 -nodes -subj "/C=AU/ST=Victoria/L=Melbourne/O=Ferndrop Pty Ltd/OU=org/CN=localhost" && \
openssl rsa -in ./ssl/key.pem -out ./ssl/dec.pem && \
sudo security add-trusted-cert -d -r trustRoot -k /Library/Keychains/System.keychain ./ssl/cert.pem
