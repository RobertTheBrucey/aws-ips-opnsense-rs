# aws-ips-opnsense-rs
Pulls AWS IPs and serves them in an OPNsense friendly format

https://hub.docker.com/r/robertthebruce/aws-ips-opnsense-rs

3. Accessing the API
Once the program is running, you can interact with the API by sending HTTP GET requests to http://localhost:3030. The server will return a list of IP prefixes (one per line).

For example:
curl http://localhost:3030
This will return a list of AWS IP prefixes filtered according to the region filter you've specified.

Region Filtering via URL Parameter
The program supports filtering AWS IP prefixes by region using the region URL parameter. This parameter allows for wildcards (*) and exclusions (using the ! symbol). For example, to retrieve IP ranges from all regions starting with eu-, you can make a request like:
http://localhost:3030?region=eu-*

To exclude regions starting with cn-, use:
http://localhost:3030?region=!cn-*

License
This project is licensed under the MIT License - see the LICENSE file for details.
