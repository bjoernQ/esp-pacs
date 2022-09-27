#[doc = "Register `SAR_TOUCH_STATUS15` reader"]
pub struct R(crate::R<SAR_TOUCH_STATUS15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_TOUCH_STATUS15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_TOUCH_STATUS15_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_TOUCH_STATUS15_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TOUCH_SLP_DATA` reader - The data of touch sleep pad, depending on the setting of SENS_TOUCH_DATA_SEL."]
pub type TOUCH_SLP_DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TOUCH_SLP_DEBOUNCE` reader - Touch sleep pad debouce value."]
pub type TOUCH_SLP_DEBOUNCE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:21 - The data of touch sleep pad, depending on the setting of SENS_TOUCH_DATA_SEL."]
    #[inline(always)]
    pub fn touch_slp_data(&self) -> TOUCH_SLP_DATA_R {
        TOUCH_SLP_DATA_R::new((self.bits & 0x003f_ffff) as u32)
    }
    #[doc = "Bits 29:31 - Touch sleep pad debouce value."]
    #[inline(always)]
    pub fn touch_slp_debounce(&self) -> TOUCH_SLP_DEBOUNCE_R {
        TOUCH_SLP_DEBOUNCE_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[doc = "Touch sleep pad status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_touch_status15](index.html) module"]
pub struct SAR_TOUCH_STATUS15_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_STATUS15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_touch_status15::R](R) reader structure"]
impl crate::Readable for SAR_TOUCH_STATUS15_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SAR_TOUCH_STATUS15 to value 0"]
impl crate::Resettable for SAR_TOUCH_STATUS15_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}