# WIP
### Simple HTTP prober in Rust

Three configuration parameters available:

* `period`: How many seconds should it wait between probes
* `ip_and_port`: Target. To be replaced by URL and port
* `URI`: Resource to check
* `threshold`: Amount of successful probes to consider the status of the service as healthy

TODO:
1. Enable hostname (DNS) resolution
1. Add unit tests
1. Add documentation comments
1. Create OCI container image spec
