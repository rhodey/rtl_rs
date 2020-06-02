use std::io;
use std::io::prelude::*;
use std::io::BufRead;
use std::io::BufWriter;

#[macro_use]
extern crate clap;
use clap::{Arg, App};

extern crate getopts;
use getopts::Options;

extern crate rtlsdr_mt;

const BUF_COUNT: u32 = 1;
const BUF_SIZE: u32 = 32768;

fn main() {
  let args = App::new("rtl_rs")
                  .arg(Arg::with_name("device")
                    .short("d")
                    .required(true)
                    .takes_value(true))
                  .arg(Arg::with_name("samplerate")
                    .short("s")
                    .required(true)
                    .takes_value(true))
                  .arg(Arg::with_name("frequency")
                    .short("f")
                    .required(true)
                    .takes_value(true))
                  .arg(Arg::with_name("gain")
                    .short("g")
                    .takes_value(true))
                  .arg(Arg::with_name("ppm")
                    .short("p")
                    .takes_value(true))
                  .get_matches();

  let device = value_t_or_exit!(args.value_of("device"), u32);
  let (mut ctl, mut reader) = rtlsdr_mt::open(device).unwrap();

  let sample_rate = value_t_or_exit!(args.value_of("samplerate"), u32);
  ctl.set_sample_rate(sample_rate).unwrap();
  let sample_rate = ctl.sample_rate();

  let frequency = value_t_or_exit!(args.value_of("frequency"), u32);
  ctl.set_center_freq(frequency).unwrap();
  let frequency = ctl.center_freq();

  let gain = value_t!(args.value_of("gain"), i32).unwrap_or(0i32);
  if gain == 0i32 {
    ctl.enable_agc().unwrap();
  } else {
    ctl.set_tuner_gain(gain).unwrap();
  }

  let ppm = value_t!(args.value_of("ppm"), i32).unwrap_or(0i32);
  ctl.set_ppm(ppm).unwrap();

  eprintln!("ok: -d {} -s {} -f {} -g {} -p {}", device, sample_rate, frequency, gain, ppm);
  eprintln!("Tuned to {} Hz.", frequency);

  let mut opts = Options::new();
  opts.optopt("s", "", "", "");
  opts.optopt("f", "", "", "");
  opts.optopt("g", "", "", "");
  opts.optopt("p", "", "", "");

  std::thread::spawn(move || {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
      let args = match opts.parse(line.unwrap().split(',')) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
      };

      match args.opt_str("s") {
        Some(s) => {
          let sample_rate = s.trim().parse::<u32>().unwrap();
          ctl.set_sample_rate(sample_rate).unwrap();
          eprintln!("ok: -s {}", sample_rate);
        }, _ => {}
      }

      match args.opt_str("f") {
        Some(f) => {
          let frequency = f.trim().parse::<u32>().unwrap();
          ctl.set_center_freq(frequency).unwrap();
          eprintln!("ok: -f {}", frequency);
        }, _ => {}
      }

      match args.opt_str("g") {
        Some(g) => {
          let gain: i32 = g.trim().parse().unwrap();
          if gain == 0i32 {
            ctl.enable_agc().unwrap();
          } else {
            ctl.set_tuner_gain(gain).unwrap();
          }
          eprintln!("ok: -g {}", gain);
        }, _ => {}
      }

      match args.opt_str("p") {
        Some(p) => {
          let ppm: i32 = p.trim().parse().unwrap();
          ctl.set_ppm(ppm).unwrap();
          eprintln!("ok: -p {}", ppm);
        }, _ => {}
      }
    }
    ctl.cancel_async_read();
  });

  let mut stdout = BufWriter::with_capacity((BUF_SIZE * 2) as usize, io::stdout());
  let mut samples = vec![0u8; (BUF_SIZE * 2) as usize];

  reader.read_async(BUF_COUNT, BUF_SIZE, |bytes| {
    for bidx in (0..BUF_SIZE as usize).step_by(1) {
      let sidx = bidx * 2;
      let magic: i16 = (bytes[bidx] as i16) - 127;
      samples[sidx] = (magic & 0xFF) as u8;
      samples[sidx + 1] = (magic >> 8) as u8;
    }

    stdout.write_all(&samples);
  }).unwrap();
}
