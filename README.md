# proxus
Fast and easy layer 3 reverse proxy



## Usage
Proxus was designed and intended for binding virtual machine and hypervisor ports together but the config allows for binding both ports and IP addresses together.

```toml
# In this example the SSH port of a virtual machine is binded to the hypervisor on port 5000 
[[data]]
a1 = "192.168.100.2:22"
a2 = "192.168.1.5:5000"

# in this example the service cockpit.socket is binded to port 80 - this requires sudo
[[data]]
a1 = "localhost:9090"
a2 = "localhost:80"
```

## Compatibility
Like all software made by myself I am willing to help bring support for windows if requested, but Windows support is not tested for nor is it a high priority.


## Performance
Proxus created a new async task for every connection, if these runtimes crash for whatever reason they will not crash other processes.
