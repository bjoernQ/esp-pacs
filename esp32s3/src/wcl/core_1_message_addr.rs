#[doc = "Register `Core_1_MESSAGE_ADDR` reader"]
pub struct R(crate::R<CORE_1_MESSAGE_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_MESSAGE_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_MESSAGE_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_MESSAGE_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Core_1_MESSAGE_ADDR` writer"]
pub struct W(crate::W<CORE_1_MESSAGE_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_1_MESSAGE_ADDR_SPEC>;
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
impl From<crate::W<CORE_1_MESSAGE_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_1_MESSAGE_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_1_MESSAGE_ADDR` reader - This field is used to set address that need to write when enter WORLD0"]
pub struct CORE_1_MESSAGE_ADDR_R(crate::FieldReader<u32, u32>);
impl CORE_1_MESSAGE_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CORE_1_MESSAGE_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_1_MESSAGE_ADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_1_MESSAGE_ADDR` writer - This field is used to set address that need to write when enter WORLD0"]
pub struct CORE_1_MESSAGE_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_1_MESSAGE_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - This field is used to set address that need to write when enter WORLD0"]
    #[inline(always)]
    pub fn core_1_message_addr(&self) -> CORE_1_MESSAGE_ADDR_R {
        CORE_1_MESSAGE_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This field is used to set address that need to write when enter WORLD0"]
    #[inline(always)]
    pub fn core_1_message_addr(&mut self) -> CORE_1_MESSAGE_ADDR_W {
        CORE_1_MESSAGE_ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear writer_buffer write address configuration register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_message_addr]
(index.html) module"]
pub struct CORE_1_MESSAGE_ADDR_SPEC;
impl crate::RegisterSpec for CORE_1_MESSAGE_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_message_addr::R]
(R) reader structure"]
impl crate::Readable for CORE_1_MESSAGE_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_1_message_addr::W]
(W) writer structure"]
impl crate::Writable for CORE_1_MESSAGE_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Core_1_MESSAGE_ADDR to value 0"]
impl crate::Resettable for CORE_1_MESSAGE_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}