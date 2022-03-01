#[doc = "Register `PRO_TG1_WDT_LEVEL_INT_MAP` reader"]
pub struct R(crate::R<PRO_TG1_WDT_LEVEL_INT_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_TG1_WDT_LEVEL_INT_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_TG1_WDT_LEVEL_INT_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_TG1_WDT_LEVEL_INT_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_TG1_WDT_LEVEL_INT_MAP` writer"]
pub struct W(crate::W<PRO_TG1_WDT_LEVEL_INT_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_TG1_WDT_LEVEL_INT_MAP_SPEC>;
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
impl From<crate::W<PRO_TG1_WDT_LEVEL_INT_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_TG1_WDT_LEVEL_INT_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_TG1_WDT_LEVEL_INT_MAP` reader - This register is used to map TG1_WDT_LEVEL_INT interrupt signal to one of the CPU interrupts."]
pub struct PRO_TG1_WDT_LEVEL_INT_MAP_R(crate::FieldReader<u8, u8>);
impl PRO_TG1_WDT_LEVEL_INT_MAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRO_TG1_WDT_LEVEL_INT_MAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_TG1_WDT_LEVEL_INT_MAP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_TG1_WDT_LEVEL_INT_MAP` writer - This register is used to map TG1_WDT_LEVEL_INT interrupt signal to one of the CPU interrupts."]
pub struct PRO_TG1_WDT_LEVEL_INT_MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_TG1_WDT_LEVEL_INT_MAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - This register is used to map TG1_WDT_LEVEL_INT interrupt signal to one of the CPU interrupts."]
    #[inline(always)]
    pub fn pro_tg1_wdt_level_int_map(&self) -> PRO_TG1_WDT_LEVEL_INT_MAP_R {
        PRO_TG1_WDT_LEVEL_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - This register is used to map TG1_WDT_LEVEL_INT interrupt signal to one of the CPU interrupts."]
    #[inline(always)]
    pub fn pro_tg1_wdt_level_int_map(&mut self) -> PRO_TG1_WDT_LEVEL_INT_MAP_W {
        PRO_TG1_WDT_LEVEL_INT_MAP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TG1_WDT_LEVEL_INT interrupt configuration register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_tg1_wdt_level_int_map]
(index.html) module"]
pub struct PRO_TG1_WDT_LEVEL_INT_MAP_SPEC;
impl crate::RegisterSpec for PRO_TG1_WDT_LEVEL_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_tg1_wdt_level_int_map::R]
(R) reader structure"]
impl crate::Readable for PRO_TG1_WDT_LEVEL_INT_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_tg1_wdt_level_int_map::W]
(W) writer structure"]
impl crate::Writable for PRO_TG1_WDT_LEVEL_INT_MAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_TG1_WDT_LEVEL_INT_MAP to value 0x10"]
impl crate::Resettable for PRO_TG1_WDT_LEVEL_INT_MAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}