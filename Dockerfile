FROM archlinux:latest
RUN echo Install packages && \
pacman -Syyu --noconfirm git base-devel rustup vim neovim screen tmux && \
rustup default stable && \
rustup component add clippy && \
rustup component add rustfmt

# try if write_and_error lint compiles and works by itself
RUN echo Install repo and prerequisits && \
cargo install --version "0.1.2" cargo-dylint dylint-link && \
git clone https://github.com/gww-parity/substrate_lints.git && \
cd substrate_lints/write_and_error && \
make

# now install dylints_updater and use it to update lints in lints cache
ENV PATH=$PATH:/root/.cargo/bin/:/vagrant/.cargo/bin/
RUN cargo install --git https://github.com/gww-parity/dylints_updater.git --branch dylints_updater
ENV DYLINT_LIBRARY_PATH=/root/.cache/dylint/lints
RUN mkdir -p $DYLINT_LIBRARY_PATH
RUN cd /substrate_lints && dylints_updater && ls -lha $DYLINT_LIBRARY_PATH

# let's try to use installed lint
RUN cd /substrate_lints/write_and_error/inputs/pseudo_write_and_err_00 && cargo dylint write_and_error

# To run last line again in terminal:
# image_hash="$(docker build .|tail -n 1|awk '{print $NF}')"
# docker run $image_hash bash -c 'cd /substrate_lints/write_and_error/inputs/pseudo_write_and_err_00; cargo dylint write_and_error'
