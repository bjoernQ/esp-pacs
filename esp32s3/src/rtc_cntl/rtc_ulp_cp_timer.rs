#[doc = "Register `RTC_ULP_CP_TIMER` reader"]
pub struct R(crate::R<RTC_ULP_CP_TIMER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_ULP_CP_TIMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_ULP_CP_TIMER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_ULP_CP_TIMER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_ULP_CP_TIMER` writer"]
pub struct W(crate::W<RTC_ULP_CP_TIMER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_ULP_CP_TIMER_SPEC>;
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
impl From<crate::W<RTC_ULP_CP_TIMER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_ULP_CP_TIMER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ULP_CP_PC_INIT` reader - ULP-coprocessor PC initial address"]
pub struct ULP_CP_PC_INIT_R(crate::FieldReader<u16, u16>);
impl ULP_CP_PC_INIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ULP_CP_PC_INIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ULP_CP_PC_INIT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ULP_CP_PC_INIT` writer - ULP-coprocessor PC initial address"]
pub struct ULP_CP_PC_INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> ULP_CP_PC_INIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
#[doc = "Field `ULP_CP_GPIO_WAKEUP_ENA` reader - ULP-coprocessor wakeup by GPIO enable"]
pub struct ULP_CP_GPIO_WAKEUP_ENA_R(crate::FieldReader<bool, bool>);
impl ULP_CP_GPIO_WAKEUP_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ULP_CP_GPIO_WAKEUP_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ULP_CP_GPIO_WAKEUP_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ULP_CP_GPIO_WAKEUP_ENA` writer - ULP-coprocessor wakeup by GPIO enable"]
pub struct ULP_CP_GPIO_WAKEUP_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ULP_CP_GPIO_WAKEUP_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `ULP_CP_GPIO_WAKEUP_CLR` writer - ULP-coprocessor wakeup by GPIO state clear"]
pub struct ULP_CP_GPIO_WAKEUP_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ULP_CP_GPIO_WAKEUP_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `ULP_CP_SLP_TIMER_EN` reader - ULP-coprocessor timer enable bit"]
pub struct ULP_CP_SLP_TIMER_EN_R(crate::FieldReader<bool, bool>);
impl ULP_CP_SLP_TIMER_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ULP_CP_SLP_TIMER_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ULP_CP_SLP_TIMER_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ULP_CP_SLP_TIMER_EN` writer - ULP-coprocessor timer enable bit"]
pub struct ULP_CP_SLP_TIMER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ULP_CP_SLP_TIMER_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - ULP-coprocessor PC initial address"]
    #[inline(always)]
    pub fn ulp_cp_pc_init(&self) -> ULP_CP_PC_INIT_R {
        ULP_CP_PC_INIT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 29 - ULP-coprocessor wakeup by GPIO enable"]
    #[inline(always)]
    pub fn ulp_cp_gpio_wakeup_ena(&self) -> ULP_CP_GPIO_WAKEUP_ENA_R {
        ULP_CP_GPIO_WAKEUP_ENA_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 31 - ULP-coprocessor timer enable bit"]
    #[inline(always)]
    pub fn ulp_cp_slp_timer_en(&self) -> ULP_CP_SLP_TIMER_EN_R {
        ULP_CP_SLP_TIMER_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - ULP-coprocessor PC initial address"]
    #[inline(always)]
    pub fn ulp_cp_pc_init(&mut self) -> ULP_CP_PC_INIT_W {
        ULP_CP_PC_INIT_W { w: self }
    }
    #[doc = "Bit 29 - ULP-coprocessor wakeup by GPIO enable"]
    #[inline(always)]
    pub fn ulp_cp_gpio_wakeup_ena(&mut self) -> ULP_CP_GPIO_WAKEUP_ENA_W {
        ULP_CP_GPIO_WAKEUP_ENA_W { w: self }
    }
    #[doc = "Bit 30 - ULP-coprocessor wakeup by GPIO state clear"]
    #[inline(always)]
    pub fn ulp_cp_gpio_wakeup_clr(&mut self) -> ULP_CP_GPIO_WAKEUP_CLR_W {
        ULP_CP_GPIO_WAKEUP_CLR_W { w: self }
    }
    #[doc = "Bit 31 - ULP-coprocessor timer enable bit"]
    #[inline(always)]
    pub fn ulp_cp_slp_timer_en(&mut self) -> ULP_CP_SLP_TIMER_EN_W {
        ULP_CP_SLP_TIMER_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure ulp\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_ulp_cp_timer]
(index.html) module"]
pub struct RTC_ULP_CP_TIMER_SPEC;
impl crate::RegisterSpec for RTC_ULP_CP_TIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_ulp_cp_timer::R]
(R) reader structure"]
impl crate::Readable for RTC_ULP_CP_TIMER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_ulp_cp_timer::W]
(W) writer structure"]
impl crate::Writable for RTC_ULP_CP_TIMER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_ULP_CP_TIMER to value 0"]
impl crate::Resettable for RTC_ULP_CP_TIMER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}