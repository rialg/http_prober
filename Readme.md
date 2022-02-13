# WIP
### Simple HTTP prober in Rust

Configuration parameters available:

* `period`: How many seconds should it wait between probes
* `hostname`: Target server hostname
* `port`: Target web service TCP port
* `URI`: Resource to check
* `threshold`: Amount of successful probes to consider the status of the service as healthy

TODO:
1. Add unit tests
1. Add documentation comments
1. Create OCI container image spec
