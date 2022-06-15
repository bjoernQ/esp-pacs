#[doc = "Register `LSTIMER0_VALUE` reader"]
pub struct R(crate::R<LSTIMER0_VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSTIMER0_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSTIMER0_VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSTIMER0_VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LSTIMER0_CNT` reader - reg_lstimer0_cnt."]
pub type LSTIMER0_CNT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:13 - reg_lstimer0_cnt."]
    #[inline(always)]
    pub fn lstimer0_cnt(&self) -> LSTIMER0_CNT_R {
        LSTIMER0_CNT_R::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "LEDC_LSTIMER0_VALUE.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lstimer0_value](index.html) module"]
pub struct LSTIMER0_VALUE_SPEC;
impl crate::RegisterSpec for LSTIMER0_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lstimer0_value::R](R) reader structure"]
impl crate::Readable for LSTIMER0_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LSTIMER0_VALUE to value 0"]
impl crate::Resettable for LSTIMER0_VALUE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
