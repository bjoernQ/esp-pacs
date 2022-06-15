#[doc = "Register `CACHE_VADDR` reader"]
pub struct R(crate::R<CACHE_VADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_VADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_VADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_VADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_VADDR` writer"]
pub struct W(crate::W<CACHE_VADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_VADDR_SPEC>;
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
impl From<crate::W<CACHE_VADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_VADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CACHE_VADDR` reader - Those bits stores the virtual address which will decide where inside the specified tag memory object will be accessed."]
pub type CACHE_VADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CACHE_VADDR` writer - Those bits stores the virtual address which will decide where inside the specified tag memory object will be accessed."]
pub type CACHE_VADDR_W<'a> = crate::FieldWriter<'a, u32, CACHE_VADDR_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Those bits stores the virtual address which will decide where inside the specified tag memory object will be accessed."]
    #[inline(always)]
    pub fn cache_vaddr(&self) -> CACHE_VADDR_R {
        CACHE_VADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Those bits stores the virtual address which will decide where inside the specified tag memory object will be accessed."]
    #[inline(always)]
    pub fn cache_vaddr(&mut self) -> CACHE_VADDR_W {
        CACHE_VADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_vaddr](index.html) module"]
pub struct CACHE_VADDR_SPEC;
impl crate::RegisterSpec for CACHE_VADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_vaddr::R](R) reader structure"]
impl crate::Readable for CACHE_VADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_vaddr::W](W) writer structure"]
impl crate::Writable for CACHE_VADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CACHE_VADDR to value 0"]
impl crate::Resettable for CACHE_VADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
