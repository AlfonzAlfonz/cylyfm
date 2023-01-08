use battery::units::Ratio;
use battery::Battery;
use battery::Manager;
use battery::State;
use soloud::*;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let min: Ratio = Ratio::new::<battery::units::ratio::ratio>(0.02);
  let m = Manager::new()?;

  loop {
    for mut b in m.batteries()?.filter_map(|x| x.ok()) {
      let state = b.state();
      if state != State::Charging {
        continue;
      }

      if is_bat_ok(&b, &min) {
        break;
      }

      let sl = Soloud::default()?;
      let mut speech = audio::Speech::default();
      speech.set_text("charge meeee")?;

      loop {
        sl.play(&speech);
        while sl.active_voice_count() > 0 {
          sleep(Duration::from_millis(100));
        }

        sleep(Duration::from_secs(1));

        m.refresh(&mut b).ok();
        if is_bat_ok(&b, &min) || state == State::Charging {
          break;
        }
      }
    }

    sleep(Duration::from_secs(120));
  }
}

fn is_bat_ok(b: &Battery, min: &Ratio) -> bool {
  &(b.energy() / b.energy_full()) >= min
}
