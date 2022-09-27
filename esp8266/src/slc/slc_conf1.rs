#[doc = "Register `SLC_CONF1` reader"]
pub struct R(crate::R<SLC_CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLC_CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLC_CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLC_CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLC_CONF1` writer"]
pub struct W(crate::W<SLC_CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLC_CONF1_SPEC>;
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
impl From<crate::W<SLC_CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLC_CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Register` reader - "]
pub type REGISTER_R = crate::FieldReader<u32, u32>;
#[doc = "Field `Register` writer - "]
pub type REGISTER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SLC_CONF1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn register(&self) -> REGISTER_R {
        REGISTER_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn register(&mut self) -> REGISTER_W<0> {
        REGISTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SLC_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slc_conf1](index.html) module"]
pub struct SLC_CONF1_SPEC;
impl crate::RegisterSpec for SLC_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slc_conf1::R](R) reader structure"]
impl crate::Readable for SLC_CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slc_conf1::W](W) writer structure"]
impl crate::Writable for SLC_CONF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLC_CONF1 to value 0"]
impl crate::Resettable for SLC_CONF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}