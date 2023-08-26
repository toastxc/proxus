# proxus
Fast and easy TCP/UDP reverse proxy


## Usage
Proxus was designed and intended for binding virtual machine and hypervisor ports together but the config allows for binding both ports and IP addresses together.

**examples can be found [here](https://github.com/toastxc/proxus/blob/main/conf.toml)**

```bash
# create config file 
proxus config.toml
```

## Installing 
```bash
# download
git clone https://github.com/toastxc/proxus.git
cd proxus
# compile to release
cargo r -r
# install 
sudo cp /target/release/proxus /bin/
```

## Compatibility
Like all software made by myself I am willing to help bring support for windows if requested, but Windows support is not tested for nor is it a high priority.


## Performance
Proxus created a new async task for every connection, if these runtimes crash for whatever reason they will not crash other processes. Proxus's performance is only limited by the multithreaded performance of the server it is deployed on.
