#[doc = "Register `RXFIFO_START_ADDR` reader"]
pub struct R(crate::R<RXFIFO_START_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXFIFO_START_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXFIFO_START_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXFIFO_START_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXFIFO_START_ADDR` reader - reg_rxfifo_start_addr."]
pub struct RXFIFO_START_ADDR_R(crate::FieldReader<u32, u32>);
impl RXFIFO_START_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RXFIFO_START_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO_START_ADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - reg_rxfifo_start_addr."]
    #[inline(always)]
    pub fn rxfifo_start_addr(&self) -> RXFIFO_START_ADDR_R {
        RXFIFO_START_ADDR_R::new(self.bits)
    }
}
#[doc = "I2C_RXFIFO_START_ADDR_REG\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfifo_start_addr]
(index.html) module"]
pub struct RXFIFO_START_ADDR_SPEC;
impl crate::RegisterSpec for RXFIFO_START_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxfifo_start_addr::R]
(R) reader structure"]
impl crate::Readable for RXFIFO_START_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXFIFO_START_ADDR to value 0"]
impl crate::Resettable for RXFIFO_START_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}