FROM rust:latest

# Add user with same UID and GID as your host system, replace if one exists with same UID
ARG USER_NAME=rusty
ARG USER_ID
ARG GROUP_NAME
ARG GROUP_ID

RUN set -ex; \
    if getent group $GROUP_ID > /dev/null; then \
    OLD_GROUP_NAME=$(getent group $GROUP_ID |  cut -d: -f1); \
    groupmod --new-name $GROUP_NAME $OLD_GROUP_NAME; \
    else \
    groupadd --gid $GROUP_ID $GROUP_NAME; \
    fi \
    && if getent passwd $USER_ID > /dev/null; then \
    OLD_USER_NAME=$(getent passwd $USER_ID |  cut -d: -f1); \
    usermod -l $USER_NAME $OLD_USER_NAME; \
    usermod -d /home/$USER_NAME -m $USER_NAME; \
    else \
    useradd -s /bin/bash --uid $USER_ID --gid $GROUP_ID -m $USER_NAME; \
    fi \
    && apt-get update \
    && apt-get install -y sudo \
    && echo $USER_NAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USER_NAME \
    && chmod 0440 /etc/sudoers.d/$USER_NAME

RUN rustup component add rustfmt clippy \
    && cargo install cargo-edit \
    && rm -rf /var/lib/apt/lists/*

USER $USER_NAME

# zsh install
RUN sh -c "$(wget -O- https://github.com/deluan/zsh-in-docker/releases/download/v1.2.0/zsh-in-docker.sh)" -- \
    -t robbyrussell \
    -p git \
    -p sudo

# Append Zsh history settings to ~/.zshrc
RUN cat <<'EOF' >> ~/.zshrc
# Persist Zsh history across container rebuilds
HISTFILE=$HOME/.zsh_history
HISTSIZE=10000
SAVEHIST=10000
setopt appendhistory
EOF

CMD ["/usr/bin/zsh"]
