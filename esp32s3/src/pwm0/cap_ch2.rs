#[doc = "Register `CAP_CH2` reader"]
pub struct R(crate::R<CAP_CH2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAP_CH2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAP_CH2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAP_CH2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CAP2_VALUE` reader - Value of last capture on channel 2"]
pub struct CAP2_VALUE_R(crate::FieldReader<u32, u32>);
impl CAP2_VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CAP2_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP2_VALUE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Value of last capture on channel 2"]
    #[inline(always)]
    pub fn cap2_value(&self) -> CAP2_VALUE_R {
        CAP2_VALUE_R::new(self.bits)
    }
}
#[doc = "Value of last capture on channel 2\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap_ch2]
(index.html) module"]
pub struct CAP_CH2_SPEC;
impl crate::RegisterSpec for CAP_CH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cap_ch2::R]
(R) reader structure"]
impl crate::Readable for CAP_CH2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CAP_CH2 to value 0"]
impl crate::Resettable for CAP_CH2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}