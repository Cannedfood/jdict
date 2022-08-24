from 'alpine:latest'


RUN mkdir -p /srv/jdict
COPY server/JMdict.xml /srv/jdict/JMdict.xml
COPY dist/ /srv/jdict/dist
COPY server/bin/release/server /srv/jdict/


ENV JDICT_DIST_DIR=/srv/jdict/dist/
ENV JDICT_XML=/srv/jdict/JMdict.xml
ENV JDICT_PORT=8080

EXPOSE 8080/tcp
CMD /srv/jdict/server
