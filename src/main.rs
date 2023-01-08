use battery::units::Ratio;
use battery::Manager;
use battery::State;
use soloud::*;
use std::{thread, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let m = Manager::new()?;
  let min: Ratio = Ratio::new::<battery::units::ratio::ratio>(0.02);
  
  let sl = Soloud::default()?;
  let mut speech = audio::Speech::default();
  speech.set_text("charge meeee")?;

  loop {
    let mut ok = true;
    for b in m.batteries()?.filter_map(|x| x.ok()) {
      let state = b.state();
      if state != State::Charging {
        continue;
      }

      if b.energy() / b.energy_full() < min {
        sl.play(&speech);
        while sl.active_voice_count() > 0 {
          std::thread::sleep(std::time::Duration::from_millis(100));
        }
        ok = false;
      }
    }

    thread::sleep(Duration::from_secs(if ok { 600 } else { 1 }));
  }
}
