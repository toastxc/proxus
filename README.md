


![proxus-logo](https://github.com/user-attachments/assets/45b7ae96-ffa4-4da7-9cb5-bd30dd5b914b)


Fast and easy TCP/UDP reverse proxy
## Usage
Proxus was designed and intended for binding virtual machine and hypervisor ports together but the config allows for binding both ports and IP addresses together.

**examples can be found [here](https://github.com/toastxc/proxus/blob/main/conf.toml)**

```bash
# create config file 
proxus config.toml
```

## Installing
### Rust (recommended)
```bash
cargo install proxus
```


### Git
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
The performance impact of Proxus is negligible, and unlikely to be an issue on any system or workload
