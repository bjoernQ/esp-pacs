#[doc = "Register `THRES0_CTRL` reader"]
pub struct R(crate::R<THRES0_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<THRES0_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<THRES0_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<THRES0_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `THRES0_CTRL` writer"]
pub struct W(crate::W<THRES0_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<THRES0_CTRL_SPEC>;
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
impl From<crate::W<THRES0_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<THRES0_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THRES0_CHANNEL` reader - configure which channel thres0 monitor"]
pub type THRES0_CHANNEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `THRES0_CHANNEL` writer - configure which channel thres0 monitor"]
pub type THRES0_CHANNEL_W<'a> = crate::FieldWriter<'a, u32, THRES0_CTRL_SPEC, u8, u8, 5, 0>;
#[doc = "Field `THRES0_HIGH` reader - thres0 monitor high thres"]
pub type THRES0_HIGH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `THRES0_HIGH` writer - thres0 monitor high thres"]
pub type THRES0_HIGH_W<'a> = crate::FieldWriter<'a, u32, THRES0_CTRL_SPEC, u16, u16, 13, 5>;
#[doc = "Field `THRES0_LOW` reader - thres0 monitor low thres"]
pub type THRES0_LOW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `THRES0_LOW` writer - thres0 monitor low thres"]
pub type THRES0_LOW_W<'a> = crate::FieldWriter<'a, u32, THRES0_CTRL_SPEC, u16, u16, 13, 18>;
impl R {
    #[doc = "Bits 0:4 - configure which channel thres0 monitor"]
    #[inline(always)]
    pub fn thres0_channel(&self) -> THRES0_CHANNEL_R {
        THRES0_CHANNEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:17 - thres0 monitor high thres"]
    #[inline(always)]
    pub fn thres0_high(&self) -> THRES0_HIGH_R {
        THRES0_HIGH_R::new(((self.bits >> 5) & 0x1fff) as u16)
    }
    #[doc = "Bits 18:30 - thres0 monitor low thres"]
    #[inline(always)]
    pub fn thres0_low(&self) -> THRES0_LOW_R {
        THRES0_LOW_R::new(((self.bits >> 18) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - configure which channel thres0 monitor"]
    #[inline(always)]
    pub fn thres0_channel(&mut self) -> THRES0_CHANNEL_W {
        THRES0_CHANNEL_W::new(self)
    }
    #[doc = "Bits 5:17 - thres0 monitor high thres"]
    #[inline(always)]
    pub fn thres0_high(&mut self) -> THRES0_HIGH_W {
        THRES0_HIGH_W::new(self)
    }
    #[doc = "Bits 18:30 - thres0 monitor low thres"]
    #[inline(always)]
    pub fn thres0_low(&mut self) -> THRES0_LOW_W {
        THRES0_LOW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure apb saradc thres monitor\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [thres0_ctrl](index.html) module"]
pub struct THRES0_CTRL_SPEC;
impl crate::RegisterSpec for THRES0_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [thres0_ctrl::R](R) reader structure"]
impl crate::Readable for THRES0_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [thres0_ctrl::W](W) writer structure"]
impl crate::Writable for THRES0_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets THRES0_CTRL to value 0x0003_ffed"]
impl crate::Resettable for THRES0_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0003_ffed
    }
}
