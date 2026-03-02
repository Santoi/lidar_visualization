ARG ROS_DISTRO=humble
FROM ros:$ROS_DISTRO AS base
ARG DEBIAN_FRONTEND=noninteractive

################################################################################
# User 'dev'
################################################################################

ARG USERNAME=dev
ARG USER_UID=1001
ARG USER_GID=1001

# Automatically creates a UID, group ${USERNAME}, adds ${USERNAME} to the group
#
# NB: If using this image as a base for a devcontainer, it is critical to have
# just one user, since dynamic UID mapping will not work otherwise, refer to:
#   https://github.com/microsoft/vscode-remote-release/issues/1155
#
RUN groupadd --gid ${USER_GID} ${USERNAME} && \
    useradd -o --uid ${USER_UID} --gid ${USER_GID} -s "/bin/bash" -m ${USERNAME} && \
    apt-get install -y sudo && \
    echo "${USERNAME} ALL=NOPASSWD: ALL" > /etc/sudoers.d/${USERNAME} && \
    chmod 0440 /etc/sudoers.d/${USERNAME}

RUN usermod -a -G dialout $USERNAME

ARG RUST_VERSION=1.89.0

COPY rust-dev/dev.bashrc /home/${USERNAME}/.bashrc.d/dev.bashrc
RUN echo "source /home/${USERNAME}/.bashrc.d/dev.bashrc" >> /home/${USERNAME}/.bashrc

RUN mkdir -p /home/${USERNAME}/.local/share/bash-completion/completions/ && \
    chown -R ${USERNAME}:${USERNAME} /home/${USERNAME}/.local

# Debugging with root.

COPY rust-dev/dev.bashrc /root/.bashrc.d/dev.bashrc
RUN echo "source /root/.bashrc.d/dev.bashrc" >> /root/.bashrc

USER ${USERNAME}

USER root

# Install dependencies
RUN sudo apt-get update && sudo apt-get install -y \
    curl \
    git \
    libclang-dev \
    ssh \
    tmux \
    python3-pip \
    && sudo rm -rf /var/lib/apt/lists/*

# Install Rust
ENV RUSTUP_HOME="/opt/rustup"
ENV CARGO_HOME="/opt/cargo"
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y  --default-toolchain ${RUST_VERSION} \
    && . "${CARGO_HOME}/env" \
    && rustup component add clippy \
    && rustup component add rustfmt \
    && cargo install cargo-deb \
    && cargo install cargo-expand \
    && cargo install cargo-rdme \
    && rustup target add x86_64-unknown-linux-gnu \
    && rustup completions bash cargo >> /home/${USERNAME}/.local/share/bash-completion/completions/cargo \
    && echo "export RUSTUP_HOME=${RUSTUP_HOME}" >> /etc/profile \
    && echo "export PATH=${CARGO_HOME}/bin:\${PATH}" >> /etc/profile
ENV PATH="$CARGO_HOME/bin/:$PATH"

# Install the colcon-cargo and colcon-ros-cargo plugins
RUN if [ "$ROS_DISTRO" = "humble" ] ;  \
    then pip install --upgrade pytest && pip install colcon-ros-cargo colcon-cargo ;  \
    else pip install --break-system-packages pytest colcon-ros-cargo colcon-cargo ; fi

RUN cargo install rerun-cli@0.23.4

WORKDIR /workspace

# Add ros source to bashrc
RUN echo "source /opt/ros/${ROS_DISTRO}/setup.bash" >> /home/${USERNAME}/.bashrc

RUN chown -R ${USERNAME}:${USERNAME} /workspace && \
    chmod -R 755 /workspace
RUN chown -R ${USERNAME}:${USERNAME} "${CARGO_HOME}" && \
    chmod -R 755 "${CARGO_HOME}"
RUN chown -R ${USERNAME}:${USERNAME} "${RUSTUP_HOME}" && \
    chmod -R 755 "${RUSTUP_HOME}"

RUN echo "export PATH=/root/.cargo/bin:\$PATH" >> /etc/profile

ENV TERM=xterm-256color
ENTRYPOINT ["/bin/bash", "--login", "-i"]
