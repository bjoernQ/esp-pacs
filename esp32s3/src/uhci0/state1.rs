#[doc = "Register `STATE1` reader"]
pub struct R(crate::R<STATE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ENCODE_STATE` reader - UHCI encoder status."]
pub type ENCODE_STATE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - UHCI encoder status."]
    #[inline(always)]
    pub fn encode_state(&self) -> ENCODE_STATE_R {
        ENCODE_STATE_R::new((self.bits & 7) as u8)
    }
}
#[doc = "UHCI transmit status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [state1](index.html) module"]
pub struct STATE1_SPEC;
impl crate::RegisterSpec for STATE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [state1::R](R) reader structure"]
impl crate::Readable for STATE1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATE1 to value 0"]
impl crate::Resettable for STATE1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
