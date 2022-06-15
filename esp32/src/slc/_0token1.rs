#[doc = "Register `_0TOKEN1` reader"]
pub struct R(crate::R<_0TOKEN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_0TOKEN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_0TOKEN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_0TOKEN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `_0TOKEN1` writer"]
pub struct W(crate::W<_0TOKEN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<_0TOKEN1_SPEC>;
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
impl From<crate::W<_0TOKEN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<_0TOKEN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC0_TOKEN1_WDATA` writer - "]
pub type SLC0_TOKEN1_WDATA_W<'a> = crate::FieldWriter<'a, u32, _0TOKEN1_SPEC, u16, u16, 12, 0>;
#[doc = "Field `SLC0_TOKEN1_WR` writer - "]
pub type SLC0_TOKEN1_WR_W<'a> = crate::BitWriter<'a, u32, _0TOKEN1_SPEC, bool, 12>;
#[doc = "Field `SLC0_TOKEN1_INC` writer - "]
pub type SLC0_TOKEN1_INC_W<'a> = crate::BitWriter<'a, u32, _0TOKEN1_SPEC, bool, 13>;
#[doc = "Field `SLC0_TOKEN1_INC_MORE` writer - "]
pub type SLC0_TOKEN1_INC_MORE_W<'a> = crate::BitWriter<'a, u32, _0TOKEN1_SPEC, bool, 14>;
#[doc = "Field `SLC0_TOKEN1` reader - "]
pub type SLC0_TOKEN1_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn slc0_token1(&self) -> SLC0_TOKEN1_R {
        SLC0_TOKEN1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn slc0_token1_wdata(&mut self) -> SLC0_TOKEN1_WDATA_W {
        SLC0_TOKEN1_WDATA_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slc0_token1_wr(&mut self) -> SLC0_TOKEN1_WR_W {
        SLC0_TOKEN1_WR_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn slc0_token1_inc(&mut self) -> SLC0_TOKEN1_INC_W {
        SLC0_TOKEN1_INC_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn slc0_token1_inc_more(&mut self) -> SLC0_TOKEN1_INC_MORE_W {
        SLC0_TOKEN1_INC_MORE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_0token1](index.html) module"]
pub struct _0TOKEN1_SPEC;
impl crate::RegisterSpec for _0TOKEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_0token1::R](R) reader structure"]
impl crate::Readable for _0TOKEN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [_0token1::W](W) writer structure"]
impl crate::Writable for _0TOKEN1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets _0TOKEN1 to value 0"]
impl crate::Resettable for _0TOKEN1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
