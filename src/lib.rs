#![doc = include_str!("../README.md")]

use std::time::Instant;

pub struct BatteryCycleCounter {
    capacity_ah: f32,
    discharge_total_ah: f32,
    cycle_count: f32,    
    last_update_time: Instant
}

impl BatteryCycleCounter{
    /// Construct a new `BatteryCycleCounter`
    /// 
    /// - `capacity_ah`: The battery capacity in Amp Hours
    /// - `initial_cycle_count`: The number of cycles already completed by the battery. This will simply be added to the computed cycle count.
    /// - `start_timestamp`: The time of the `initial_cycle_count` or `None` if that time is now
    pub fn new(
        capacity_ah: f32,
        initial_cycle_count: f32,
        start_timestamp: Option<Instant>
    ) -> Self{
        Self{
            capacity_ah,
            discharge_total_ah: initial_cycle_count * capacity_ah,
            cycle_count: initial_cycle_count,
            last_update_time: start_timestamp.unwrap_or(Instant::now())
        }
    }

    /// Update the cycle count by recording current flow into or out of the battery.
    /// The more frequently this is called the more accurately the cycle count will be tracked.
    /// 
    /// - `discharge_current_a`: The current in Amps flowing out of the battery at the `timestamp`. Note that a positive number means the battery is discharging and a negative number means it is charging.
    /// - `timestamp`: The time at which the `discharge_current_a` was measured, or `None` if that time is now.
    /// 
    /// Returns the current cycle count.
    pub fn update(&mut self, discharge_current_a: f32, timestamp: Option<Instant>) -> f32 {
        let timestamp = timestamp.unwrap_or(Instant::now());
        let time_window_s = (timestamp - self.last_update_time).as_secs_f32();
        
        self.last_update_time = timestamp;

        if discharge_current_a < 0.0 {
            return self.cycle_count
        }

        let time_window_h = time_window_s / 3600.0;
        let delta_discharge_total_ah = time_window_h * discharge_current_a;
        self.discharge_total_ah += delta_discharge_total_ah;
        
        self.cycle_count = self.discharge_total_ah / self.capacity_ah;


        self.cycle_count
    }

    /// Get the current cycle count in Full Equivalent Cycles
    pub fn cycle_count(&self) -> f32 {
        self.cycle_count
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn it_works() {

        let start_time = Instant::now();

        let mut subject = BatteryCycleCounter::new(100.0, 42.0, Some(start_time));

        let cycle_count = subject.cycle_count();
        assert_eq!(cycle_count, 42.0);

        let time_1 = start_time.checked_add(Duration::from_secs_f32(3600.0)).unwrap();
        subject.update(1.0, Some(time_1));
        assert_eq!(subject.cycle_count(), 42.01);

        let time_2 = time_1.checked_add(Duration::from_secs_f32(3600.0)).unwrap();
        subject.update(-1.0, Some(time_2));
        assert_eq!(subject.cycle_count(), 42.01);

        let time_3 = time_2.checked_add(Duration::from_secs_f32(3600.0)).unwrap();
        subject.update(1.0, Some(time_3));
        assert_eq!(subject.cycle_count(), 42.02);

        let time_4 = time_3.checked_add(Duration::from_secs_f32(3600.0)).unwrap();
        subject.update(10.0, Some(time_4));
        assert_eq!(subject.cycle_count(), 42.12);

        let time_5 = time_4.checked_add(Duration::from_secs_f32(3600.0 * 2.0)).unwrap();
        subject.update(1.0, Some(time_5));
        assert_eq!(subject.cycle_count(), 42.14);
    }
}
