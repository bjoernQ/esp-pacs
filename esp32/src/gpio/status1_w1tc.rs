#[doc = "Register `STATUS1_W1TC` reader"]
pub struct R(crate::R<STATUS1_W1TC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS1_W1TC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS1_W1TC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS1_W1TC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS1_W1TC` writer"]
pub struct W(crate::W<STATUS1_W1TC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS1_W1TC_SPEC>;
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
impl From<crate::W<STATUS1_W1TC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS1_W1TC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STATUS1_INT_W1TC` reader - GPIO32~39 interrupt status write 1 to clear"]
pub struct STATUS1_INT_W1TC_R(crate::FieldReader<u8, u8>);
impl STATUS1_INT_W1TC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STATUS1_INT_W1TC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATUS1_INT_W1TC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STATUS1_INT_W1TC` writer - GPIO32~39 interrupt status write 1 to clear"]
pub struct STATUS1_INT_W1TC_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS1_INT_W1TC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - GPIO32~39 interrupt status write 1 to clear"]
    #[inline(always)]
    pub fn status1_int_w1tc(&self) -> STATUS1_INT_W1TC_R {
        STATUS1_INT_W1TC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO32~39 interrupt status write 1 to clear"]
    #[inline(always)]
    pub fn status1_int_w1tc(&mut self) -> STATUS1_INT_W1TC_W {
        STATUS1_INT_W1TC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status1_w1tc]
(index.html) module"]
pub struct STATUS1_W1TC_SPEC;
impl crate::RegisterSpec for STATUS1_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status1_w1tc::R]
(R) reader structure"]
impl crate::Readable for STATUS1_W1TC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status1_w1tc::W]
(W) writer structure"]
impl crate::Writable for STATUS1_W1TC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATUS1_W1TC to value 0"]
impl crate::Resettable for STATUS1_W1TC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}