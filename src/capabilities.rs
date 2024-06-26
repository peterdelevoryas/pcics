/*!
## PCI Capabilities

Each Capability structure has a Capability ID assigned by the PCI-SIG.

Capabilities list
- [x] [Null Capability](CapabilityKind::NullCapability) (00h)
- [x] [PCI Power Management Interface](power_management_interface) (01h)
- [x] [AGP](accelerated_graphics_port) (02h)
- [x] [VPD](vital_product_data) (03h)
- [x] [Slot Identification](slot_identification) (04h)
- [x] [Message Signaled Interrupts](message_signaled_interrups) (05h)
- [ ] [CompactPCI Hot Swap](compact_pci_hot_swap) (06h)
- [x] [PCI-X](pci_x) (07h)
- [x] [HyperTransport](hypertransport) (08h)
- [x] [Vendor Specific](vendor_specific) (09h)
- [x] [Debug port](debug_port) (0Ah)
- [ ] [CompactPCI central resource control](compact_pci_resource_control) (0Bh)
- [x] [PCI Hot-Plug](pci_hot_plug) (0Ch)
- [x] [PCI Bridge Subsystem Vendor ID](bridge_subsystem_vendor_id) (0Dh)
- [x] [AGP 8x](agp_8x) (0Eh)
- [x] [Secure Device](secure_device) (0Fh)
- [x] [PCI Express](pci_express) (10h)
- [x] [MSI-X](msi_x) (11h)
- [x] [Serial ATA Data/Index Configuration](sata) (12h)
- [x] [Advanced Features](advanced_features) (13h)
- [x] [Enhanced Allocation](enhanced_allocation) (14h)
- [x] [Flattening Portal Bridge](flattening_portal_bridge) (15h)

Others Reserved

## Example

lspci out:
```plaintext
Capabilities: [80] MSI: Enable+ Count=1/1 Maskable- 64bit-
        Address: fee00358  Data: 0000
Capabilities: [70] Power Management version 3
        Flags: PMEClk- DSI- D1- D2- AuxCurrent=0mA PME(D0-,D1-,D2-,D3hot+,D3cold-)
        Status: D0 NoSoftRst+ PME-Enable- DSel=0 DScale=0 PME-
Capabilities: [a8] SATA HBA v1.0 BAR4 Offset=00000004
```

pcics capabilities:
```rust
# use pcics::Header;
# use pcics::capabilities::{
#     Capabilities,
#     Capability,
#     CapabilityKind,
#     MessageSignaledInterrups,
#     PowerManagementInterface,
#     Sata,
#     message_signaled_interrups as msi,
#     power_management_interface as pmi,
#     sata,
# };
// Empty header
let mut header: Header = [0u8; 0x40].as_slice().try_into().unwrap();
// Set pointer to first capability
header.capabilities_pointer = 0x80;

let device_dependent_region = [
    // 0    1    2    3    4    5    6    7    8    9    A    B    C    D    E    F
    0x00,0x80,0x00,0x80,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // 0x40
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // 0x50
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // 0x60
    0x01,0xa8,0x03,0x40,0x08,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // 0x70
    0x05,0x70,0x01,0x00,0x58,0x03,0xe0,0xfe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // 0x80
    0x60,0x1c,0x23,0x83,0x83,0x01,0x00,0x1c,0x20,0x02,0x1c,0x20,0x20,0x00,0x00,0x80, // 0x90
    0xa8,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x12,0x00,0x10,0x00,0x48,0x00,0x00,0x00, // 0xa0
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // 0xb0
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // 0xc0
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // 0xd0
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00, // 0xe0
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xb1,0x0f,0x06,0x08,0x00,0x00,0x00,0x00, // 0xf0
];

let result = Capabilities::new(&device_dependent_region, &header)
    .collect::<Vec<_>>();

let sample = vec![
    Ok(Capability {
        pointer: 0x80,
        kind: CapabilityKind::MessageSignaledInterrups(MessageSignaledInterrups {
            message_control: msi::MessageControl {
                msi_enable: true,
                multiple_message_capable: msi::MultipleMessage(0),
                multiple_message_enable: msi::MultipleMessage(0),
                per_vector_masking_capable: false,
                a_64_bit_address_capable: false,
                extended_message_data_capable: false,
                extended_message_data_enable: false,
            },
            message_address: msi::MessageAddress::Dword(0xfee00358),
            message_data: 0x0000,
            extended_message_data: 0x0000,
            mask_bits: None,
            pending_bits: None,
        })
    }),
    Ok(Capability {
        pointer: 0x70,
        kind: CapabilityKind::PowerManagementInterface(PowerManagementInterface {
            capabilities: pmi::Capabilities {
                version: 0b11,
                pme_clock: false,
                immediate_readiness_on_return_to_d0: false,
                device_specific_initialization: false,
                aux_current: pmi::AuxCurrent::SelfPowered,
                d1_support: false,
                d2_support: false,
                pme_support: pmi::PmeSupport {
                    d0: false,
                    d1: false,
                    d2: false,
                    d3_hot: true,
                    d3_cold: false,
                },
            },
            control: pmi::Control {
                power_state: pmi::PowerState::D0,
                no_soft_reset: true,
                pme_enabled: false,
                data_select: pmi::DataSelect::PowerConsumedD0,
                data_scale: pmi::DataScale::Unknown,
                pme_status: false,
            },
            bridge: pmi::Bridge {
                reserved: 0,
                b2_b3: false,
                bpcc_enabled: false,
            },
            data: 0,
        })
    }),
    Ok(Capability {
        pointer: 0xa8,
        kind: CapabilityKind::Sata(Sata {
            revision: sata::Revision { major: 1, minor: 0 },
            bar_offset: sata::BarOffset(0x00000004),
            bar_location: sata::BarLocation::Bar4,
        })
    }),
];

assert_eq!(sample, result);
```
*/

