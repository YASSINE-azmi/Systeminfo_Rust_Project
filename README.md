# System Resource & Process Monitor (System_Rust_Project)

A lightweight Cross-Platform Systems utility written in **Rust** using the `sysinfo` crate. This tool interacts directly with the operating system to retrieve and display live hardware utilization and a neatly formatted process monitoring table.

---

## 🚀 Features

* **OS Identification:** Detects and displays the host operating system name.
* **Memory Tracking:** Monitors RAM and Swap memory usage in real-time.
* **CPU Core Count:** Displays total available logical CPU cores.
* **Storage Diagnostics:** Fetches and prints detailed information about all mounted disks.
* **Process Monitor:** Generates a clean, aligned tabular list of all running processes including their **PIDs**, **Names**, and **Disk Usage**.
* **Memory Safety:** Built purely in Rust, safely handling OS-specific text encoding (`OsStr`) without runtime panics.

---

## 🛠️ Prerequisites

To run this project, you need to have the Rust toolchain installed on your system. If you don't have it, install it via:

```bash
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
