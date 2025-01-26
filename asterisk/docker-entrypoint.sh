#!/bin/sh
# ... do some setup ...
# then run the CMD passed as command-line arguments

# Check if the env variable $NAT_ADDRESS is set
if [ -z "$NAT_ADDRESS" ]; then
  echo "The env variable NAT_ADDRESS is not set. Exiting..."
  exit 1
fi

# Change the {{external_signaling_address}} from the env variable $NAT_ADDRESS
sed -i "s/{{NAT_ADDRESS}}/$NAT_ADDRESS/g" /etc/asterisk/pjsip.conf

exec "$@"