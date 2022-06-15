#[doc = "Register `LSCH5_DUTY` reader"]
pub struct R(crate::R<LSCH5_DUTY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSCH5_DUTY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSCH5_DUTY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSCH5_DUTY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DUTY_LSCH5` reader - This register represents the current duty of the output signal for low speed channel5."]
pub type DUTY_LSCH5_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:24 - This register represents the current duty of the output signal for low speed channel5."]
    #[inline(always)]
    pub fn duty_lsch5(&self) -> DUTY_LSCH5_R {
        DUTY_LSCH5_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch5_duty](index.html) module"]
pub struct LSCH5_DUTY_SPEC;
impl crate::RegisterSpec for LSCH5_DUTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lsch5_duty::R](R) reader structure"]
impl crate::Readable for LSCH5_DUTY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LSCH5_DUTY to value 0"]
impl crate::Resettable for LSCH5_DUTY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
