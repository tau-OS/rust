ARG VARIANT="37"
FROM fedora:${VARIANT}

RUN useradd -m vscode
RUN echo vscode ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/vscode
RUN dnf -y install rpm-ostree git 'dnf-command(config-manager)'
RUN dnf config-manager --add-repo='https://terra.fyralabs.com/terra.repo'
RUN dnf -y install libhelium-devel libbismuth-devel gcc
USER vscode
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

