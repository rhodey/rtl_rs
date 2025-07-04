# rtl_rs
RTLSDR driver which continuously reads args from stdin

Read [better software defined radio drivers](https://rhodey.org/blog/better-sdr-drivers) for arguments / philosophy

## Build
See Dockerfile if you want to install within your OS / not inside container
```
docker build -t rtl_rs .
```

## Run
All arguments follow RTLSDR [conventions](https://osmocom.org/projects/rtl-sdr/wiki/Rtl-sdr)

The helper script rtl_devices.sh adds args needed to map USB to the container

The command line program [demod](https://github.com/cubehub/demod) is installed into the container
```
docker run $(./rtl_devices.sh) --rm -i --entrypoint /bin/bash rtl_rs -c "rtl_rs -d 0 -s 230400 -f 94100000 | \
  demod --samplerate 230400 --intype i16 --outtype i16 --bandwidth 100000 fm --deviation 75000" | \
    play -t raw -r 230400 -e signed-integer -b16 -c 1 -V1 -
```

## Usage
Example shows tune to 94.1 FM and play audio and then tune to 92.3 FM and continue to play without exit
```
rtl_rs -d 0 -s 230400 -f 94100000 | \
  demod --samplerate 230400 --intype i16 --outtype i16 --bandwidth 100000 fm --deviation 75000 | \
    play -t raw -r 230400 -e signed-integer -b16 -c 1 -V1 -
stdin> -f 92300000
stderr> ok: -f 92300000
```

Docker has a history of doing things wrong with pipes / you may need to do this
```
mkfifo /tmp/in
cat /tmp/in | docker run $(./rtl_devices.sh) --rm -i --entrypoint /bin/bash rtl_rs -c "rtl_rs -d 0 -s 230400 -f 94100000 | \
  demod --samplerate 230400 --intype i16 --outtype i16 --bandwidth 100000 fm --deviation 75000" | \
    play -t raw -r 230400 -e signed-integer -b16 -c 1 -V1 -
```

Then in another terminal
```
exec 3>/tmp/in
echo "-f 92300000" >&3
```

## License
Copyright 2025 - mike@rhodey.org

MIT
