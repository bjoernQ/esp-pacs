#[doc = "Register `CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON` reader"]
pub struct R(crate::R<CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON` writer"]
pub struct W(crate::W<CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON_SPEC>;
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
impl From<crate::W<CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_FORCE_ON_DB_ENCRYPT` reader - The bit is used to close clock gating of encrypt clock. 1: close gating, 0: open clock gating."]
pub struct CLK_FORCE_ON_DB_ENCRYPT_R(crate::FieldReader<bool, bool>);
impl CLK_FORCE_ON_DB_ENCRYPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_FORCE_ON_DB_ENCRYPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_FORCE_ON_DB_ENCRYPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_FORCE_ON_DB_ENCRYPT` writer - The bit is used to close clock gating of encrypt clock. 1: close gating, 0: open clock gating."]
pub struct CLK_FORCE_ON_DB_ENCRYPT_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_FORCE_ON_DB_ENCRYPT_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `CLK_FORCE_ON_G0CB_DECRYPT` reader - The bit is used to close clock gating of decrypt clock. 1: close gating, 0: open clock gating."]
pub struct CLK_FORCE_ON_G0CB_DECRYPT_R(crate::FieldReader<bool, bool>);
impl CLK_FORCE_ON_G0CB_DECRYPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_FORCE_ON_G0CB_DECRYPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_FORCE_ON_G0CB_DECRYPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_FORCE_ON_G0CB_DECRYPT` writer - The bit is used to close clock gating of decrypt clock. 1: close gating, 0: open clock gating."]
pub struct CLK_FORCE_ON_G0CB_DECRYPT_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_FORCE_ON_G0CB_DECRYPT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `CLK_FORCE_ON_AUTOMATIC_ENCRYPT_DECRYPT` reader - The bit is used to close clock gating of encrypt and decrypt clock. 1: close gating, 0: open clock gating."]
pub struct CLK_FORCE_ON_AUTOMATIC_ENCRYPT_DECRYPT_R(crate::FieldReader<bool, bool>);
impl CLK_FORCE_ON_AUTOMATIC_ENCRYPT_DECRYPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_FORCE_ON_AUTOMATIC_ENCRYPT_DECRYPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_FORCE_ON_AUTOMATIC_ENCRYPT_DECRYPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_FORCE_ON_AUTOMATIC_ENCRYPT_DECRYPT` writer - The bit is used to close clock gating of encrypt and decrypt clock. 1: close gating, 0: open clock gating."]
pub struct CLK_FORCE_ON_AUTOMATIC_ENCRYPT_DECRYPT_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_FORCE_ON_AUTOMATIC_ENCRYPT_DECRYPT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - The bit is used to close clock gating of encrypt clock. 1: close gating, 0: open clock gating."]
    #[inline(always)]
    pub fn clk_force_on_db_encrypt(&self) -> CLK_FORCE_ON_DB_ENCRYPT_R {
        CLK_FORCE_ON_DB_ENCRYPT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - The bit is used to close clock gating of decrypt clock. 1: close gating, 0: open clock gating."]
    #[inline(always)]
    pub fn clk_force_on_g0cb_decrypt(&self) -> CLK_FORCE_ON_G0CB_DECRYPT_R {
        CLK_FORCE_ON_G0CB_DECRYPT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The bit is used to close clock gating of encrypt and decrypt clock. 1: close gating, 0: open clock gating."]
    #[inline(always)]
    pub fn clk_force_on_automatic_encrypt_decrypt(
        &self,
    ) -> CLK_FORCE_ON_AUTOMATIC_ENCRYPT_DECRYPT_R {
        CLK_FORCE_ON_AUTOMATIC_ENCRYPT_DECRYPT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to close clock gating of encrypt clock. 1: close gating, 0: open clock gating."]
    #[inline(always)]
    pub fn clk_force_on_db_encrypt(&mut self) -> CLK_FORCE_ON_DB_ENCRYPT_W {
        CLK_FORCE_ON_DB_ENCRYPT_W { w: self }
    }
    #[doc = "Bit 1 - The bit is used to close clock gating of decrypt clock. 1: close gating, 0: open clock gating."]
    #[inline(always)]
    pub fn clk_force_on_g0cb_decrypt(&mut self) -> CLK_FORCE_ON_G0CB_DECRYPT_W {
        CLK_FORCE_ON_G0CB_DECRYPT_W { w: self }
    }
    #[doc = "Bit 2 - The bit is used to close clock gating of encrypt and decrypt clock. 1: close gating, 0: open clock gating."]
    #[inline(always)]
    pub fn clk_force_on_automatic_encrypt_decrypt(
        &mut self,
    ) -> CLK_FORCE_ON_AUTOMATIC_ENCRYPT_DECRYPT_W {
        CLK_FORCE_ON_AUTOMATIC_ENCRYPT_DECRYPT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_encrypt_decrypt_clk_force_on]
(index.html) module"]
pub struct CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON_SPEC;
impl crate::RegisterSpec for CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_encrypt_decrypt_clk_force_on::R]
(R) reader structure"]
impl crate::Readable for CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_encrypt_decrypt_clk_force_on::W]
(W) writer structure"]
impl crate::Writable for CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON to value 0x07"]
impl crate::Resettable for CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}