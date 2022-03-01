#[doc = "Register `RD_MAC_SPI_SYS_1` reader"]
pub struct R(crate::R<RD_MAC_SPI_SYS_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_MAC_SPI_SYS_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_MAC_SPI_SYS_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_MAC_SPI_SYS_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MAC_1` reader - Stores the high 16 bits of MAC address."]
pub struct MAC_1_R(crate::FieldReader<u16, u16>);
impl MAC_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MAC_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAC_1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_PAD_CONF_0` reader - Stores the zeroth part of SPI_PAD_CONF."]
pub struct SPI_PAD_CONF_0_R(crate::FieldReader<u16, u16>);
impl SPI_PAD_CONF_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SPI_PAD_CONF_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_PAD_CONF_0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Stores the high 16 bits of MAC address."]
    #[inline(always)]
    pub fn mac_1(&self) -> MAC_1_R {
        MAC_1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Stores the zeroth part of SPI_PAD_CONF."]
    #[inline(always)]
    pub fn spi_pad_conf_0(&self) -> SPI_PAD_CONF_0_R {
        SPI_PAD_CONF_0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Register 1 of BLOCK1.\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_mac_spi_sys_1]
(index.html) module"]
pub struct RD_MAC_SPI_SYS_1_SPEC;
impl crate::RegisterSpec for RD_MAC_SPI_SYS_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_mac_spi_sys_1::R]
(R) reader structure"]
impl crate::Readable for RD_MAC_SPI_SYS_1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_MAC_SPI_SYS_1 to value 0"]
impl crate::Resettable for RD_MAC_SPI_SYS_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}