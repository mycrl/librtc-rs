mod raw;

// use anyhow::{anyhow, Result};
// use futures::task::AtomicWaker;
// use libc::*;
// use std::ffi::CString;
// use std::future::Future;
// use std::pin::Pin;
// use std::sync::atomic::{AtomicPtr, Ordering};
// use std::sync::Arc;
// use std::task::*;

// #[derive(Default)]
// pub struct RTCIceServer {
//     credential: Option<CString>,
//     username: Option<CString>,
//     urls: Option<Vec<*const c_char>>,
//     raw_urls: Vec<CString>,
// }

// impl RTCIceServer {
//     pub fn set_credential(&mut self, credential: &str) {
//         self.credential = Some(CString::new(credential).unwrap());
//     }

//     pub fn set_username(&mut self, username: &str) {
//         self.username = Some(CString::new(username).unwrap());
//     }

//     pub fn set_urls(&mut self, urls: &[&str]) {
//         self.raw_urls = urls.iter().map(|url| CString::new(*url).unwrap()).collect();
//         self.urls = Some(
//             self.raw_urls
//                 .iter()
//                 .map(|url| url.as_c_str().as_ptr())
//                 .collect(),
//         );
//     }

//     pub fn as_raw(&self) -> raw::RTCIceServer {
//         raw::RTCIceServer {
//             credential: self.credential.as_ref().map(|c| c.as_c_str().as_ptr()),
//             username: self.username.as_ref().map(|u| u.as_c_str().as_ptr()),
//             urls: self.urls.as_ref().map(|u| u.as_ptr()),
//             urls_size: match &self.urls {
//                 Some(urls) => urls.len() as c_int,
//                 None => 0,
//             },
//         }
//     }
// }

// #[derive(Default)]
// pub struct RTCConfiguration {
//     pub bundle_policy: Option<raw::BundelPolicy>,
//     pub ice_transport_policy: Option<raw::IceTransportPolicy>,
//     pub peer_identity: Option<CString>,
//     pub rtcp_mux_policy: Option<raw::RtcpMuxPolicy>,
//     pub ice_servers: Option<Vec<raw::RTCIceServer>>,
//     pub ice_candidate_pool_size: Option<u8>,
// }

// impl RTCConfiguration {
//     pub fn set_bundle_policy(&mut self, bundle_policy: raw::BundelPolicy) {
//         self.bundle_policy = Some(bundle_policy);
//     }

//     pub fn set_ice_transport_policy(&mut self, ice_transport_policy: raw::IceTransportPolicy) {
//         self.ice_transport_policy = Some(ice_transport_policy);
//     }

//     pub fn set_peer_identity(&mut self, peer_identity: &str) {
//         self.peer_identity = Some(CString::new(peer_identity).unwrap());
//     }

//     pub fn set_rtcp_mux_policy(&mut self, rtcp_mux_policy: raw::RtcpMuxPolicy) {
//         self.rtcp_mux_policy = Some(rtcp_mux_policy);
//     }

//     pub fn set_ice_servers(&mut self, ice_servers: Vec<RTCIceServer>) {
//         self.ice_servers = Some(ice_servers.iter().map(|i| i.as_raw()).collect());
//     }

//     pub fn set_ice_candidate_pool_size(&mut self, ice_candidate_pool_size: u8) {
//         self.ice_candidate_pool_size = Some(ice_candidate_pool_size);
//     }

//     pub fn as_raw(&self) -> *const raw::RTCPeerConnectionConfigure {
//         Box::into_raw(Box::new(raw::RTCPeerConnectionConfigure {
//             bundle_policy: self.bundle_policy,
//             ice_transport_policy: self.ice_transport_policy,
//             rtcp_mux_policy: self.rtcp_mux_policy,
//             peer_identity: self.peer_identity.as_ref().map(|s| s.as_ptr()),
//             ice_candidate_pool_size: self.ice_candidate_pool_size.map(|i| i as c_int),
//             ice_servers: self.ice_servers.as_ref().map(|i| i.as_ptr()),
//             ice_servers_size: match &self.ice_servers {
//                 Some(i) => i.len() as c_int,
//                 None => 0,
//             },
//         }))
//     }
// }

// #[derive(Debug)]
// pub struct RTCSessionDescription {
//     r#type: raw::RtcSessionDescriptionType,
//     sdp: CString,
// }

