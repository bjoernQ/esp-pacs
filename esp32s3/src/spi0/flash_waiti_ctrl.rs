#[doc = "Register `FLASH_WAITI_CTRL` reader"]
pub struct R(crate::R<FLASH_WAITI_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_WAITI_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_WAITI_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_WAITI_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_WAITI_CTRL` writer"]
pub struct W(crate::W<FLASH_WAITI_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_WAITI_CTRL_SPEC>;
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
impl From<crate::W<FLASH_WAITI_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_WAITI_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAITI_EN` reader - Set this bit to enable auto-waiting flash idle operation when PP/SE/BE/CE/WRSR/PES command is sent."]
pub struct WAITI_EN_R(crate::FieldReader<bool, bool>);
impl WAITI_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAITI_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAITI_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAITI_EN` writer - Set this bit to enable auto-waiting flash idle operation when PP/SE/BE/CE/WRSR/PES command is sent."]
pub struct WAITI_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITI_EN_W<'a> {
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
#[doc = "Field `WAITI_DUMMY` reader - Set this bit to enable DUMMY phase in auto wait flash idle transfer(RDSR)."]
pub struct WAITI_DUMMY_R(crate::FieldReader<bool, bool>);
impl WAITI_DUMMY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAITI_DUMMY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAITI_DUMMY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAITI_DUMMY` writer - Set this bit to enable DUMMY phase in auto wait flash idle transfer(RDSR)."]
pub struct WAITI_DUMMY_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITI_DUMMY_W<'a> {
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
#[doc = "Field `WAITI_CMD` reader - The command value of auto wait flash idle transfer(RDSR)."]
pub struct WAITI_CMD_R(crate::FieldReader<u8, u8>);
impl WAITI_CMD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WAITI_CMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAITI_CMD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAITI_CMD` writer - The command value of auto wait flash idle transfer(RDSR)."]
pub struct WAITI_CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITI_CMD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 2)) | ((value as u32 & 0xff) << 2);
        self.w
    }
}
#[doc = "Field `WAITI_DUMMY_CYCLELEN` reader - The dummy cycle length when wait flash idle(RDSR)."]
pub struct WAITI_DUMMY_CYCLELEN_R(crate::FieldReader<u8, u8>);
impl WAITI_DUMMY_CYCLELEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WAITI_DUMMY_CYCLELEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAITI_DUMMY_CYCLELEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAITI_DUMMY_CYCLELEN` writer - The dummy cycle length when wait flash idle(RDSR)."]
pub struct WAITI_DUMMY_CYCLELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITI_DUMMY_CYCLELEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 10)) | ((value as u32 & 0x3f) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Set this bit to enable auto-waiting flash idle operation when PP/SE/BE/CE/WRSR/PES command is sent."]
    #[inline(always)]
    pub fn waiti_en(&self) -> WAITI_EN_R {
        WAITI_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set this bit to enable DUMMY phase in auto wait flash idle transfer(RDSR)."]
    #[inline(always)]
    pub fn waiti_dummy(&self) -> WAITI_DUMMY_R {
        WAITI_DUMMY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:9 - The command value of auto wait flash idle transfer(RDSR)."]
    #[inline(always)]
    pub fn waiti_cmd(&self) -> WAITI_CMD_R {
        WAITI_CMD_R::new(((self.bits >> 2) & 0xff) as u8)
    }
    #[doc = "Bits 10:15 - The dummy cycle length when wait flash idle(RDSR)."]
    #[inline(always)]
    pub fn waiti_dummy_cyclelen(&self) -> WAITI_DUMMY_CYCLELEN_R {
        WAITI_DUMMY_CYCLELEN_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable auto-waiting flash idle operation when PP/SE/BE/CE/WRSR/PES command is sent."]
    #[inline(always)]
    pub fn waiti_en(&mut self) -> WAITI_EN_W {
        WAITI_EN_W { w: self }
    }
    #[doc = "Bit 1 - Set this bit to enable DUMMY phase in auto wait flash idle transfer(RDSR)."]
    #[inline(always)]
    pub fn waiti_dummy(&mut self) -> WAITI_DUMMY_W {
        WAITI_DUMMY_W { w: self }
    }
    #[doc = "Bits 2:9 - The command value of auto wait flash idle transfer(RDSR)."]
    #[inline(always)]
    pub fn waiti_cmd(&mut self) -> WAITI_CMD_W {
        WAITI_CMD_W { w: self }
    }
    #[doc = "Bits 10:15 - The dummy cycle length when wait flash idle(RDSR)."]
    #[inline(always)]
    pub fn waiti_dummy_cyclelen(&mut self) -> WAITI_DUMMY_CYCLELEN_W {
        WAITI_DUMMY_CYCLELEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 wait idle control register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_waiti_ctrl]
(index.html) module"]
pub struct FLASH_WAITI_CTRL_SPEC;
impl crate::RegisterSpec for FLASH_WAITI_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_waiti_ctrl::R]
(R) reader structure"]
impl crate::Readable for FLASH_WAITI_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_waiti_ctrl::W]
(W) writer structure"]
impl crate::Writable for FLASH_WAITI_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASH_WAITI_CTRL to value 0x14"]
impl crate::Resettable for FLASH_WAITI_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x14
    }
}