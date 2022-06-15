#[doc = "Register `HSCH6_DUTY` reader"]
pub struct R(crate::R<HSCH6_DUTY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSCH6_DUTY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSCH6_DUTY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSCH6_DUTY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DUTY_HSCH6` reader - This register represents the current duty of the output signal for high speed channel6."]
pub type DUTY_HSCH6_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:24 - This register represents the current duty of the output signal for high speed channel6."]
    #[inline(always)]
    pub fn duty_hsch6(&self) -> DUTY_HSCH6_R {
        DUTY_HSCH6_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsch6_duty](index.html) module"]
pub struct HSCH6_DUTY_SPEC;
impl crate::RegisterSpec for HSCH6_DUTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsch6_duty::R](R) reader structure"]
impl crate::Readable for HSCH6_DUTY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HSCH6_DUTY to value 0"]
impl crate::Resettable for HSCH6_DUTY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
