use heterob::{endianness::Le, P3};


#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ClassCode {
    /// Specific register-level programming interface (if any) so that device independent software
    /// can interact with the device
    pub interface: u8, 
    /// Sub-class code which identifies more specifically the function of the device
    pub sub: u8, 
    /// Base class code which broadly classifies the type of function the device performs
    pub base: u8,
}

impl From<[u8; 3]> for ClassCode {
    fn from(bytes: [u8; 3]) -> Self {
        let Le((interface, sub, base)) = P3(bytes).into();
        Self {
            interface,
            sub,
            base,
        }
    }
}

impl ClassCode {
    pub fn meaning(&self) -> (&str, Option<&str>, Option<&str>) {
        match (self.base, self.sub, self.interface) {
            (0x00, 0x00,    _) => ("Unclassified device", Some("Non-VGA unclassified device"), None),
            (0x00, 0x01,    _) => ("Unclassified device", Some("VGA compatible unclassified device"), None),
            (0x00, 0x05,    _) => ("Unclassified device", Some("Image coprocessor"), None),
            (0x00,    _,    _) => ("Unclassified device", None, None),
            (0x01, 0x00,    _) => ("Mass storage controller", Some("SCSI storage controller"), None),
            (0x01, 0x01, 0x00) => ("Mass storage controller", Some("IDE interface"), Some("ISA Compatibility mode-only controller")),
            (0x01, 0x01, 0x05) => ("Mass storage controller", Some("IDE interface"), Some("PCI native mode-only controller")),
            (0x01, 0x01, 0x0A) => ("Mass storage controller", Some("IDE interface"), Some("ISA Compatibility mode controller, supports both channels switched to PCI native mode")),
            (0x01, 0x01, 0x0F) => ("Mass storage controller", Some("IDE interface"), Some("PCI native mode controller, supports both channels switched to ISA compatibility mode")),
            (0x01, 0x01, 0x80) => ("Mass storage controller", Some("IDE interface"), Some("ISA Compatibility mode-only controller, supports bus mastering")),
            (0x01, 0x01, 0x85) => ("Mass storage controller", Some("IDE interface"), Some("PCI native mode-only controller, supports bus mastering")),
            (0x01, 0x01, 0x8A) => ("Mass storage controller", Some("IDE interface"), Some("ISA Compatibility mode controller, supports both channels switched to PCI native mode, supports bus mastering")),
            (0x01, 0x01, 0x8F) => ("Mass storage controller", Some("IDE interface"), Some("PCI native mode controller, supports both channels switched to ISA compatibility mode, supports bus mastering")),
            (0x01, 0x01,    _) => ("Mass storage controller", Some("IDE interface"), None),
            (0x01, 0x02,    _) => ("Mass storage controller", Some("Floppy disk controller"), None),
            (0x01, 0x03,    _) => ("Mass storage controller", Some("IPI bus controller"), None),
            (0x01, 0x04,    _) => ("Mass storage controller", Some("RAID bus controller"), None),
            (0x01, 0x05, 0x20) => ("Mass storage controller", Some("ATA controller"), Some("ADMA single stepping")),
            (0x01, 0x05, 0x30) => ("Mass storage controller", Some("ATA controller"), Some("ADMA continuous operation")),
            (0x01, 0x05,    _) => ("Mass storage controller", Some("ATA controller"), None),
            (0x01, 0x06, 0x00) => ("Mass storage controller", Some("SATA controller"), Some("Vendor specific")),
            (0x01, 0x06, 0x01) => ("Mass storage controller", Some("SATA controller"), Some("AHCI 1.0")),
            (0x01, 0x06, 0x02) => ("Mass storage controller", Some("SATA controller"), Some("Serial Storage Bus")),
            (0x01, 0x06,    _) => ("Mass storage controller", Some("SATA controller"), None),
            (0x01, 0x07, 0x01) => ("Mass storage controller", Some("Serial Attached SCSI controller"), Some("Serial Storage Bus")),
            (0x01, 0x07,    _) => ("Mass storage controller", Some("Serial Attached SCSI controller"), None),
            (0x01, 0x08, 0x01) => ("Mass storage controller", Some("Non-Volatile memory controller"), Some("NVMHCI")),
            (0x01, 0x08, 0x02) => ("Mass storage controller", Some("Non-Volatile memory controller"), Some("NVM Express")),
            (0x01, 0x08,    _) => ("Mass storage controller", Some("Non-Volatile memory controller"), None),
            (0x01, 0x80,    _) => ("Mass storage controller", Some("Mass storage controller"), None),
            (0x01,    _,    _) => ("Mass storage controller", None, None),
            (0x02, 0x00,    _) => ("Network controller", Some("Ethernet controller"), None),
            (0x02, 0x01,    _) => ("Network controller", Some("Token ring network controller"), None),
            (0x02, 0x02,    _) => ("Network controller", Some("FDDI network controller"), None),
            (0x02, 0x03,    _) => ("Network controller", Some("ATM network controller"), None),
            (0x02, 0x04,    _) => ("Network controller", Some("ISDN controller"), None),
            (0x02, 0x05,    _) => ("Network controller", Some("WorldFip controller"), None),
            (0x02, 0x06,    _) => ("Network controller", Some("PICMG controller"), None),
            (0x02, 0x07,    _) => ("Network controller", Some("Infiniband controller"), None),
            (0x02, 0x08,    _) => ("Network controller", Some("Fabric controller"), None),
            (0x02, 0x80,    _) => ("Network controller", Some("Network controller"), None),
            (0x02,    _,    _) => ("Network controller", None, None),
            (0x03, 0x00, 0x00) => ("Display controller", Some("VGA compatible controller"), Some("VGA controller")),
            (0x03, 0x00, 0x01) => ("Display controller", Some("VGA compatible controller"), Some("8514 controller")),
            (0x03, 0x00,    _) => ("Display controller", Some("VGA compatible controller"), None),
            (0x03, 0x01,    _) => ("Display controller", Some("XGA compatible controller"), None),
            (0x03, 0x02,    _) => ("Display controller", Some("3D controller"), None),
            (0x03, 0x80,    _) => ("Display controller", Some("Display controller"), None),
            (0x03,    _,    _) => ("Display controller", None, None),
            (0x04, 0x00,    _) => ("Multimedia controller", Some("Multimedia video controller"), None),
            (0x04, 0x01,    _) => ("Multimedia controller", Some("Multimedia audio controller"), None),
            (0x04, 0x02,    _) => ("Multimedia controller", Some("Computer telephony device"), None),
            (0x04, 0x03,    _) => ("Multimedia controller", Some("Audio device"), None),
            (0x04, 0x80,    _) => ("Multimedia controller", Some("Multimedia controller"), None),
            (0x04,    _,    _) => ("Multimedia controller", None, None),
            (0x05, 0x00,    _) => ("Memory controller", Some("RAM memory"), None),
            (0x05, 0x01,    _) => ("Memory controller", Some("FLASH memory"), None),
            (0x05, 0x80,    _) => ("Memory controller", Some("Memory controller"), None),
            (0x05,    _,    _) => ("Memory controller", None, None),
            (0x06, 0x00,    _) => ("Bridge", Some("Host bridge"), None),
            (0x06, 0x01,    _) => ("Bridge", Some("ISA bridge"), None),
            (0x06, 0x02,    _) => ("Bridge", Some("EISA bridge"), None),
            (0x06, 0x03,    _) => ("Bridge", Some("MicroChannel bridge"), None),
            (0x06, 0x04, 0x00) => ("Bridge", Some("PCI bridge"), Some("Normal decode")),
            (0x06, 0x04, 0x01) => ("Bridge", Some("PCI bridge"), Some("Subtractive decode")),
            (0x06, 0x04,    _) => ("Bridge", Some("PCI bridge"), None),
            (0x06, 0x05,    _) => ("Bridge", Some("PCMCIA bridge"), None),
            (0x06, 0x06,    _) => ("Bridge", Some("NuBus bridge"), None),
            (0x06, 0x07,    _) => ("Bridge", Some("CardBus bridge"), None),
            (0x06, 0x08, 0x00) => ("Bridge", Some("RACEway bridge"), Some("Transparent mode")),
            (0x06, 0x08, 0x01) => ("Bridge", Some("RACEway bridge"), Some("Endpoint mode")),
            (0x06, 0x08,    _) => ("Bridge", Some("RACEway bridge"), None),
            (0x06, 0x09, 0x40) => ("Bridge", Some("Semi-transparent PCI-to-PCI bridge"), Some("Primary bus towards host CPU")),
            (0x06, 0x09, 0x80) => ("Bridge", Some("Semi-transparent PCI-to-PCI bridge"), Some("Secondary bus towards host CPU")),
            (0x06, 0x09,    _) => ("Bridge", Some("Semi-transparent PCI-to-PCI bridge"), None),
            (0x06, 0x0A,    _) => ("Bridge", Some("InfiniBand to PCI host bridge"), None),
            (0x06, 0x80,    _) => ("Bridge", Some("Bridge"), None),
            (0x06,    _,    _) => ("Bridge", None, None),
            (0x07, 0x00, 0x00) => ("Communication controller", Some("Serial controller"), Some("8250")),
            (0x07, 0x00, 0x01) => ("Communication controller", Some("Serial controller"), Some("16450")),
            (0x07, 0x00, 0x02) => ("Communication controller", Some("Serial controller"), Some("16550")),
            (0x07, 0x00, 0x03) => ("Communication controller", Some("Serial controller"), Some("16650")),
            (0x07, 0x00, 0x04) => ("Communication controller", Some("Serial controller"), Some("16750")),
            (0x07, 0x00, 0x05) => ("Communication controller", Some("Serial controller"), Some("16850")),
            (0x07, 0x00, 0x06) => ("Communication controller", Some("Serial controller"), Some("16950")),
            (0x07, 0x00,    _) => ("Communication controller", Some("Serial controller"), None),
            (0x07, 0x01, 0x00) => ("Communication controller", Some("Parallel controller"), Some("SPP")),
            (0x07, 0x01, 0x01) => ("Communication controller", Some("Parallel controller"), Some("BiDir")),
            (0x07, 0x01, 0x02) => ("Communication controller", Some("Parallel controller"), Some("ECP")),
            (0x07, 0x01, 0x03) => ("Communication controller", Some("Parallel controller"), Some("IEEE1284")),
            (0x07, 0x01, 0xFE) => ("Communication controller", Some("Parallel controller"), Some("IEEE1284 Target")),
            (0x07, 0x01,    _) => ("Communication controller", Some("Parallel controller"), None),
            (0x07, 0x02,    _) => ("Communication controller", Some("Multiport serial controller"), None),
            (0x07, 0x03, 0x00) => ("Communication controller", Some("Modem"), Some("Generic")),
            (0x07, 0x03, 0x01) => ("Communication controller", Some("Modem"), Some("Hayes/16450")),
            (0x07, 0x03, 0x02) => ("Communication controller", Some("Modem"), Some("Hayes/16550")),
            (0x07, 0x03, 0x03) => ("Communication controller", Some("Modem"), Some("Hayes/16650")),
            (0x07, 0x03, 0x04) => ("Communication controller", Some("Modem"), Some("Hayes/16750")),
            (0x07, 0x03,    _) => ("Communication controller", Some("Modem"), None),
            (0x07, 0x04,    _) => ("Communication controller", Some("GPIB controller"), None),
            (0x07, 0x05,    _) => ("Communication controller", Some("Smard Card controller"), None),
            (0x07, 0x80,    _) => ("Communication controller", Some("Communication controller"), None),
            (0x07,    _,    _) => ("Communication controller", None, None),
            (0x08, 0x00, 0x00) => ("Generic system peripheral", Some("PIC"), Some("8259")),
            (0x08, 0x00, 0x01) => ("Generic system peripheral", Some("PIC"), Some("ISA PIC")),
            (0x08, 0x00, 0x02) => ("Generic system peripheral", Some("PIC"), Some("EISA PIC")),
            (0x08, 0x00, 0x10) => ("Generic system peripheral", Some("PIC"), Some("IO-APIC")),
            (0x08, 0x00, 0x20) => ("Generic system peripheral", Some("PIC"), Some("IO(X)-APIC")),
            (0x08, 0x00,    _) => ("Generic system peripheral", Some("PIC"), None),
            (0x08, 0x01, 0x00) => ("Generic system peripheral", Some("DMA controller"), Some("8237")),
            (0x08, 0x01, 0x01) => ("Generic system peripheral", Some("DMA controller"), Some("ISA DMA")),
            (0x08, 0x01, 0x02) => ("Generic system peripheral", Some("DMA controller"), Some("EISA DMA")),
            (0x08, 0x01,    _) => ("Generic system peripheral", Some("DMA controller"), None),
            (0x08, 0x02, 0x00) => ("Generic system peripheral", Some("Timer"), Some("8254")),
            (0x08, 0x02, 0x01) => ("Generic system peripheral", Some("Timer"), Some("ISA Timer")),
            (0x08, 0x02, 0x02) => ("Generic system peripheral", Some("Timer"), Some("EISA Timers")),
            (0x08, 0x02, 0x03) => ("Generic system peripheral", Some("Timer"), Some("HPET")),
            (0x08, 0x02,    _) => ("Generic system peripheral", Some("Timer"), None),
            (0x08, 0x03, 0x00) => ("Generic system peripheral", Some("RTC"), Some("Generic")),
            (0x08, 0x03, 0x01) => ("Generic system peripheral", Some("RTC"), Some("ISA RTC")),
            (0x08, 0x03,    _) => ("Generic system peripheral", Some("RTC"), None),
            (0x08, 0x04,    _) => ("Generic system peripheral", Some("PCI Hot-plug controller"), None),
            (0x08, 0x05,    _) => ("Generic system peripheral", Some("SD Host controller"), None),
            (0x08, 0x06,    _) => ("Generic system peripheral", Some("IOMMU"), None),
            (0x08, 0x80,    _) => ("Generic system peripheral", Some("System peripheral"), None),
            (0x08, 0x99, 0x01) => ("Generic system peripheral", Some("Timing Card"), Some("TAP Timing Card")),
            (0x08, 0x99,    _) => ("Generic system peripheral", Some("Timing Card"), None),
            (0x08,    _,    _) => ("Generic system peripheral", None, None),
            (0x09, 0x00,    _) => ("Input device controller", Some("Keyboard controller"), None),
            (0x09, 0x01,    _) => ("Input device controller", Some("Digitizer Pen"), None),
            (0x09, 0x02,    _) => ("Input device controller", Some("Mouse controller"), None),
            (0x09, 0x03,    _) => ("Input device controller", Some("Scanner controller"), None),
            (0x09, 0x04, 0x00) => ("Input device controller", Some("Gameport controller"), Some("Generic")),
            (0x09, 0x04, 0x01) => ("Input device controller", Some("Gameport controller"), Some("Extended")),
            (0x09, 0x04,    _) => ("Input device controller", Some("Gameport controller"), None),
            (0x09, 0x80,    _) => ("Input device controller", Some("Input device controller"), None),
            (0x09,    _,    _) => ("Input device controller", None, None),
            (0x0A, 0x00,    _) => ("Docking station", Some("Generic Docking Station"), None),
            (0x0A, 0x80,    _) => ("Docking station", Some("Docking Station"), None),
            (0x0A,    _,    _) => ("Docking station", None, None),
            (0x0B, 0x00,    _) => ("Processor", Some("386"), None),
            (0x0B, 0x01,    _) => ("Processor", Some("486"), None),
            (0x0B, 0x02,    _) => ("Processor", Some("Pentium"), None),
            (0x0B, 0x10,    _) => ("Processor", Some("Alpha"), None),
            (0x0B, 0x20,    _) => ("Processor", Some("Power PC"), None),
            (0x0B, 0x30,    _) => ("Processor", Some("MIPS"), None),
            (0x0B, 0x40,    _) => ("Processor", Some("Co-processor"), None),
            (0x0B,    _,    _) => ("Processor", None, None),
            (0x0C, 0x00, 0x00) => ("Serial bus controller", Some("FireWire (IEEE 1394)"), Some("Generic")),
            (0x0C, 0x00, 0x10) => ("Serial bus controller", Some("FireWire (IEEE 1394)"), Some("OHCI")),
            (0x0C, 0x00,    _) => ("Serial bus controller", Some("FireWire (IEEE 1394)"), None),
            (0x0C, 0x01,    _) => ("Serial bus controller", Some("ACCESS Bus"), None),
            (0x0C, 0x02,    _) => ("Serial bus controller", Some("SSA"), None),
            (0x0C, 0x03, 0x00) => ("Serial bus controller", Some("USB controller"), Some("UHCI")),
            (0x0C, 0x03, 0x10) => ("Serial bus controller", Some("USB controller"), Some("OHCI")),
            (0x0C, 0x03, 0x20) => ("Serial bus controller", Some("USB controller"), Some("EHCI")),
            (0x0C, 0x03, 0x30) => ("Serial bus controller", Some("USB controller"), Some("XHCI")),
            (0x0C, 0x03, 0x40) => ("Serial bus controller", Some("USB controller"), Some("USB4 Host Interface")),
            (0x0C, 0x03, 0x80) => ("Serial bus controller", Some("USB controller"), Some("Unspecified")),
            (0x0C, 0x03, 0xFE) => ("Serial bus controller", Some("USB controller"), Some("USB Device")),
            (0x0C, 0x03,    _) => ("Serial bus controller", Some("USB controller"), None),
            (0x0C, 0x04,    _) => ("Serial bus controller", Some("Fibre Channel"), None),
            (0x0C, 0x05,    _) => ("Serial bus controller", Some("SMBus"), None),
            (0x0C, 0x06,    _) => ("Serial bus controller", Some("InfiniBand"), None),
            (0x0C, 0x07, 0x00) => ("Serial bus controller", Some("IPMI Interface"), Some("SMIC")),
            (0x0C, 0x07, 0x01) => ("Serial bus controller", Some("IPMI Interface"), Some("KCS")),
            (0x0C, 0x07, 0x02) => ("Serial bus controller", Some("IPMI Interface"), Some("BT (Block Transfer)")),
            (0x0C, 0x07,    _) => ("Serial bus controller", Some("IPMI Interface"), None),
            (0x0C, 0x08,    _) => ("Serial bus controller", Some("SERCOS interface"), None),
            (0x0C, 0x09,    _) => ("Serial bus controller", Some("CANBUS"), None),
            (0x0C,    _,    _) => ("Serial bus controller", None, None),
            (0x0D, 0x00,    _) => ("Wireless controller", Some("IRDA controller"), None),
            (0x0D, 0x01,    _) => ("Wireless controller", Some("Consumer IR controller"), None),
            (0x0D, 0x10,    _) => ("Wireless controller", Some("RF controller"), None),
            (0x0D, 0x11,    _) => ("Wireless controller", Some("Bluetooth"), None),
            (0x0D, 0x12,    _) => ("Wireless controller", Some("Broadband"), None),
            (0x0D, 0x20,    _) => ("Wireless controller", Some("802.1a controller"), None),
            (0x0D, 0x21,    _) => ("Wireless controller", Some("802.1b controller"), None),
            (0x0D, 0x80,    _) => ("Wireless controller", Some("Wireless controller"), None),
            (0x0D,    _,    _) => ("Wireless controller", None, None),
            (0x0E, 0x00,    _) => ("Intelligent controller", Some("I2O"), None),
            (0x0E,    _,    _) => ("Intelligent controller", None, None),
            (0x0F, 0x01,    _) => ("Satellite communications controller", Some("Satellite TV controller"), None),
            (0x0F, 0x02,    _) => ("Satellite communications controller", Some("Satellite audio communication controller"), None),
            (0x0F, 0x03,    _) => ("Satellite communications controller", Some("Satellite voice communication controller"), None),
            (0x0F, 0x04,    _) => ("Satellite communications controller", Some("Satellite data communication controller"), None),
            (0x0F,    _,    _) => ("Satellite communications controller", None, None),
            (0x10, 0x00,    _) => ("Encryption controller", Some("Network and computing encryption device"), None),
            (0x10, 0x10,    _) => ("Encryption controller", Some("Entertainment encryption device"), None),
            (0x10, 0x80,    _) => ("Encryption controller", Some("Encryption controller"), None),
            (0x10,    _,    _) => ("Encryption controller", None, None),
            (0x11, 0x00,    _) => ("Signal processing controller", Some("DPIO module"), None),
            (0x11, 0x01,    _) => ("Signal processing controller", Some("Performance counters"), None),
            (0x11, 0x10,    _) => ("Signal processing controller", Some("Communication synchronizer"), None),
            (0x11, 0x20,    _) => ("Signal processing controller", Some("Signal processing management"), None),
            (0x11, 0x80,    _) => ("Signal processing controller", Some("Signal processing controller"), None),
            (0x11,    _,    _) => ("Signal processing controller", None, None),
            (0x12, 0x00,    _) => ("Processing accelerators", Some("Processing accelerators"), None),
            (0x12, 0x01,    _) => ("Processing accelerators", Some("AI Inference Accelerator"), None),
            (0x12,    _,    _) => ("Processing accelerators", None, None),
            (0x13,    _,    _) => ("Non-Essential Instrumentation", None, None),
            (0x40,    _,    _) => ("Coprocessor", None, None),
            (0xFF,    _,    _) => ("Unassigned class", None, None),
            (_,    _,    _) => ("Reserved", None, None),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn meaning() {
        let data = [0x00, 0x00, 0x05];
        let result: ClassCode = data.try_into().unwrap();
        let result = result.meaning();
        assert_eq!(("Memory controller", Some("RAM memory"), None), result);
    }
}
