#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Field `ADC2_THRES_INT_RAW` reader - Raw bit of APB_SARADC_ADC2_THRES_INT interrupt."]
pub type ADC2_THRES_INT_RAW_R = crate::BitReader;
#[doc = "Field `ADC1_THRES_INT_RAW` reader - Raw bit of APB_SARADC_ADC1_THRES_INT interrupt."]
pub type ADC1_THRES_INT_RAW_R = crate::BitReader;
#[doc = "Field `ADC2_DONE_INT_RAW` reader - Raw bit of APB_SARADC_ADC2_DONE_INT interrupt."]
pub type ADC2_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `ADC1_DONE_INT_RAW` reader - Raw bit of APB_SARADC_ADC1_DONE_INT interrupt."]
pub type ADC1_DONE_INT_RAW_R = crate::BitReader;
impl R {
    #[doc = "Bit 28 - Raw bit of APB_SARADC_ADC2_THRES_INT interrupt."]
    #[inline(always)]
    pub fn adc2_thres_int_raw(&self) -> ADC2_THRES_INT_RAW_R {
        ADC2_THRES_INT_RAW_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Raw bit of APB_SARADC_ADC1_THRES_INT interrupt."]
    #[inline(always)]
    pub fn adc1_thres_int_raw(&self) -> ADC1_THRES_INT_RAW_R {
        ADC1_THRES_INT_RAW_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Raw bit of APB_SARADC_ADC2_DONE_INT interrupt."]
    #[inline(always)]
    pub fn adc2_done_int_raw(&self) -> ADC2_DONE_INT_RAW_R {
        ADC2_DONE_INT_RAW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Raw bit of APB_SARADC_ADC1_DONE_INT interrupt."]
    #[inline(always)]
    pub fn adc1_done_int_raw(&self) -> ADC1_DONE_INT_RAW_R {
        ADC1_DONE_INT_RAW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field(
                "adc2_thres_int_raw",
                &format_args!("{}", self.adc2_thres_int_raw().bit()),
            )
            .field(
                "adc1_thres_int_raw",
                &format_args!("{}", self.adc1_thres_int_raw().bit()),
            )
            .field(
                "adc2_done_int_raw",
                &format_args!("{}", self.adc2_done_int_raw().bit()),
            )
            .field(
                "adc1_done_int_raw",
                &format_args!("{}", self.adc1_done_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "DIG ADC interrupt raw bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
