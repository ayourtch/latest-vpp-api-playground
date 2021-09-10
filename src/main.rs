use bincode::Options;
use std::convert::TryInto;
use std::process::Command;
use latest_vpp_api::interface::*;
use latest_vpp_api::interface_types::*;
use latest_vpp_api::ip_types::*;
use latest_vpp_api::reqrecv::*;
use latest_vpp_api::vhost_user::*;
use latest_vpp_api::virtio_types::*;
use latest_vpp_api::vpe::CliInband;
use latest_vpp_api::vpe::CliInbandReply;
use latest_vpp_api::vpe::*;
use vpp_api_transport::afunix;
use vpp_api_transport::VppApiTransport;

fn get_encoder() -> impl bincode::config::Options {
    bincode::DefaultOptions::new()
        .with_big_endian()
        .with_fixint_encoding()
}

fn main() {
    // Connecting to the VPP API socket
    let mut t: Box<dyn VppApiTransport> = Box::new(afunix::Transport::new("/run/vpp/api.sock"));
    // Testing Connection and loading Message buffers
    println!("Connect result: {:?}", t.connect("api-test", None, 256));

    // Step 3: Create Host interface
    let show_version: ShowVersionReply = send_recv_msg(
        &ShowVersion::get_message_name_and_crc(),
        &ShowVersion::builder()
            .client_index(t.get_client_index())
            .context(0)
            .build()
            .unwrap(),
        &mut *t,
        &ShowVersionReply::get_message_name_and_crc(),
    );
    println!("{:#?}", show_version);

    t.disconnect();
}

