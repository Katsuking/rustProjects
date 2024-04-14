FROM busybox:1.36.1-uclibc as busybox

FROM surrealdb/surrealdb:latest
COPY --from=busybox /bin/sh /bin/sh
COPY --from=busybox /bin/mkdir /bin/mkdir
COPY --from=busybox /bin/cat /bin/cat
COPY --from=busybox /bin/chmod /bin/chmod
ENTRYPOINT /surreal start --log debug --user $DB_USER --pass $DB_PASSWORD file://database.db