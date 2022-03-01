#[doc = "Register `DMA_IN_STATUS` reader"]
pub struct R(crate::R<DMA_IN_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_IN_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_IN_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_IN_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IN_FULL` reader - "]
pub struct IN_FULL_R(crate::FieldReader<bool, bool>);
impl IN_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_EMPTY` reader - "]
pub struct IN_EMPTY_R(crate::FieldReader<bool, bool>);
impl IN_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_ERR_CAUSE` reader - This register stores the errors caused in out link descriptor's data packet."]
pub struct RX_ERR_CAUSE_R(crate::FieldReader<u8, u8>);
impl RX_ERR_CAUSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_ERR_CAUSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_ERR_CAUSE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn in_full(&self) -> IN_FULL_R {
        IN_FULL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn in_empty(&self) -> IN_EMPTY_R {
        IN_EMPTY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - This register stores the errors caused in out link descriptor's data packet."]
    #[inline(always)]
    pub fn rx_err_cause(&self) -> RX_ERR_CAUSE_R {
        RX_ERR_CAUSE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
#[doc = "\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_status]
(index.html) module"]
pub struct DMA_IN_STATUS_SPEC;
impl crate::RegisterSpec for DMA_IN_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_in_status::R]
(R) reader structure"]
impl crate::Readable for DMA_IN_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_IN_STATUS to value 0x02"]
impl crate::Resettable for DMA_IN_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}