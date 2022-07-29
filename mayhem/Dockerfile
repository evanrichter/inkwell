# Build Stage
FROM ghcr.io/evanrichter/llvm-asan:14.0.5 as builder

## Add source code to the build stage.
ADD . /src
WORKDIR /src

RUN echo building instrumented harnesses && \
    export LLVM_SYS_140_PREFIX=/opt/llvm/ && \
    bash -c "pushd fuzz && cargo +nightly -Z sparse-registry fuzz build && popd" && \
    mv fuzz/target/x86_64-unknown-linux-gnu/release/bitcode_parse /bitcode_parse && \
    echo done

FROM ghcr.io/evanrichter/llvm-asan:14.0.5.no-asan as builder_no_inst

## Add source code to the build stage.
ADD . /src
WORKDIR /src

RUN echo building non-instrumented harnesses && \
    export LLVM_SYS_140_PREFIX=/opt/llvm/ && \
    export RUSTFLAGS="--cfg fuzzing -Clink-dead-code -Cdebug-assertions -C codegen-units=1" && \
    bash -c "pushd fuzz && cargo +nightly -Z sparse-registry build --release && popd" && \
    mv fuzz/target/release/bitcode_parse /bitcode_parse_no_inst && \
    echo done

# Package Stage
FROM rustlang/rust:nightly

COPY --from=builder /bitcode_parse /
COPY --from=builder_no_inst /bitcode_parse_no_inst /
