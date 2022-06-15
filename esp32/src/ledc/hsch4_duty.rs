#[doc = "Register `HSCH4_DUTY` reader"]
pub struct R(crate::R<HSCH4_DUTY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSCH4_DUTY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSCH4_DUTY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSCH4_DUTY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DUTY_HSCH4` reader - This register represents the current duty of the output signal for high speed channel4."]
pub type DUTY_HSCH4_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:24 - This register represents the current duty of the output signal for high speed channel4."]
    #[inline(always)]
    pub fn duty_hsch4(&self) -> DUTY_HSCH4_R {
        DUTY_HSCH4_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsch4_duty](index.html) module"]
pub struct HSCH4_DUTY_SPEC;
impl crate::RegisterSpec for HSCH4_DUTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsch4_duty::R](R) reader structure"]
impl crate::Readable for HSCH4_DUTY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HSCH4_DUTY to value 0"]
impl crate::Resettable for HSCH4_DUTY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
