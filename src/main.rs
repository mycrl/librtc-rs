#![feature(vec_into_raw_parts)]

mod ffi;

// use ffi::*;

// #[tokio::main]
// async fn main() {
//     let config = RTCConfiguration::default();
//     let peer = RTCPeerConnection::new(&config).unwrap();

//     tokio::spawn(async move {
//         let offer = peer.create_offer().await.unwrap();
//         println!("type: {:?}", offer.get_type());
//         println!("sdp: {:?}", offer.get_sdp());
//     });

//     unsafe { raw::rtc_run() }
// }

fn main() {}
