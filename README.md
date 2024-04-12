
# 🌌 VRChat Picture Sort Tool 📂

Welcome to the VRChat Picture Sort Tool! This command-line utility dives into the depths of your VRChat memories, organizing those precious snapshots into neat, chronological folders. Say goodbye to endless scrolling in search of that one perfect moment.

## Features 🚀

- **Automatic Sorting:** Automatically sorts your VRChat pictures into folders based on the year and month they were captured.
- **Thorough Organization:** Uses a smart, recursive directory traversal to ensure no picture gets left behind, regardless of where it's stored.
- **Easy to Use:** A simple command-line interface means you get up and running with just a single command.
- **Threaded Efficiency:** Harnesses the power of Rust's threading to speed up the sorting process, handling large volumes of pictures without breaking a sweat.
- **Clutter-Free:** Cleans up empty directories left behind, keeping your storage space neat and tidy.

## How It Works 🛠️

The tool scans the specified directory for VRChat pictures, then groups them by their creation dates. It organizes these groups into new folders named in a `YY-MM` format (year-month), making your memories easy to find and enjoy.

## Getting Started 🚀

To use this tool, follow these simple steps:

1. **Clone the Repository:**

   ```bash
   git clone https://github.com/your-username/VRChat-Picture-Sort.git
   ```

2. **Navigate to the Project Directory:**

   ```bash
   cd VRChat-Picture-Sort
   ```

3. **Compile the Project:**

   Make sure you have Rust and Cargo installed on your system. Then, compile the project with:

   ```bash
   cargo build --release
   ```

4. **Run the Tool:**

   Navigate to the target directory where your compiled binary is located and run:

   ```bash
   ./vrchat_picture_sort <path-to-your-vrchat-pictures>
   ```

Replace `<path-to-your-vrchat-pictures>` with the actual path where your VRChat pictures are stored.

## Contribution 🤝

Contributions are more than welcome! If you have suggestions for improvements or new features, feel free to open an issue or submit a pull request.

## License 📜

This project is licensed under [MIT License](LICENSE). Feel free to use it, contribute, or share it as you see fit.

---

Happy Sorting! 🎉 Rediscover your VRChat adventures with the VRChat Picture Sort Tool.
