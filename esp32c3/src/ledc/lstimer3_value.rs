#[doc = "Register `LSTIMER3_VALUE` reader"]
pub struct R(crate::R<LSTIMER3_VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSTIMER3_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSTIMER3_VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSTIMER3_VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LSTIMER3_CNT` reader - reg_lstimer3_cnt."]
pub type LSTIMER3_CNT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:13 - reg_lstimer3_cnt."]
    #[inline(always)]
    pub fn lstimer3_cnt(&self) -> LSTIMER3_CNT_R {
        LSTIMER3_CNT_R::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "LEDC_LSTIMER3_VALUE.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lstimer3_value](index.html) module"]
pub struct LSTIMER3_VALUE_SPEC;
impl crate::RegisterSpec for LSTIMER3_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lstimer3_value::R](R) reader structure"]
impl crate::Readable for LSTIMER3_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LSTIMER3_VALUE to value 0"]
impl crate::Resettable for LSTIMER3_VALUE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
