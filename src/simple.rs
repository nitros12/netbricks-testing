use netbricks::common::Result;
use netbricks::interface::{PacketRx, PacketTx};
use netbricks::operators::{Batch, ReceiveBatch};
use netbricks::packets::ip::v4::Ipv4;
use netbricks::packets::{Ethernet, Packet, RawPacket};
use netbricks::runtime::Runtime;
use netbricks::scheduler::Scheduler;
use std::fmt::Display;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

static GLOBAL_TCP_PACKET_COUNT: AtomicUsize = AtomicUsize::new(0);
static GLOBAL_ETHERNET_PACKET_COUNT: AtomicUsize = AtomicUsize::new(0);

pub fn run(mut runtime: Runtime) -> Runtime {
    runtime.add_pipeline_to_run(install);
    runtime.add_task_to_run(log_packets_received, Duration::new(5, 0));

    runtime
}

fn install<T, S>(ports: Vec<T>, sched: &mut S)
where
    T: PacketRx + PacketTx + Display + Clone + 'static,
    S: Scheduler + Sized,
{
    let pipelines: Vec<_> = ports
        .iter()
        .map(|port|
             ReceiveBatch::new(port.clone())
             .map(eth_nf)
             .map(ipv4_nf)
             .send(port.clone()))
        .collect();

    println!("running {} pipes", pipelines.len());

    for pipeline in pipelines {
        sched.add_task(pipeline).unwrap();
    }
}

#[inline]
pub fn eth_nf(packet: RawPacket) -> Result<RawPacket> {
    let mut ethernet = packet.parse::<Ethernet>()?;

    ethernet.swap_addresses();

    GLOBAL_ETHERNET_PACKET_COUNT.fetch_add(1, Ordering::Relaxed);

    // let info_fmt = format!("[eth] {}", ethernet).magenta().bold();
    // println!("{}", info_fmt);

    Ok(ethernet.deparse())
}

#[inline]
pub fn ipv4_nf(p: RawPacket) -> Result<RawPacket> {
    let ipv4 = p.parse::<Ethernet>()?.parse::<Ipv4>()?;

    Ok(ipv4.deparse().deparse())
}

fn log_packets_received() {
    println!("tcp packets: {}", GLOBAL_TCP_PACKET_COUNT.load(Ordering::Relaxed));
    println!("ethernet packets: {}", GLOBAL_ETHERNET_PACKET_COUNT.load(Ordering::Relaxed));
}
