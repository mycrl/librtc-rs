use super::rtc_datachannel::*;
use super::rtc_peerconnection::*;
use std::sync::Arc;
use tokio::sync::broadcast::*;
use tokio::sync::Mutex;

#[repr(C)]
pub struct RawEvents {
    ctx: *mut EventerContext,
    on_connectionstatechange: extern "C" fn(*mut EventerContext, ConnectionState),
    on_datachannel: extern "C" fn(*mut EventerContext, RawRTCDataChannel),
}

extern "C" fn on_connectionstatechange(ctx: *mut EventerContext, state: ConnectionState) {
    unsafe { &*ctx }
        .connectionstatechange_sdr
        .send(state)
        .unwrap();
}

extern "C" fn on_datachannel(ctx: *mut EventerContext, datachanel: RawRTCDataChannel) {
    // unsafe { &*ctx }
    //     .datachannel_sdr
    //     .send(Some(datachanel))
    //     .unwrap();
}

pub struct EventerContext {
    connectionstatechange_sdr: Sender<ConnectionState>,
    datachannel_sdr: Sender<RTCDataChannel>,
}

impl EventerContext {
    pub fn get_raw(&self) -> RawEvents {
        RawEvents {
            ctx: self as *const Self as *mut Self,
            on_connectionstatechange: on_connectionstatechange,
            on_datachannel: on_datachannel,
        }
    }
}

#[derive(Clone)]
pub struct ChennelRecv<T> {
    inner: Arc<Mutex<Receiver<T>>>,
}

impl<T: Clone> ChennelRecv<T> {
    pub fn new(capacity: usize) -> (Sender<T>, Self) {
        let (sender, inner) = channel(capacity);
        (
            sender,
            Self {
                inner: Arc::new(Mutex::new(inner)),
            },
        )
    }

    pub async fn recv(&self) -> Option<T> {
        self.inner.lock().await.recv().await.ok()
    }
}

pub struct Eventer {
    pub connectionstatechange_rev: ChennelRecv<ConnectionState>,
    pub datachannel_rev: ChennelRecv<RTCDataChannel>,
    pub(crate) ctx: EventerContext,
}

impl Eventer {
    pub fn new() -> Self {
        let (connectionstatechange_sdr, connectionstatechange_rev) = ChennelRecv::new(1);
        let (datachannel_sdr, datachannel_rev) = ChennelRecv::new(1);
        Self {
            connectionstatechange_rev,
            datachannel_rev,
            ctx: EventerContext {
                connectionstatechange_sdr,
                datachannel_sdr,
            },
        }
    }
}
