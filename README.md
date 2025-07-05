# MacViz - MAC Address Visualizer

## 🔍 Overview

MacViz is a handy command-line tool that helps you visualize and identify devices on your network by matching MAC addresses with friendly device names. It takes nmap's XML output and compares it against a JSON database of known devices to give you a clear table of IPs, MACs, and device names.

## ✨ Features

- Parses nmap XML output to extract IP and MAC addresses
- Matches MAC addresses against a customizable device database
- Displays results in a beautiful, formatted table
- Simple JSON configuration for device definitions
- Lightweight and fast

## 🚀 Installation

### Prerequisites
- Rust toolchain (install via [rustup](https://rustup.rs/))
- nmap (for network scanning)

### Build from source
```bash
git clone https://github.com/nt202/macviz.git
cd macviz
cargo build --release
```

The binary will be available at `target/release/macviz`.

## 🛠 Usage

1. First, create a JSON file with your device definitions (see example below)
2. Run nmap and pipe its XML output to MacViz:

```bash
sudo nmap -sn -oX - 192.168.0.0/24 | ./macviz --devices ./devices.json
```

### Example devices.json
```json
[
    {
        "name": "My Laptop",
        "macs": ["aa:bb:cc:dd:ee:ff", "00:11:22:33:44:55"]
    },
    {
        "name": "Smart TV",
        "macs": ["66:77:88:99:00:11"]
    }
]
```

## 📊 Sample Output

```
+-------------+-------------------+------------+
| IP          | MAC               | Device     |
+==============================================+
| 192.168.0.2 | AA:BB:CC:DD:EE:FF | My Laptop  |
| 192.168.0.3 | 66:77:88:99:00:11 | Smart TV   |
| 192.168.0.4 | 11:22:33:44:55:66 | Unknown    |
+-------------+-------------------+------------+
```

## 🤝 Contributing

Contributions are welcome! Please open an issue or pull request for any bugs or feature requests.

## 📜 License

MIT - See [LICENSE](LICENSE) for more information.

---

Made with ❤️ by [nt202]