#[doc = "Register `UNIT0_OP` reader"]
pub struct R(crate::R<UNIT0_OP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UNIT0_OP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UNIT0_OP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UNIT0_OP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UNIT0_OP` writer"]
pub struct W(crate::W<UNIT0_OP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UNIT0_OP_SPEC>;
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
impl From<crate::W<UNIT0_OP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UNIT0_OP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_UNIT0_VALUE_VALID` reader - reg_timer_unit0_value_valid"]
pub struct TIMER_UNIT0_VALUE_VALID_R(crate::FieldReader<bool, bool>);
impl TIMER_UNIT0_VALUE_VALID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER_UNIT0_VALUE_VALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_UNIT0_VALUE_VALID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER_UNIT0_UPDATE` writer - update timer_unit0"]
pub struct TIMER_UNIT0_UPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_UNIT0_UPDATE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 29 - reg_timer_unit0_value_valid"]
    #[inline(always)]
    pub fn timer_unit0_value_valid(&self) -> TIMER_UNIT0_VALUE_VALID_R {
        TIMER_UNIT0_VALUE_VALID_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - update timer_unit0"]
    #[inline(always)]
    pub fn timer_unit0_update(&mut self) -> TIMER_UNIT0_UPDATE_W {
        TIMER_UNIT0_UPDATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSTIMER_UNIT0_OP.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [unit0_op]
(index.html) module"]
pub struct UNIT0_OP_SPEC;
impl crate::RegisterSpec for UNIT0_OP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [unit0_op::R]
(R) reader structure"]
impl crate::Readable for UNIT0_OP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [unit0_op::W]
(W) writer structure"]
impl crate::Writable for UNIT0_OP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UNIT0_OP to value 0"]
impl crate::Resettable for UNIT0_OP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}