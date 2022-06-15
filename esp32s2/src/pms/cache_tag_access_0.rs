#[doc = "Register `CACHE_TAG_ACCESS_0` reader"]
pub struct R(crate::R<CACHE_TAG_ACCESS_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_TAG_ACCESS_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_TAG_ACCESS_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_TAG_ACCESS_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_TAG_ACCESS_0` writer"]
pub struct W(crate::W<CACHE_TAG_ACCESS_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_TAG_ACCESS_0_SPEC>;
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
impl From<crate::W<CACHE_TAG_ACCESS_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_TAG_ACCESS_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CACHE_TAG_ACCESS_LOCK` reader - Lock register. Setting to 1 locks cache tag permission control registers."]
pub type CACHE_TAG_ACCESS_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `CACHE_TAG_ACCESS_LOCK` writer - Lock register. Setting to 1 locks cache tag permission control registers."]
pub type CACHE_TAG_ACCESS_LOCK_W<'a> = crate::BitWriter<'a, u32, CACHE_TAG_ACCESS_0_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Lock register. Setting to 1 locks cache tag permission control registers."]
    #[inline(always)]
    pub fn cache_tag_access_lock(&self) -> CACHE_TAG_ACCESS_LOCK_R {
        CACHE_TAG_ACCESS_LOCK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lock register. Setting to 1 locks cache tag permission control registers."]
    #[inline(always)]
    pub fn cache_tag_access_lock(&mut self) -> CACHE_TAG_ACCESS_LOCK_W {
        CACHE_TAG_ACCESS_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache tag permission control register 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_tag_access_0](index.html) module"]
pub struct CACHE_TAG_ACCESS_0_SPEC;
impl crate::RegisterSpec for CACHE_TAG_ACCESS_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_tag_access_0::R](R) reader structure"]
impl crate::Readable for CACHE_TAG_ACCESS_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_tag_access_0::W](W) writer structure"]
impl crate::Writable for CACHE_TAG_ACCESS_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CACHE_TAG_ACCESS_0 to value 0"]
impl crate::Resettable for CACHE_TAG_ACCESS_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
