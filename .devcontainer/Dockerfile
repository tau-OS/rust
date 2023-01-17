ARG VARIANT="37"
FROM fedora:${VARIANT}

RUN useradd -m vscode
RUN echo vscode ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/vscode
RUN dnf -y install git 'dnf-command(config-manager)'
RUN dnf config-manager --add-repo='https://raw.githubusercontent.com/terrapkg/subatomic-repos/main/tau1.repo'
RUN dnf -y install libhelium-devel libbismuth-devel gcc fusebox-devel
USER vscode
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

