#[doc = "Register `SLC_INTVEC_TOHOST` reader"]
pub struct R(crate::R<SLC_INTVEC_TOHOST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLC_INTVEC_TOHOST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLC_INTVEC_TOHOST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLC_INTVEC_TOHOST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLC_INTVEC_TOHOST` writer"]
pub struct W(crate::W<SLC_INTVEC_TOHOST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLC_INTVEC_TOHOST_SPEC>;
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
impl From<crate::W<SLC_INTVEC_TOHOST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLC_INTVEC_TOHOST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC_TOHOST_INTVEC` reader - "]
pub type SLC_TOHOST_INTVEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLC_TOHOST_INTVEC` writer - "]
pub type SLC_TOHOST_INTVEC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SLC_INTVEC_TOHOST_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn slc_tohost_intvec(&self) -> SLC_TOHOST_INTVEC_R {
        SLC_TOHOST_INTVEC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn slc_tohost_intvec(&mut self) -> SLC_TOHOST_INTVEC_W<0> {
        SLC_TOHOST_INTVEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SLC_INTVEC_TOHOST\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slc_intvec_tohost](index.html) module"]
pub struct SLC_INTVEC_TOHOST_SPEC;
impl crate::RegisterSpec for SLC_INTVEC_TOHOST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slc_intvec_tohost::R](R) reader structure"]
impl crate::Readable for SLC_INTVEC_TOHOST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slc_intvec_tohost::W](W) writer structure"]
impl crate::Writable for SLC_INTVEC_TOHOST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLC_INTVEC_TOHOST to value 0"]
impl crate::Resettable for SLC_INTVEC_TOHOST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}