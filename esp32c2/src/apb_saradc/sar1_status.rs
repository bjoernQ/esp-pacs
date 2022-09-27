#[doc = "Register `SAR1_STATUS` reader"]
pub struct R(crate::R<SAR1_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR1_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR1_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR1_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SARADC_SAR1_STATUS` reader - Need add description"]
pub type SARADC_SAR1_STATUS_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Need add description"]
    #[inline(always)]
    pub fn saradc_sar1_status(&self) -> SARADC_SAR1_STATUS_R {
        SARADC_SAR1_STATUS_R::new(self.bits)
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar1_status](index.html) module"]
pub struct SAR1_STATUS_SPEC;
impl crate::RegisterSpec for SAR1_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar1_status::R](R) reader structure"]
impl crate::Readable for SAR1_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SAR1_STATUS to value 0"]
impl crate::Resettable for SAR1_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}