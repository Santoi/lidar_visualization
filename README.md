# ROS 2 Rust Development Environment (`rclrs_demo`)

This project uses a Docker-based devcontainer to provide a fully configured ROS 2 (Humble) and Rust development environment. 

To ensure the ROS 2 message generators compile correctly without being overwritten by Docker bind mounts, please follow the exact setup sequence below.

## Prerequisites

* [Docker](https://docs.docker.com/get-docker/) installed and running.
* [Visual Studio Code](https://code.visualstudio.com/) installed.
* The [Dev Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) extension installed in VS Code.

## 🛠️ Environment Setup

### 1. Local Workspace Preparation (Do this FIRST)
We need to clone the `ros2_rust` generators directly to your **local host machine** before starting the container. This ensures the files are present when VS Code mounts your local directory into the container's `/workspace`.

Open a terminal on your local machine, navigate to the root of this project, and run:

```bash
# Create the src directory if it doesn't exist
mkdir -p src

# Clone the ros2_rust repository locally
git clone https://github.com/ros2-rust/ros2_rust.git src/ros2_rust

# Import the necessary repositories for Humble
vcs import src < src/ros2_rust/ros2_rust_humble.repos
```

### 2. Start the Devcontainer
1. Open this project folder in VS Code.
2. Press Ctrl+Shift+P (or Cmd+Shift+P on macOS) to open the Command Palette.
3. Select "Dev Containers: Reopen in Container".
4. Wait for the image to build and the container to start.

### 3. Install ROS Dependencies
Once inside the Devcontainer, open an integrated terminal (`Ctrl+``) and install the underlying C/C++ dependencies required by the message generators:

```bash
sudo apt-get update
rosdep update
rosdep install -y -r -q --from-paths src --ignore-src --rosdistro humble
```
### 4. Run instructions

#### Build
```bash
. /opt/ros/humble/setup.bash
vcs import --input src/ros2_rust/ros2_rust_humble.repos src/
colcon build --packages-up-to rclrs
colcon build --packages-up-to geometry_msgs
colcon build --packages-select rclrs_demo
. ./install/setup.bash
```
#### Run
```bash
ros2 run rclrs_demo rclrs_demo
```
