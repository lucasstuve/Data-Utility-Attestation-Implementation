RUN curl -L https://risczero.com/install | bash 
ENV PATH ="/root/.cargo/bin:$(PATH)"
RUN rzup install 
RUN rustup toolchain list --verbose | grep risc0