# rtl_rs
Better rtlsdr driver through stdin & stderr.

## build & install
```
$ rustup override add nightly
$ cargo build --release
$ cargo install --path .
```

## usage
```
$ rtl_rs -d 0 -s 230400 -f 94700000 | \
    demod --samplerate 230400 --intype i16 --outtype i16 --bandwidth 100000 fm --deviation 75000 | \
    play -t raw -r 230.4k -e signed-integer -b16 -c 1 -V1 -
stdin> -g 62, -p 18
stderr> ok: -g 62
stderr> ok: -p 18
stdin> -f 100300000
stderr> ok: -f 100300000
```

## license
GPLv3.
