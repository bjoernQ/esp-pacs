#[doc = "Register `EDMA_PMS_SPI2` reader"]
pub struct R(crate::R<EDMA_PMS_SPI2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EDMA_PMS_SPI2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EDMA_PMS_SPI2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EDMA_PMS_SPI2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EDMA_PMS_SPI2` writer"]
pub struct W(crate::W<EDMA_PMS_SPI2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EDMA_PMS_SPI2_SPEC>;
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
impl From<crate::W<EDMA_PMS_SPI2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EDMA_PMS_SPI2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATTR1` reader - This field is used to configure the permission of SPI2 accessing address, which is larger than boundary 0 and less than boundary 1, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
pub struct ATTR1_R(crate::FieldReader<u8, u8>);
impl ATTR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ATTR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATTR1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATTR1` writer - This field is used to configure the permission of SPI2 accessing address, which is larger than boundary 0 and less than boundary 1, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
pub struct ATTR1_W<'a> {
    w: &'a mut W,
}
impl<'a> ATTR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `ATTR2` reader - This field is used to configure the permission of SPI2 accessing address, which is larger than boundary 1 and less than boundary 2, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
pub struct ATTR2_R(crate::FieldReader<u8, u8>);
impl ATTR2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ATTR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATTR2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATTR2` writer - This field is used to configure the permission of SPI2 accessing address, which is larger than boundary 1 and less than boundary 2, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
pub struct ATTR2_W<'a> {
    w: &'a mut W,
}
impl<'a> ATTR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - This field is used to configure the permission of SPI2 accessing address, which is larger than boundary 0 and less than boundary 1, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
    #[inline(always)]
    pub fn attr1(&self) -> ATTR1_R {
        ATTR1_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - This field is used to configure the permission of SPI2 accessing address, which is larger than boundary 1 and less than boundary 2, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
    #[inline(always)]
    pub fn attr2(&self) -> ATTR2_R {
        ATTR2_R::new(((self.bits >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - This field is used to configure the permission of SPI2 accessing address, which is larger than boundary 0 and less than boundary 1, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
    #[inline(always)]
    pub fn attr1(&mut self) -> ATTR1_W {
        ATTR1_W { w: self }
    }
    #[doc = "Bits 2:3 - This field is used to configure the permission of SPI2 accessing address, which is larger than boundary 1 and less than boundary 2, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
    #[inline(always)]
    pub fn attr2(&mut self) -> ATTR2_W {
        ATTR2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EDMA-SPI2 permission control register.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [edma_pms_spi2]
(index.html) module"]
pub struct EDMA_PMS_SPI2_SPEC;
impl crate::RegisterSpec for EDMA_PMS_SPI2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [edma_pms_spi2::R]
(R) reader structure"]
impl crate::Readable for EDMA_PMS_SPI2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [edma_pms_spi2::W]
(W) writer structure"]
impl crate::Writable for EDMA_PMS_SPI2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EDMA_PMS_SPI2 to value 0x0f"]
impl crate::Resettable for EDMA_PMS_SPI2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}