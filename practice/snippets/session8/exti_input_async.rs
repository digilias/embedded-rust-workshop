// Session 8 Snippet: Async EXTI with ExtiInput

use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{Input, Pull};
use embassy_executor::Spawner;
use defmt::info;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    // Create EXTI input for LIS3DH INT1 pin (e.g., PC1)
    let irq_pin = ExtiInput::new(p.PC1, p.EXTI1, Pull::None);

    spawner.spawn(handle_interrupts(irq_pin)).unwrap();
}

#[embassy_executor::task]
async fn handle_interrupts(mut irq: ExtiInput<'static>) {
    info!("Waiting for interrupts...");

    loop {
        // Wait for rising edge (LIS3DH INT1 goes high)
        irq.wait_for_rising_edge().await;

        info!("Data ready!");

        // TODO: Read sensor data here
        // let data = sensor.read_accel().await;
    }
}

// Alternative: wait for any edge
async fn wait_any_edge_example(mut irq: ExtiInput<'static>) {
    loop {
        irq.wait_for_any_edge().await;
        if irq.is_high() {
            info!("Rising edge");
        } else {
            info!("Falling edge");
        }
    }
}