use snafu::prelude::*;

use super::DDR_OFFSET;
use crate::header::{Header, HeaderType};

// 01h PCI Power Management Interface
pub mod power_management_interface;
pub use power_management_interface::PowerManagementInterface;

// 02h AGP
pub mod accelerated_graphics_port;
pub use accelerated_graphics_port::AcceleratedGraphicsPort;

// 03h VPD
pub mod vital_product_data;
pub use vital_product_data::VitalProductData;

// 04h Slot Identification
pub mod slot_identification;
pub use slot_identification::SlotIdentification;

// 05h Message Signaled Interrupts
pub mod message_signaled_interrups;
pub use message_signaled_interrups::MessageSignaledInterrups;

// 06h CompactPCI Hot Swap
pub mod compact_pci_hot_swap {
    /*!
    # CompactPCI Hot Swap

    This Capability structure provides a standard interface to control and sense
    status within a device that supports Hot Swap insertion and extraction in a
    CompactPCI system.
    */
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct CompactPciHotSwap;
}
pub use compact_pci_hot_swap::CompactPciHotSwap;

// 07h PCI-X
pub mod pci_x;
pub use pci_x::{PciX, PciXBridge};

// 08h HyperTransport
pub mod hypertransport;
pub use hypertransport::Hypertransport;

// 09h Vendor Specific
pub mod vendor_specific;
pub use vendor_specific::VendorSpecific;

// 0Ah Debug port
pub mod debug_port;
pub use debug_port::DebugPort;

// 0Bh CompactPCI central resource control
pub mod compact_pci_resource_control {
    /*!
    # CompactPCI central resource control
    */
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct CompactPciResourceControl;
}
pub use compact_pci_resource_control::CompactPciResourceControl;

// 0Ch PCI Hot-Plug
pub mod pci_hot_plug {
    /*!
    # PCI Hot-Plug

    Indicates that the associated device conforms to the Standard Hot-Plug
    Controller model
    */
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct PciHotPlug;
}
pub use pci_hot_plug::PciHotPlug;

// 0Dh PCI Bridge Subsystem Vendor ID
pub mod bridge_subsystem_vendor_id;
pub use bridge_subsystem_vendor_id::BridgeSubsystemVendorId;

// 0Eh AGP 8x
pub mod agp_8x {
    /*!
    # AGP 8x
    */
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Agp8x;
}
pub use agp_8x::Agp8x;

// 0Fh Secure Device
pub mod secure_device {
    /*!
    # Secure Device
    */
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct SecureDevice;
}
pub use secure_device::SecureDevice;

// 10h PCI Express
pub mod pci_express;
pub use pci_express::PciExpress;

// 11h MSI-X
pub mod msi_x;
pub use msi_x::MsiX;

// 12h Serial ATA Data/Index Configuration
pub mod sata;
pub use sata::Sata;

// 13h Advanced Features (AF)
pub mod advanced_features;
pub use advanced_features::AdvancedFeatures;

// 14h Enhanced Allocation
pub mod enhanced_allocation;
pub use enhanced_allocation::EnhancedAllocation;

// 15h Flattening Portal Bridge
pub mod flattening_portal_bridge;
pub use flattening_portal_bridge::FlatteningPortalBridge;

