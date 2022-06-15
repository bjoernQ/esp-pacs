#[doc = "Register `LSTIMER2_VALUE` reader"]
pub struct R(crate::R<LSTIMER2_VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSTIMER2_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSTIMER2_VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSTIMER2_VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LSTIMER2_CNT` reader - software can read this register to get the current counter value in low speed timer2."]
pub type LSTIMER2_CNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:19 - software can read this register to get the current counter value in low speed timer2."]
    #[inline(always)]
    pub fn lstimer2_cnt(&self) -> LSTIMER2_CNT_R {
        LSTIMER2_CNT_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lstimer2_value](index.html) module"]
pub struct LSTIMER2_VALUE_SPEC;
impl crate::RegisterSpec for LSTIMER2_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lstimer2_value::R](R) reader structure"]
impl crate::Readable for LSTIMER2_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LSTIMER2_VALUE to value 0"]
impl crate::Resettable for LSTIMER2_VALUE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
