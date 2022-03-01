#[doc = "Register `LACTRTC` reader"]
pub struct R(crate::R<LACTRTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LACTRTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LACTRTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LACTRTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LACTRTC` writer"]
pub struct W(crate::W<LACTRTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LACTRTC_SPEC>;
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
impl From<crate::W<LACTRTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LACTRTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LACT_RTC_STEP_LEN` reader - Reserved."]
pub struct LACT_RTC_STEP_LEN_R(crate::FieldReader<u32, u32>);
impl LACT_RTC_STEP_LEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        LACT_RTC_STEP_LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LACT_RTC_STEP_LEN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LACT_RTC_STEP_LEN` writer - Reserved."]
pub struct LACT_RTC_STEP_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LACT_RTC_STEP_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff_ffff << 6)) | ((value as u32 & 0x03ff_ffff) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:31 - Reserved."]
    #[inline(always)]
    pub fn lact_rtc_step_len(&self) -> LACT_RTC_STEP_LEN_R {
        LACT_RTC_STEP_LEN_R::new(((self.bits >> 6) & 0x03ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 6:31 - Reserved."]
    #[inline(always)]
    pub fn lact_rtc_step_len(&mut self) -> LACT_RTC_STEP_LEN_W {
        LACT_RTC_STEP_LEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LACT RTC register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lactrtc]
(index.html) module"]
pub struct LACTRTC_SPEC;
impl crate::RegisterSpec for LACTRTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lactrtc::R]
(R) reader structure"]
impl crate::Readable for LACTRTC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lactrtc::W]
(W) writer structure"]
impl crate::Writable for LACTRTC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LACTRTC to value 0"]
impl crate::Resettable for LACTRTC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}