// impl RTCSessionDescription {
//     pub fn from_raw(raw: *const raw::RTCSessionDescription) -> Self {
//         let raw = unsafe { &*raw };
//         Self {
//             r#type: raw.r#type,
//             sdp: unsafe { CString::from_raw(raw.sdp as *mut c_char) },
//         }
//     }

//     pub fn get_type(&self) -> raw::RtcSessionDescriptionType {
//         self.r#type
//     }

//     pub fn get_sdp(&self) -> Result<&str> {
//         Ok(self.sdp.to_str()?)
//     }
// }

// pub struct RTCPeerConnection {
//     raw: &'static raw::RTCPeerConnection,
// }

// impl RTCPeerConnection {
//     pub fn new(config: &RTCConfiguration) -> Result<Self> {
//         let raw = unsafe { raw::create_rtc_peer_connection(config.as_raw()) };
//         if raw.is_null() {
//             return Err(anyhow!("crate RTCPeerConnection failed!"));
//         }

//         Ok(Self {
//             raw: unsafe { &*raw },
//         })
//     }

//     pub fn create_offer(&self) -> CreateSessionDescription {
//         CreateSessionDescription::new(self.raw, CreateSessionDescriptionKind::Offer)
//     }

//     pub fn create_answer(&self) -> CreateSessionDescription {
//         CreateSessionDescription::new(self.raw, CreateSessionDescriptionKind::Answer)
//     }
// }

// pub struct CreateSessionDescriptionContext {
//     callback: Box<dyn FnMut(*const c_char, *const raw::RTCSessionDescription)>,
// }

// #[derive(PartialEq, PartialOrd)]
// pub enum CreateSessionDescriptionKind {
//     Offer,
//     Answer,
// }

// pub struct CreateSessionDescription {
//     kind: CreateSessionDescriptionKind,
//     waker: Arc<AtomicWaker>,
//     peer: &'static raw::RTCPeerConnection,
//     res: Arc<AtomicPtr<Result<*mut raw::RTCSessionDescription>>>,
//     begin: bool,
// }

// impl CreateSessionDescription {
//     pub fn new(peer: &'static raw::RTCPeerConnection, kind: CreateSessionDescriptionKind) -> Self {
//         Self {
//             waker: Arc::new(AtomicWaker::new()),
//             res: Arc::new(AtomicPtr::new(std::ptr::null_mut())),
//             begin: false,
//             peer,
//             kind,
//         }
//     }
// }

// impl Future for CreateSessionDescription {
//     type Output = anyhow::Result<RTCSessionDescription>;
//     fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//         self.as_ref().waker.register(cx.waker());

//         if !self.as_ref().begin {
//             let waker = self.as_ref().waker.clone();
//             let desc = self.as_mut().res.clone();
//             let ctx = Box::new(CreateSessionDescriptionContext {
//                 callback: Box::new(move |err, sdesc| {
//                     desc.store(
//                         Box::into_raw(Box::new(if err.is_null() {
//                             Ok(sdesc as *mut raw::RTCSessionDescription)
//                         } else {
//                             Err(anyhow!(unsafe {
//                                 CString::from_raw(err as *mut c_char).into_string().unwrap()
//                             }))
//                         })),
//                         Ordering::Relaxed,
//                     );
//                     waker.wake();
//                 }),
//             });

//             if self.as_ref().kind == CreateSessionDescriptionKind::Offer {
//                 unsafe {
//                     raw::rtc_create_offer(
//                         self.as_ref().peer,
//                         callback,
//                         Box::into_raw(ctx) as *mut c_void,
//                     )
//                 };
//             } else {
//                 unsafe {
//                     raw::rtc_create_answer(
//                         self.as_ref().peer,
//                         callback,
//                         Box::into_raw(ctx) as *mut c_void,
//                     )
//                 };
//             }

//             self.as_mut().begin = true;
//             Poll::Pending
//         } else {
//             let res = unsafe { Box::from_raw(self.as_ref().res.load(Ordering::Relaxed)) };
//             Poll::Ready(res.map(|desc| RTCSessionDescription::from_raw(desc)))
//         }
//     }
// }

// extern "C" fn callback(
//     error: *const c_char,
//     desc: *const raw::RTCSessionDescription,
//     ctx: *mut c_void,
// ) {
//     let mut ctx = unsafe { Box::from_raw(ctx as *mut CreateSessionDescriptionContext) };
//     (ctx.callback)(error, desc);
// }