/// Capability parsing error
#[derive(Debug, Clone, PartialEq, Eq, Snafu)]
pub enum CapabilityError {
    #[snafu(display("capabilities pointer should be greater than 0x40"))]
    Pointer,
    #[snafu(display("[{ptr:02x}] capability header is not available"))]
    Header { ptr: u8 },
    #[snafu(display("[{ptr:02x}] {source} data read error"))]
    Data {
        ptr: u8,
        source: CapabilityDataError,
    },
    #[snafu(display("[{ptr:02x}] PCI Express error: {source}"))]
    PciExpress {
        ptr: u8,
        source: pci_express::PciExpressError,
    },
    #[snafu(display("[{ptr:02x}] HyperTransport: {source}"))]
    Hypertransport {
        ptr: u8,
        source: hypertransport::HypertransportError,
    },
    #[snafu(display("[{ptr:02x}] Vendor Specific error: {source}"))]
    VendorSpecific {
        ptr: u8,
        source: vendor_specific::VendorSpecificError,
    },
    #[snafu(display("[{ptr:02x}] MSI error: {source}"))]
    MessageSignaledInterrups {
        ptr: u8,
        source: message_signaled_interrups::MessageSignaledInterrupsError,
    },
    #[snafu(display("[{ptr:02x}] PCI-X error: {source}"))]
    PciX {
        ptr: u8,
        source: pci_x::PciXError,
    },
    #[snafu(display("[{ptr:02x}] PCI-X Bridge error: {source}"))]
    PciXBridge {
        ptr: u8,
        source: pci_x::PciXBridgeError,
    },
    #[snafu(display("[{ptr:02x}] Enhanced Allocation error: {source}"))]
    EnhancedAllocation {
        ptr: u8,
        source: enhanced_allocation::EnhancedAllocationError,
    },
}


/// Common error for reading capability data
#[derive(Snafu, Debug, Clone, Copy, PartialEq, Eq)]
#[snafu(display("{name} ({size} bytes)"))]
pub struct CapabilityDataError {
    name: &'static str,
    size: usize,
}

/// An iterator through *Capabilities List*
///
/// Used to point to a linked list of new capabilities implemented by this device. This
/// register is only valid if the “Capabilities List” bit in the [crate::header::Status] Register is set. If
/// implemented, the bottom two bits are reserved and should be set to 00b.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Capabilities<'a> {
    data: &'a [u8],
    header: &'a Header,
    pointer: u8,
}
impl<'a> Capabilities<'a> {
    pub fn new(data: &'a [u8], header: &'a Header) -> Self {
        Self { data, header, pointer: header.capabilities_pointer }
    }
}
impl<'a> Iterator for Capabilities<'a> {
    type Item = CapabilityResult<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // Stop iterating if next pointer is null
        (self.pointer != 0).then(|| parse_cap(self.data, &mut self.pointer, self.header))
    }
}

type CapabilityResult<'a> = Result<Capability<'a>, CapabilityError>;
fn parse_cap<'a>(bytes: &'a [u8], pointer: &mut u8, header: &'a Header) -> CapabilityResult<'a> {
    let ptr = *pointer;
    // Capability data resides in Device dependent region (starts from 0x40)
    let offset = (*pointer as usize).checked_sub(DDR_OFFSET).ok_or_else(|| {
        *pointer = 0;
        CapabilityError::Pointer
    })?;
    let (id, cap_data) = if let Some([id, next, rest @ ..]) = bytes.get(offset..) {
        *pointer = *next;
        (*id, rest)
    } else {
        return Err(CapabilityError::Header { ptr });
    };
    use CapabilityKind as Kind;
    let kind = match id {
        0x00 => Kind::NullCapability,
        0x01 => cap_data
            .try_into()
            .map(Kind::PowerManagementInterface)
            .context(DataSnafu { ptr })?,
        0x02 => cap_data
            .try_into()
            .map(Kind::AcceleratedGraphicsPort)
            .context(DataSnafu { ptr })?,
        0x03 => cap_data
            .try_into()
            .map(Kind::VitalProductData)
            .context(DataSnafu { ptr })?,
        0x04 => cap_data
            .try_into()
            .map(Kind::SlotIdentification)
            .context(DataSnafu { ptr })?,
        0x05 => cap_data
            .try_into()
            .map(Kind::MessageSignaledInterrups)
            .context(MessageSignaledInterrupsSnafu { ptr })?,
        0x06 => Kind::CompactPciHotSwap(CompactPciHotSwap),
        0x07 => {
            if matches!(header.header_type, HeaderType::Bridge(_)) {
                cap_data
                    .try_into()
                    .map(Kind::PciXBridge)
                    .context(PciXBridgeSnafu { ptr })?
            } else {
                cap_data
                    .try_into()
                    .map(Kind::PciX)
                    .context(PciXSnafu { ptr })?
            }
        }
        0x08 => cap_data
            .try_into()
            .map(Kind::Hypertransport)
            .context(HypertransportSnafu { ptr })?,
        0x09 => VendorSpecific::try_new(cap_data, header)
            .map(Kind::VendorSpecific)
            .context(VendorSpecificSnafu { ptr })?,
        0x0a => cap_data
            .try_into()
            .map(Kind::DebugPort)
            .context(DataSnafu { ptr })?,
        0x0b => Kind::CompactPciResourceControl(CompactPciResourceControl),
        0x0c => Kind::PciHotPlug(PciHotPlug),
        0x0d => cap_data
            .try_into()
            .map(Kind::BridgeSubsystemVendorId)
            .context(DataSnafu { ptr })?,
        0x0e => Kind::Agp8x(Agp8x),
        0x0f => Kind::SecureDevice(SecureDevice),
        0x10 => cap_data
            .try_into()
            .map(Kind::PciExpress)
            .context(PciExpressSnafu { ptr })?,
        0x11 => cap_data
            .try_into()
            .map(Kind::MsiX)
            .context(DataSnafu { ptr })?,
        0x12 => cap_data
            .try_into()
            .map(Kind::Sata)
            .context(DataSnafu { ptr })?,
        0x13 => cap_data
            .try_into()
            .map(Kind::AdvancedFeatures)
            .context(DataSnafu { ptr })?,
        0x14 => EnhancedAllocation::try_new(cap_data, header)
            .map(Kind::EnhancedAllocation)
            .context(EnhancedAllocationSnafu { ptr })?,
        0x15 => cap_data
            .try_into()
            .map(Kind::FlatteningPortalBridge)
            .context(DataSnafu { ptr })?,
        v => Kind::Reserved(v),
    };
    Ok(Capability { pointer: ptr, kind })
}





