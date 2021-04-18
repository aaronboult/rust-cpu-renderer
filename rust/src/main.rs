mod simulator;

fn main() -> Result<(), ()> {

    let mut sim = simulator::Simulator::new();

    sim.start();

    Ok(())

}