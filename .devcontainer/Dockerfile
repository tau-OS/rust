ARG VARIANT="37"
FROM fedora:${VARIANT}

RUN useradd -m vscode
RUN echo vscode ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/vscode
RUN dnf -y install git 'dnf-command(config-manager)' gcc vala valadoc 'pkgconfig(gweather4)' 'pkgconfig(gnome-desktop-4)' 'pkgconfig(libgtop-2.0)' 'pkgconfig(gudev-1.0)' 'pkgconfig(udisks2)' 'pkgconfig(accountsservice)' desktop-file-utils
RUN dnf config-manager --add-repo='https://raw.githubusercontent.com/terrapkg/subatomic-repos/main/tau1.repo'
RUN dnf -y install libhelium-devel libbismuth-devel fusebox-devel sass meson
USER vscode
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

