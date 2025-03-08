# Asterisk Docker Container

Easily run Asterisk in docker container, in order to test the library.

**This is a testing docker, not for production use.**

## Usage
1) Update the [docker-compose.yml](docker-compose.yml) file, changing the NAT_ADDRESS with your local IP address.

2) Execute the docker compose:

    ```bash
    docker compose up -d  
    ```

3) Run the image in interactive mode and start asterisk in verbose CLI mode:
    ```
    docker compose exec -ti asterisk /bin/bash 
    asterisk -r
    ```
4) In your soft phone (e.g. Zoiper5 or MicroSIP) configure user `phone@localhost:5060` with password `mypassword` and 
   - Check if you are connected to the asterisk server
   - Dial extension 100 for the hello world example.

5) Run the example:
    ```
    cargo run --example monkeys-example
    ```
   - Dial extension 101 for the monkeys sound example, managed by the ARI client.


**NOTE:** Asterisk configs folder is based on [Asterisk getting started page](https://docs.asterisk.org/Getting-Started/Hello-World) with ARI setup added.
For additional info about ARI see [here](https://docs.asterisk.org/Configuration/Interfaces/Asterisk-REST-Interface-ARI/Getting-Started-with-ARI).


Once your asterisk is up & running you can run full examples like:
```
cargo run --example monkeys-example
```