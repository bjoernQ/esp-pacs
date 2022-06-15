#[doc = "Register `HSCH1_DUTY` reader"]
pub struct R(crate::R<HSCH1_DUTY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSCH1_DUTY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSCH1_DUTY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSCH1_DUTY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DUTY_HSCH1` reader - This register represents the current duty of the output signal for high speed channel1."]
pub type DUTY_HSCH1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:24 - This register represents the current duty of the output signal for high speed channel1."]
    #[inline(always)]
    pub fn duty_hsch1(&self) -> DUTY_HSCH1_R {
        DUTY_HSCH1_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsch1_duty](index.html) module"]
pub struct HSCH1_DUTY_SPEC;
impl crate::RegisterSpec for HSCH1_DUTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsch1_duty::R](R) reader structure"]
impl crate::Readable for HSCH1_DUTY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HSCH1_DUTY to value 0"]
impl crate::Resettable for HSCH1_DUTY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
