#[doc = "Register `RD_REPEAT_ERR0` reader"]
pub struct R(crate::R<RD_REPEAT_ERR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_REPEAT_ERR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_REPEAT_ERR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_REPEAT_ERR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RD_DIS_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_RD_DIS."]
pub struct RD_DIS_ERR_R(crate::FieldReader<u8, u8>);
impl RD_DIS_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RD_DIS_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_DIS_ERR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_RTC_RAM_BOOT_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_DIS_RTC_RAM_BOOT."]
pub struct DIS_RTC_RAM_BOOT_ERR_R(crate::FieldReader<bool, bool>);
impl DIS_RTC_RAM_BOOT_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIS_RTC_RAM_BOOT_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_RTC_RAM_BOOT_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_ICACHE_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_DIS_ICACHE."]
pub struct DIS_ICACHE_ERR_R(crate::FieldReader<bool, bool>);
impl DIS_ICACHE_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIS_ICACHE_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_ICACHE_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_DCACHE_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_DIS_DCACHE."]
pub struct DIS_DCACHE_ERR_R(crate::FieldReader<bool, bool>);
impl DIS_DCACHE_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIS_DCACHE_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_DCACHE_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_DOWNLOAD_ICACHE_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_DIS_DOWNLOAD_ICACHE."]
pub struct DIS_DOWNLOAD_ICACHE_ERR_R(crate::FieldReader<bool, bool>);
impl DIS_DOWNLOAD_ICACHE_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIS_DOWNLOAD_ICACHE_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_DOWNLOAD_ICACHE_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_DOWNLOAD_DCACHE_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_DIS_DOWNLOAD_DCACHE."]
pub struct DIS_DOWNLOAD_DCACHE_ERR_R(crate::FieldReader<bool, bool>);
impl DIS_DOWNLOAD_DCACHE_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIS_DOWNLOAD_DCACHE_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_DOWNLOAD_DCACHE_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_FORCE_DOWNLOAD_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_DIS_FORCE_DOWNLOAD."]
pub struct DIS_FORCE_DOWNLOAD_ERR_R(crate::FieldReader<bool, bool>);
impl DIS_FORCE_DOWNLOAD_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIS_FORCE_DOWNLOAD_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_FORCE_DOWNLOAD_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_USB_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_DIS_USB."]
pub struct DIS_USB_ERR_R(crate::FieldReader<bool, bool>);
impl DIS_USB_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIS_USB_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_USB_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_CAN_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_DIS_CAN."]
pub struct DIS_CAN_ERR_R(crate::FieldReader<bool, bool>);
impl DIS_CAN_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIS_CAN_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_CAN_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_BOOT_REMAP_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_DIS_BOOT_REMAP."]
pub struct DIS_BOOT_REMAP_ERR_R(crate::FieldReader<bool, bool>);
impl DIS_BOOT_REMAP_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIS_BOOT_REMAP_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_BOOT_REMAP_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPT4_RESERVED5_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_RPT4_RESERVED5."]
pub struct RPT4_RESERVED5_ERR_R(crate::FieldReader<bool, bool>);
impl RPT4_RESERVED5_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RPT4_RESERVED5_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPT4_RESERVED5_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFT_DIS_JTAG_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_SOFT_DIS_JTAG."]
pub struct SOFT_DIS_JTAG_ERR_R(crate::FieldReader<bool, bool>);
impl SOFT_DIS_JTAG_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SOFT_DIS_JTAG_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFT_DIS_JTAG_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HARD_DIS_JTAG_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_HARD_DIS_JTAG."]
pub struct HARD_DIS_JTAG_ERR_R(crate::FieldReader<bool, bool>);
impl HARD_DIS_JTAG_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HARD_DIS_JTAG_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HARD_DIS_JTAG_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_DIS_DOWNLOAD_MANUAL_ENCRYPT."]
pub struct DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR_R(crate::FieldReader<bool, bool>);
impl DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_DREFH_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_USB_DREFH."]
pub struct USB_DREFH_ERR_R(crate::FieldReader<u8, u8>);
impl USB_DREFH_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USB_DREFH_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_DREFH_ERR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_DREFL_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_USB_DREFL."]
pub struct USB_DREFL_ERR_R(crate::FieldReader<u8, u8>);
impl USB_DREFL_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USB_DREFL_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_DREFL_ERR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_EXCHG_PINS_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_USB_EXCHG_PINS."]
pub struct USB_EXCHG_PINS_ERR_R(crate::FieldReader<bool, bool>);
impl USB_EXCHG_PINS_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB_EXCHG_PINS_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_EXCHG_PINS_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXT_PHY_ENABLE_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_EXT_PHY_ENABLE."]
pub struct EXT_PHY_ENABLE_ERR_R(crate::FieldReader<bool, bool>);
impl EXT_PHY_ENABLE_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXT_PHY_ENABLE_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXT_PHY_ENABLE_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_FORCE_NOPERSIST_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_USB_FORCE_NOPERSIST."]
pub struct USB_FORCE_NOPERSIST_ERR_R(crate::FieldReader<bool, bool>);
impl USB_FORCE_NOPERSIST_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB_FORCE_NOPERSIST_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_FORCE_NOPERSIST_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPT4_RESERVED0_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_RPT4_RESERVED0."]
pub struct RPT4_RESERVED0_ERR_R(crate::FieldReader<u8, u8>);
impl RPT4_RESERVED0_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RPT4_RESERVED0_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPT4_RESERVED0_ERR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDD_SPI_MODECURLIM_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_VDD_SPI_MODECURLIM."]
pub struct VDD_SPI_MODECURLIM_ERR_R(crate::FieldReader<bool, bool>);
impl VDD_SPI_MODECURLIM_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VDD_SPI_MODECURLIM_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDD_SPI_MODECURLIM_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDD_SPI_DREFH_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_VDD_SPI_DREFH."]
pub struct VDD_SPI_DREFH_ERR_R(crate::FieldReader<u8, u8>);
impl VDD_SPI_DREFH_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VDD_SPI_DREFH_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDD_SPI_DREFH_ERR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:6 - Any bit equal to 1 denotes a programming error in EFUSE_RD_DIS."]
    #[inline(always)]
    pub fn rd_dis_err(&self) -> RD_DIS_ERR_R {
        RD_DIS_ERR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Any bit equal to 1 denotes a programming error in EFUSE_DIS_RTC_RAM_BOOT."]
    #[inline(always)]
    pub fn dis_rtc_ram_boot_err(&self) -> DIS_RTC_RAM_BOOT_ERR_R {
        DIS_RTC_RAM_BOOT_ERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Any bit equal to 1 denotes a programming error in EFUSE_DIS_ICACHE."]
    #[inline(always)]
    pub fn dis_icache_err(&self) -> DIS_ICACHE_ERR_R {
        DIS_ICACHE_ERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Any bit equal to 1 denotes a programming error in EFUSE_DIS_DCACHE."]
    #[inline(always)]
    pub fn dis_dcache_err(&self) -> DIS_DCACHE_ERR_R {
        DIS_DCACHE_ERR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Any bit equal to 1 denotes a programming error in EFUSE_DIS_DOWNLOAD_ICACHE."]
    #[inline(always)]
    pub fn dis_download_icache_err(&self) -> DIS_DOWNLOAD_ICACHE_ERR_R {
        DIS_DOWNLOAD_ICACHE_ERR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Any bit equal to 1 denotes a programming error in EFUSE_DIS_DOWNLOAD_DCACHE."]
    #[inline(always)]
    pub fn dis_download_dcache_err(&self) -> DIS_DOWNLOAD_DCACHE_ERR_R {
        DIS_DOWNLOAD_DCACHE_ERR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Any bit equal to 1 denotes a programming error in EFUSE_DIS_FORCE_DOWNLOAD."]
    #[inline(always)]
    pub fn dis_force_download_err(&self) -> DIS_FORCE_DOWNLOAD_ERR_R {
        DIS_FORCE_DOWNLOAD_ERR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Any bit equal to 1 denotes a programming error in EFUSE_DIS_USB."]
    #[inline(always)]
    pub fn dis_usb_err(&self) -> DIS_USB_ERR_R {
        DIS_USB_ERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Any bit equal to 1 denotes a programming error in EFUSE_DIS_CAN."]
    #[inline(always)]
    pub fn dis_can_err(&self) -> DIS_CAN_ERR_R {
        DIS_CAN_ERR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Any bit equal to 1 denotes a programming error in EFUSE_DIS_BOOT_REMAP."]
    #[inline(always)]
    pub fn dis_boot_remap_err(&self) -> DIS_BOOT_REMAP_ERR_R {
        DIS_BOOT_REMAP_ERR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Any bit equal to 1 denotes a programming error in EFUSE_RPT4_RESERVED5."]
    #[inline(always)]
    pub fn rpt4_reserved5_err(&self) -> RPT4_RESERVED5_ERR_R {
        RPT4_RESERVED5_ERR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Any bit equal to 1 denotes a programming error in EFUSE_SOFT_DIS_JTAG."]
    #[inline(always)]
    pub fn soft_dis_jtag_err(&self) -> SOFT_DIS_JTAG_ERR_R {
        SOFT_DIS_JTAG_ERR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Any bit equal to 1 denotes a programming error in EFUSE_HARD_DIS_JTAG."]
    #[inline(always)]
    pub fn hard_dis_jtag_err(&self) -> HARD_DIS_JTAG_ERR_R {
        HARD_DIS_JTAG_ERR_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Any bit equal to 1 denotes a programming error in EFUSE_DIS_DOWNLOAD_MANUAL_ENCRYPT."]
    #[inline(always)]
    pub fn dis_download_manual_encrypt_err(&self) -> DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR_R {
        DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - Any bit equal to 1 denotes a programming error in EFUSE_USB_DREFH."]
    #[inline(always)]
    pub fn usb_drefh_err(&self) -> USB_DREFH_ERR_R {
        USB_DREFH_ERR_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Any bit equal to 1 denotes a programming error in EFUSE_USB_DREFL."]
    #[inline(always)]
    pub fn usb_drefl_err(&self) -> USB_DREFL_ERR_R {
        USB_DREFL_ERR_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bit 24 - Any bit equal to 1 denotes a programming error in EFUSE_USB_EXCHG_PINS."]
    #[inline(always)]
    pub fn usb_exchg_pins_err(&self) -> USB_EXCHG_PINS_ERR_R {
        USB_EXCHG_PINS_ERR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Any bit equal to 1 denotes a programming error in EFUSE_EXT_PHY_ENABLE."]
    #[inline(always)]
    pub fn ext_phy_enable_err(&self) -> EXT_PHY_ENABLE_ERR_R {
        EXT_PHY_ENABLE_ERR_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Any bit equal to 1 denotes a programming error in EFUSE_USB_FORCE_NOPERSIST."]
    #[inline(always)]
    pub fn usb_force_nopersist_err(&self) -> USB_FORCE_NOPERSIST_ERR_R {
        USB_FORCE_NOPERSIST_ERR_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 27:28 - Any bit equal to 1 denotes a programming error in EFUSE_RPT4_RESERVED0."]
    #[inline(always)]
    pub fn rpt4_reserved0_err(&self) -> RPT4_RESERVED0_ERR_R {
        RPT4_RESERVED0_ERR_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bit 29 - Any bit equal to 1 denotes a programming error in EFUSE_VDD_SPI_MODECURLIM."]
    #[inline(always)]
    pub fn vdd_spi_modecurlim_err(&self) -> VDD_SPI_MODECURLIM_ERR_R {
        VDD_SPI_MODECURLIM_ERR_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 30:31 - Any bit equal to 1 denotes a programming error in EFUSE_VDD_SPI_DREFH."]
    #[inline(always)]
    pub fn vdd_spi_drefh_err(&self) -> VDD_SPI_DREFH_ERR_R {
        VDD_SPI_DREFH_ERR_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
#[doc = "Programming error record register 0 of BLOCK0.\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_repeat_err0]
(index.html) module"]
pub struct RD_REPEAT_ERR0_SPEC;
impl crate::RegisterSpec for RD_REPEAT_ERR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_repeat_err0::R]
(R) reader structure"]
impl crate::Readable for RD_REPEAT_ERR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_REPEAT_ERR0 to value 0"]
impl crate::Resettable for RD_REPEAT_ERR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}