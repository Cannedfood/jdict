from 'alpine:latest'


RUN mkdir -p /srv/jdict
COPY res/  /srv/jdict/res
COPY dist/ /srv/jdict/dist
COPY server/bin/release/server /srv/jdict/


ENV JDICT_DIST_DIR=/srv/jdict/dist/
ENV JDICT_RES_DIR=/srv/jdict/res/
ENV JDICT_PORT=8080

EXPOSE 8080/tcp
CMD /srv/jdict/server
