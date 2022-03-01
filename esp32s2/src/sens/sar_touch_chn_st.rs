#[doc = "Register `SAR_TOUCH_CHN_ST` reader"]
pub struct R(crate::R<SAR_TOUCH_CHN_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_TOUCH_CHN_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_TOUCH_CHN_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_TOUCH_CHN_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_TOUCH_CHN_ST` writer"]
pub struct W(crate::W<SAR_TOUCH_CHN_ST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_TOUCH_CHN_ST_SPEC>;
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
impl From<crate::W<SAR_TOUCH_CHN_ST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_TOUCH_CHN_ST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUCH_PAD_ACTIVE` reader - Touch active status"]
pub struct TOUCH_PAD_ACTIVE_R(crate::FieldReader<u16, u16>);
impl TOUCH_PAD_ACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TOUCH_PAD_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD_ACTIVE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_CHANNEL_CLR` writer - Clear touch channel"]
pub struct TOUCH_CHANNEL_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_CHANNEL_CLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 15)) | ((value as u32 & 0x7fff) << 15);
        self.w
    }
}
#[doc = "Field `TOUCH_MEAS_DONE` reader - Signal flag that indicates one touch pad is done."]
pub struct TOUCH_MEAS_DONE_R(crate::FieldReader<bool, bool>);
impl TOUCH_MEAS_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_MEAS_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_MEAS_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:14 - Touch active status"]
    #[inline(always)]
    pub fn touch_pad_active(&self) -> TOUCH_PAD_ACTIVE_R {
        TOUCH_PAD_ACTIVE_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - Signal flag that indicates one touch pad is done."]
    #[inline(always)]
    pub fn touch_meas_done(&self) -> TOUCH_MEAS_DONE_R {
        TOUCH_MEAS_DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 15:29 - Clear touch channel"]
    #[inline(always)]
    pub fn touch_channel_clr(&mut self) -> TOUCH_CHANNEL_CLR_W {
        TOUCH_CHANNEL_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Touch channel status register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_touch_chn_st]
(index.html) module"]
pub struct SAR_TOUCH_CHN_ST_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_CHN_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_touch_chn_st::R]
(R) reader structure"]
impl crate::Readable for SAR_TOUCH_CHN_ST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_touch_chn_st::W]
(W) writer structure"]
impl crate::Writable for SAR_TOUCH_CHN_ST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_TOUCH_CHN_ST to value 0"]
impl crate::Resettable for SAR_TOUCH_CHN_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}