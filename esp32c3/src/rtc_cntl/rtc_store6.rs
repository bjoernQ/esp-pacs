#[doc = "Register `RTC_STORE6` reader"]
pub struct R(crate::R<RTC_STORE6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_STORE6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_STORE6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_STORE6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_STORE6` writer"]
pub struct W(crate::W<RTC_STORE6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_STORE6_SPEC>;
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
impl From<crate::W<RTC_STORE6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_STORE6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_SCRATCH6` reader - reserved register"]
pub struct RTC_SCRATCH6_R(crate::FieldReader<u32, u32>);
impl RTC_SCRATCH6_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RTC_SCRATCH6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_SCRATCH6_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_SCRATCH6` writer - reserved register"]
pub struct RTC_SCRATCH6_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_SCRATCH6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - reserved register"]
    #[inline(always)]
    pub fn rtc_scratch6(&self) -> RTC_SCRATCH6_R {
        RTC_SCRATCH6_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - reserved register"]
    #[inline(always)]
    pub fn rtc_scratch6(&mut self) -> RTC_SCRATCH6_W {
        RTC_SCRATCH6_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_store6]
(index.html) module"]
pub struct RTC_STORE6_SPEC;
impl crate::RegisterSpec for RTC_STORE6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_store6::R]
(R) reader structure"]
impl crate::Readable for RTC_STORE6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_store6::W]
(W) writer structure"]
impl crate::Writable for RTC_STORE6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_STORE6 to value 0"]
impl crate::Resettable for RTC_STORE6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}