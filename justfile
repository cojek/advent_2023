run day part: (lint day part) (test day part)
    cd day-{{day}} && \
        cargo run --bin part{{part}}

lint day part:
    cd day-{{day}} && \
      cargo clippy --bin part{{part}}

test day part:
    cd day-{{day}} && \
      cargo test --bin part{{part}}

create day:
    #!/usr/bin/env bash
    cargo new day-{{day}}
    mkdir day-{{day}}/src/bin
    rm day-{{day}}/src/main.rs
    cp template/* day-{{day}}/src/bin


