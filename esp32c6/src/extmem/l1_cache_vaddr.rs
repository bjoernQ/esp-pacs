#[doc = "Register `L1_CACHE_VADDR` reader"]
pub struct R(crate::R<L1_CACHE_VADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1_CACHE_VADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1_CACHE_VADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1_CACHE_VADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L1_CACHE_VADDR` writer"]
pub struct W(crate::W<L1_CACHE_VADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L1_CACHE_VADDR_SPEC>;
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
impl From<crate::W<L1_CACHE_VADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L1_CACHE_VADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `L1_CACHE_VADDR` reader - Those bits stores the virtual address which will decide where inside the specified tag memory object will be accessed."]
pub type L1_CACHE_VADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `L1_CACHE_VADDR` writer - Those bits stores the virtual address which will decide where inside the specified tag memory object will be accessed."]
pub type L1_CACHE_VADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, L1_CACHE_VADDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Those bits stores the virtual address which will decide where inside the specified tag memory object will be accessed."]
    #[inline(always)]
    pub fn l1_cache_vaddr(&self) -> L1_CACHE_VADDR_R {
        L1_CACHE_VADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Those bits stores the virtual address which will decide where inside the specified tag memory object will be accessed."]
    #[inline(always)]
    #[must_use]
    pub fn l1_cache_vaddr(&mut self) -> L1_CACHE_VADDR_W<0> {
        L1_CACHE_VADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache Vaddr register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1_cache_vaddr](index.html) module"]
pub struct L1_CACHE_VADDR_SPEC;
impl crate::RegisterSpec for L1_CACHE_VADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1_cache_vaddr::R](R) reader structure"]
impl crate::Readable for L1_CACHE_VADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l1_cache_vaddr::W](W) writer structure"]
impl crate::Writable for L1_CACHE_VADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L1_CACHE_VADDR to value 0x4000_0000"]
impl crate::Resettable for L1_CACHE_VADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000_0000;
}
