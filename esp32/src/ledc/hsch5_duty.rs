#[doc = "Register `HSCH5_DUTY` reader"]
pub struct R(crate::R<HSCH5_DUTY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSCH5_DUTY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSCH5_DUTY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSCH5_DUTY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DUTY_HSCH5` reader - This register represents the current duty of the output signal for high speed channel5."]
pub type DUTY_HSCH5_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:24 - This register represents the current duty of the output signal for high speed channel5."]
    #[inline(always)]
    pub fn duty_hsch5(&self) -> DUTY_HSCH5_R {
        DUTY_HSCH5_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsch5_duty](index.html) module"]
pub struct HSCH5_DUTY_SPEC;
impl crate::RegisterSpec for HSCH5_DUTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsch5_duty::R](R) reader structure"]
impl crate::Readable for HSCH5_DUTY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HSCH5_DUTY to value 0"]
impl crate::Resettable for HSCH5_DUTY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
