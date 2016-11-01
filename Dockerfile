FROM fedora
RUN dnf install -y lighttpd
RUN rm -rf /etc/lighttpd/ /var/www/*
COPY certs/server.pem /etc/lighttpd/
COPY certs/ca.crt /etc/lighttpd/
COPY lighttpd/ /etc/lighttpd/
COPY target/release/sticky /var/www/sticky/
RUN useradd --system --user-group --no-create-home --base-dir /var/www/sticky --shell /usr/sbin/nologin www-data
CMD lighttpd -Df /etc/lighttpd/lighttpd.conf