/// Capability structure
#[derive(Debug, PartialEq, Eq)]
pub struct Capability<'a> {
    pub pointer: u8,
    pub kind: CapabilityKind<'a>,
}
impl<'a> Capability<'a> {
    /// Each capability in the capability list consists of an 8-bit ID field assigned by the PCI
    /// SIG, an 8 bit pointer in configuration space to the next capability.
    pub const HEADER_SIZE: usize = 2;
}

/// Capability ID assigned by the PCI-SIG
#[derive(Debug, PartialEq, Eq)]
pub enum CapabilityKind<'a> {
    /// Null Capability (00h)
    ///
    /// This capability contains no registers. It may be present in any Function. Functions may
    /// contain multiple instances of this capability.
    NullCapability,
    PowerManagementInterface(PowerManagementInterface),
    AcceleratedGraphicsPort(AcceleratedGraphicsPort),
    VitalProductData(VitalProductData),
    SlotIdentification(SlotIdentification),
    MessageSignaledInterrups(MessageSignaledInterrups),
    CompactPciHotSwap(CompactPciHotSwap),
    PciX(PciX),
    PciXBridge(PciXBridge),
    Hypertransport(Hypertransport),
    VendorSpecific(VendorSpecific<'a>),
    DebugPort(DebugPort),
    CompactPciResourceControl(CompactPciResourceControl),
    PciHotPlug(PciHotPlug),
    BridgeSubsystemVendorId(BridgeSubsystemVendorId),
    Agp8x(Agp8x),
    SecureDevice(SecureDevice),
    PciExpress(PciExpress),
    MsiX(MsiX),
    Sata(Sata),
    AdvancedFeatures(AdvancedFeatures),
    EnhancedAllocation(EnhancedAllocation<'a>),
    FlatteningPortalBridge(FlatteningPortalBridge),
    Reserved(u8),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ECS_OFFSET;
    use pretty_assertions::assert_eq;
    use std::prelude::v1::*;

    #[test]
    fn capabilities() {
        // Capabilities: [50] Power Management version 3
        //         Flags: PMEClk- DSI- D1- D2- AuxCurrent=55mA PME(D0-,D1-,D2-,D3hot+,D3cold+)
        //         Status: D0 NoSoftRst+ PME-Enable- DSel=0 DScale=0 PME-
        // Capabilities: [80] Vendor Specific Information: Len=14 <?>
        // Capabilities: [60] MSI: Enable+ Count=1/1 Maskable- 64bit+
        //         Address: 00000000fee00578  Data: 0000
        let data = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/data/device/8086_9dc8/config"
        ));
        let header = data.as_slice().try_into().unwrap();
        let ddr = &data[DDR_OFFSET..ECS_OFFSET];
        let result = Capabilities::new(ddr, &header).collect::<Vec<_>>();
        let sample = vec![
            Ok(Capability {
                pointer: 0x50,
                kind: CapabilityKind::PowerManagementInterface(
                    data[0x50 + 2 .. ].try_into().unwrap(),
                ),
            }),
            Ok(Capability {
                pointer: 0x80,
                kind: VendorSpecific::try_new(&data[(0x80 + 2)..], &header)
                    .map(CapabilityKind::VendorSpecific)
                    .unwrap(),
            }),
            Ok(Capability {
                pointer: 0x60,
                kind: CapabilityKind::MessageSignaledInterrups(
                    data[(0x60 + 2)..].try_into().unwrap(),
                ),
            }),
        ];
        assert_eq!(sample, result);
    }
}
