#[doc = "Register `INT_CLR` writer"]
pub struct W(crate::W<INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC2_THRES_INT_CLR` writer - Clear bit of APB_SARADC_ADC2_THRES_INT interrupt."]
pub type ADC2_THRES_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 28>;
#[doc = "Field `ADC1_THRES_INT_CLR` writer - Clear bit of APB_SARADC_ADC1_THRES_INT interrupt."]
pub type ADC1_THRES_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 29>;
#[doc = "Field `ADC2_DONE_INT_CLR` writer - Clear bit of APB_SARADC_ADC2_DONE_INT interrupt."]
pub type ADC2_DONE_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 30>;
#[doc = "Field `ADC1_DONE_INT_CLR` writer - Clear bit of APB_SARADC_ADC1_DONE_INT interrupt."]
pub type ADC1_DONE_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 31>;
impl W {
    #[doc = "Bit 28 - Clear bit of APB_SARADC_ADC2_THRES_INT interrupt."]
    #[inline(always)]
    pub fn adc2_thres_int_clr(&mut self) -> ADC2_THRES_INT_CLR_W {
        ADC2_THRES_INT_CLR_W::new(self)
    }
    #[doc = "Bit 29 - Clear bit of APB_SARADC_ADC1_THRES_INT interrupt."]
    #[inline(always)]
    pub fn adc1_thres_int_clr(&mut self) -> ADC1_THRES_INT_CLR_W {
        ADC1_THRES_INT_CLR_W::new(self)
    }
    #[doc = "Bit 30 - Clear bit of APB_SARADC_ADC2_DONE_INT interrupt."]
    #[inline(always)]
    pub fn adc2_done_int_clr(&mut self) -> ADC2_DONE_INT_CLR_W {
        ADC2_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 31 - Clear bit of APB_SARADC_ADC1_DONE_INT interrupt."]
    #[inline(always)]
    pub fn adc1_done_int_clr(&mut self) -> ADC1_DONE_INT_CLR_W {
        ADC1_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear DIG ADC interrupts\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](index.html) module"]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_clr::W](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
