#[doc = "Register `APB_SARADC_SAR2_PATT_TAB3` reader"]
pub struct R(crate::R<APB_SARADC_SAR2_PATT_TAB3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_SARADC_SAR2_PATT_TAB3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_SARADC_SAR2_PATT_TAB3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_SARADC_SAR2_PATT_TAB3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB_SARADC_SAR2_PATT_TAB3` writer"]
pub struct W(crate::W<APB_SARADC_SAR2_PATT_TAB3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB_SARADC_SAR2_PATT_TAB3_SPEC>;
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
impl From<crate::W<APB_SARADC_SAR2_PATT_TAB3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB_SARADC_SAR2_PATT_TAB3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SARADC_SAR2_PATT_TAB3` reader - Item 8 ~ 11 for pattern table 2 (each item one byte)"]
pub type SARADC_SAR2_PATT_TAB3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SARADC_SAR2_PATT_TAB3` writer - Item 8 ~ 11 for pattern table 2 (each item one byte)"]
pub type SARADC_SAR2_PATT_TAB3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, APB_SARADC_SAR2_PATT_TAB3_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Item 8 ~ 11 for pattern table 2 (each item one byte)"]
    #[inline(always)]
    pub fn saradc_sar2_patt_tab3(&self) -> SARADC_SAR2_PATT_TAB3_R {
        SARADC_SAR2_PATT_TAB3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Item 8 ~ 11 for pattern table 2 (each item one byte)"]
    #[inline(always)]
    pub fn saradc_sar2_patt_tab3(&mut self) -> SARADC_SAR2_PATT_TAB3_W<0> {
        SARADC_SAR2_PATT_TAB3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_saradc_sar2_patt_tab3](index.html) module"]
pub struct APB_SARADC_SAR2_PATT_TAB3_SPEC;
impl crate::RegisterSpec for APB_SARADC_SAR2_PATT_TAB3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb_saradc_sar2_patt_tab3::R](R) reader structure"]
impl crate::Readable for APB_SARADC_SAR2_PATT_TAB3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb_saradc_sar2_patt_tab3::W](W) writer structure"]
impl crate::Writable for APB_SARADC_SAR2_PATT_TAB3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB_SARADC_SAR2_PATT_TAB3 to value 0x0f0f_0f0f"]
impl crate::Resettable for APB_SARADC_SAR2_PATT_TAB3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f0f_0f0f
    }
}