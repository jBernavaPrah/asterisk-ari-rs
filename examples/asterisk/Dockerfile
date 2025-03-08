FROM alpine:latest

# Update package lists and install dependencies
RUN apk add --no-cache \
    asterisk \
    asterisk-sample-config \
    asterisk-sounds-en \
    bash \
    vim

# Copy configuration files
COPY ./configs/* /etc/asterisk/

# Change the owner of the configuration files
RUN chown -R asterisk:asterisk /etc/asterisk

# Copy entrypoint script
COPY --chmod=755  ./docker-entrypoint.sh /docker-entrypoint.sh

ENTRYPOINT ["/docker-entrypoint.sh"]
CMD	["asterisk", "-fp"]