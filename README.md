# Battery Cycles

Count battery discharge cycles using the Full Equivalent Cycle (FEC) method.

## Usage

```rust
use std::time::Duration;
use battery_cycles::BatteryCycleCounter;

let battery_capacity_ah = 10.0;
let initial_cycle_count = 0.0;

let mut battery_cycle_counter = BatteryCycleCounter::new(battery_capacity_ah, initial_cycle_count, None);

// Repeatedly update the BatteryCycleCounter with the current discharge current
for _ in 0..10 {
    // Some time passes
    // The shorter the time between updates the more accurately the cycles will be tracked
    std::thread::sleep(Duration::from_secs_f32(1.0));

    // Measure the current discharge current of your battery somehow
    // a positive number means it's discharging, a negative number means its charging
    let battery_discharge_current_a = 0.42;

    // Repeatedly update the BatteryCycleCounter with the current discharge current
    battery_cycle_counter.update(battery_discharge_current_a, None);
}

// Get the current equivalent cycle count via the `cycle_count` method
let cycle_count = battery_cycle_counter.cycle_count();
println!("The battery has completed {cycle_count} equivalent cycles.");
```