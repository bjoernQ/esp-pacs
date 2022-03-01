#[doc = "Register `SRAM_ACE3_SIZE` reader"]
pub struct R(crate::R<SRAM_ACE3_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAM_ACE3_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAM_ACE3_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAM_ACE3_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRAM_ACE3_SIZE` writer"]
pub struct W(crate::W<SRAM_ACE3_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAM_ACE3_SIZE_SPEC>;
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
impl From<crate::W<SRAM_ACE3_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAM_ACE3_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRAM_ACE3_SIZE` reader - ******* Description ***********"]
pub struct SRAM_ACE3_SIZE_R(crate::FieldReader<u16, u16>);
impl SRAM_ACE3_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SRAM_ACE3_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_ACE3_SIZE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_ACE3_SIZE` writer - ******* Description ***********"]
pub struct SRAM_ACE3_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_ACE3_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - ******* Description ***********"]
    #[inline(always)]
    pub fn sram_ace3_size(&self) -> SRAM_ACE3_SIZE_R {
        SRAM_ACE3_SIZE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ******* Description ***********"]
    #[inline(always)]
    pub fn sram_ace3_size(&mut self) -> SRAM_ACE3_SIZE_W {
        SRAM_ACE3_SIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sram_ace3_size]
(index.html) module"]
pub struct SRAM_ACE3_SIZE_SPEC;
impl crate::RegisterSpec for SRAM_ACE3_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sram_ace3_size::R]
(R) reader structure"]
impl crate::Readable for SRAM_ACE3_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sram_ace3_size::W]
(W) writer structure"]
impl crate::Writable for SRAM_ACE3_SIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRAM_ACE3_SIZE to value 0x1000"]
impl crate::Resettable for SRAM_ACE3_SIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1000
    }
}