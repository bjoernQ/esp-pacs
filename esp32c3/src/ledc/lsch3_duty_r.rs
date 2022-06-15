#[doc = "Register `LSCH3_DUTY_R` reader"]
pub struct R(crate::R<LSCH3_DUTY_R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSCH3_DUTY_R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSCH3_DUTY_R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSCH3_DUTY_R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DUTY_LSCH3_R` reader - reg_duty_lsch3_r."]
pub type DUTY_LSCH3_R_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:18 - reg_duty_lsch3_r."]
    #[inline(always)]
    pub fn duty_lsch3_r(&self) -> DUTY_LSCH3_R_R {
        DUTY_LSCH3_R_R::new((self.bits & 0x0007_ffff) as u32)
    }
}
#[doc = "LEDC_LSCH3_DUTY_R.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch3_duty_r](index.html) module"]
pub struct LSCH3_DUTY_R_SPEC;
impl crate::RegisterSpec for LSCH3_DUTY_R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lsch3_duty_r::R](R) reader structure"]
impl crate::Readable for LSCH3_DUTY_R_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LSCH3_DUTY_R to value 0"]
impl crate::Resettable for LSCH3_DUTY_R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
