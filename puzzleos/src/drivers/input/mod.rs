use crate::drivers::bus::virtio::VirtioHal;
use crate::sync::UPIntrFreeCell;

use core::any::Any;
use virtio_drivers::VirtIOInput;

use virtio_input_decoder::Decoder;

struct VirtIOINPUT(UPIntrFreeCell<VirtIOInput<'static, VirtioHal>>);

pub trait INPUTDevice: Send + Sync + Any {
    fn handle_irq(&self);
}

impl VirtIOINPUT {}

impl INPUTDevice for VirtIOINPUT {
    fn handle_irq(&self) {
        let mut input = self.0.exclusive_access();
        input.ack_interrupt();
        let event = input.pop_pending_event().unwrap();
        let dtype = match Decoder::decode(
            event.event_type as usize,
            event.code as usize,
            event.value as usize,
        ) {
            Ok(dtype) => dtype,
            Err(_) => return,
        };
        match dtype {
            virtio_input_decoder::DecodeType::Mouse(mouse) => println!("{:?}", mouse),
            _ => {}
        }
    }
}
