#[doc = "Register `RTC_GPIO_STATUS` reader"]
pub struct R(crate::R<RTC_GPIO_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_GPIO_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_GPIO_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_GPIO_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_GPIO_STATUS` writer"]
pub struct W(crate::W<RTC_GPIO_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_GPIO_STATUS_SPEC>;
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
impl From<crate::W<RTC_GPIO_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_GPIO_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_STATUS_INT` reader - GPIO0 ~ 21 interrupt status register. Bit10 corresponds to GPIO0, bit11 corresponds to GPIO1, etc. This register should be used together with RTCIO_RTC_GPIO_PINn_INT_TYPE in RTCIO_RTC_GPIO_PINn_REG. 0: no interrupt; 1: corresponding interrupt."]
pub struct GPIO_STATUS_INT_R(crate::FieldReader<u32, u32>);
impl GPIO_STATUS_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        GPIO_STATUS_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_STATUS_INT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_STATUS_INT` writer - GPIO0 ~ 21 interrupt status register. Bit10 corresponds to GPIO0, bit11 corresponds to GPIO1, etc. This register should be used together with RTCIO_RTC_GPIO_PINn_INT_TYPE in RTCIO_RTC_GPIO_PINn_REG. 0: no interrupt; 1: corresponding interrupt."]
pub struct GPIO_STATUS_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_STATUS_INT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x003f_ffff << 10)) | ((value as u32 & 0x003f_ffff) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 10:31 - GPIO0 ~ 21 interrupt status register. Bit10 corresponds to GPIO0, bit11 corresponds to GPIO1, etc. This register should be used together with RTCIO_RTC_GPIO_PINn_INT_TYPE in RTCIO_RTC_GPIO_PINn_REG. 0: no interrupt; 1: corresponding interrupt."]
    #[inline(always)]
    pub fn gpio_status_int(&self) -> GPIO_STATUS_INT_R {
        GPIO_STATUS_INT_R::new(((self.bits >> 10) & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 10:31 - GPIO0 ~ 21 interrupt status register. Bit10 corresponds to GPIO0, bit11 corresponds to GPIO1, etc. This register should be used together with RTCIO_RTC_GPIO_PINn_INT_TYPE in RTCIO_RTC_GPIO_PINn_REG. 0: no interrupt; 1: corresponding interrupt."]
    #[inline(always)]
    pub fn gpio_status_int(&mut self) -> GPIO_STATUS_INT_W {
        GPIO_STATUS_INT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC GPIO interrupt status register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_gpio_status]
(index.html) module"]
pub struct RTC_GPIO_STATUS_SPEC;
impl crate::RegisterSpec for RTC_GPIO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_gpio_status::R]
(R) reader structure"]
impl crate::Readable for RTC_GPIO_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_status::W]
(W) writer structure"]
impl crate::Writable for RTC_GPIO_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_GPIO_STATUS to value 0"]
impl crate::Resettable for RTC_GPIO_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}