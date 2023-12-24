FROM postgres:15

COPY *.sql /docker-entrypoint-initdb.d/
COPY *.csv /docker-entrypoint-initdb.d/
RUN chmod uga+r /docker-entrypoint-initdb.d/*

