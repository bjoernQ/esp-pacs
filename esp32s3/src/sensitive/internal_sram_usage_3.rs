#[doc = "Register `INTERNAL_SRAM_USAGE_3` reader"]
pub struct R(crate::R<INTERNAL_SRAM_USAGE_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERNAL_SRAM_USAGE_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERNAL_SRAM_USAGE_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERNAL_SRAM_USAGE_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTERNAL_SRAM_USAGE_3` writer"]
pub struct W(crate::W<INTERNAL_SRAM_USAGE_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTERNAL_SRAM_USAGE_3_SPEC>;
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
impl From<crate::W<INTERNAL_SRAM_USAGE_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTERNAL_SRAM_USAGE_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTERNAL_SRAM_MAC_DUMP_USAGE` reader - Set 1 to someone bit means corresponding internal SRAM level can be accessed by mac dump."]
pub struct INTERNAL_SRAM_MAC_DUMP_USAGE_R(crate::FieldReader<u8, u8>);
impl INTERNAL_SRAM_MAC_DUMP_USAGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INTERNAL_SRAM_MAC_DUMP_USAGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERNAL_SRAM_MAC_DUMP_USAGE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTERNAL_SRAM_MAC_DUMP_USAGE` writer - Set 1 to someone bit means corresponding internal SRAM level can be accessed by mac dump."]
pub struct INTERNAL_SRAM_MAC_DUMP_USAGE_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERNAL_SRAM_MAC_DUMP_USAGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Set 1 to someone bit means corresponding internal SRAM level can be accessed by mac dump."]
    #[inline(always)]
    pub fn internal_sram_mac_dump_usage(&self) -> INTERNAL_SRAM_MAC_DUMP_USAGE_R {
        INTERNAL_SRAM_MAC_DUMP_USAGE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Set 1 to someone bit means corresponding internal SRAM level can be accessed by mac dump."]
    #[inline(always)]
    pub fn internal_sram_mac_dump_usage(&mut self) -> INTERNAL_SRAM_MAC_DUMP_USAGE_W {
        INTERNAL_SRAM_MAC_DUMP_USAGE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal SRAM configuration register 3.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [internal_sram_usage_3]
(index.html) module"]
pub struct INTERNAL_SRAM_USAGE_3_SPEC;
impl crate::RegisterSpec for INTERNAL_SRAM_USAGE_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [internal_sram_usage_3::R]
(R) reader structure"]
impl crate::Readable for INTERNAL_SRAM_USAGE_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [internal_sram_usage_3::W]
(W) writer structure"]
impl crate::Writable for INTERNAL_SRAM_USAGE_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTERNAL_SRAM_USAGE_3 to value 0"]
impl crate::Resettable for INTERNAL_SRAM_USAGE_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}