#[doc = "Register `PERIP_RST_EN1` reader"]
pub struct R(crate::R<PERIP_RST_EN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIP_RST_EN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIP_RST_EN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIP_RST_EN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERIP_RST_EN1` writer"]
pub struct W(crate::W<PERIP_RST_EN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERIP_RST_EN1_SPEC>;
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
impl From<crate::W<PERIP_RST_EN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERIP_RST_EN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRYPTO_AES_RST` reader - Set this bit to reset cryptography AES."]
pub type CRYPTO_AES_RST_R = crate::BitReader<bool>;
#[doc = "Field `CRYPTO_AES_RST` writer - Set this bit to reset cryptography AES."]
pub type CRYPTO_AES_RST_W<'a> = crate::BitWriter<'a, u32, PERIP_RST_EN1_SPEC, bool, 1>;
#[doc = "Field `CRYPTO_SHA_RST` reader - Set this bit to reset cryptography SHA."]
pub type CRYPTO_SHA_RST_R = crate::BitReader<bool>;
#[doc = "Field `CRYPTO_SHA_RST` writer - Set this bit to reset cryptography SHA."]
pub type CRYPTO_SHA_RST_W<'a> = crate::BitWriter<'a, u32, PERIP_RST_EN1_SPEC, bool, 2>;
#[doc = "Field `CRYPTO_RSA_RST` reader - Set this bit to reset cryptography RSA."]
pub type CRYPTO_RSA_RST_R = crate::BitReader<bool>;
#[doc = "Field `CRYPTO_RSA_RST` writer - Set this bit to reset cryptography RSA."]
pub type CRYPTO_RSA_RST_W<'a> = crate::BitWriter<'a, u32, PERIP_RST_EN1_SPEC, bool, 3>;
#[doc = "Field `CRYPTO_DS_RST` reader - Set this bit to reset cryptography digital signature."]
pub type CRYPTO_DS_RST_R = crate::BitReader<bool>;
#[doc = "Field `CRYPTO_DS_RST` writer - Set this bit to reset cryptography digital signature."]
pub type CRYPTO_DS_RST_W<'a> = crate::BitWriter<'a, u32, PERIP_RST_EN1_SPEC, bool, 4>;
#[doc = "Field `CRYPTO_HMAC_RST` reader - Set this bit to reset cryptography HMAC."]
pub type CRYPTO_HMAC_RST_R = crate::BitReader<bool>;
#[doc = "Field `CRYPTO_HMAC_RST` writer - Set this bit to reset cryptography HMAC."]
pub type CRYPTO_HMAC_RST_W<'a> = crate::BitWriter<'a, u32, PERIP_RST_EN1_SPEC, bool, 5>;
#[doc = "Field `CRYPTO_DMA_RST` reader - Set this bit to reset cryptography DMA."]
pub type CRYPTO_DMA_RST_R = crate::BitReader<bool>;
#[doc = "Field `CRYPTO_DMA_RST` writer - Set this bit to reset cryptography DMA."]
pub type CRYPTO_DMA_RST_W<'a> = crate::BitWriter<'a, u32, PERIP_RST_EN1_SPEC, bool, 6>;
impl R {
    #[doc = "Bit 1 - Set this bit to reset cryptography AES."]
    #[inline(always)]
    pub fn crypto_aes_rst(&self) -> CRYPTO_AES_RST_R {
        CRYPTO_AES_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to reset cryptography SHA."]
    #[inline(always)]
    pub fn crypto_sha_rst(&self) -> CRYPTO_SHA_RST_R {
        CRYPTO_SHA_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to reset cryptography RSA."]
    #[inline(always)]
    pub fn crypto_rsa_rst(&self) -> CRYPTO_RSA_RST_R {
        CRYPTO_RSA_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to reset cryptography digital signature."]
    #[inline(always)]
    pub fn crypto_ds_rst(&self) -> CRYPTO_DS_RST_R {
        CRYPTO_DS_RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to reset cryptography HMAC."]
    #[inline(always)]
    pub fn crypto_hmac_rst(&self) -> CRYPTO_HMAC_RST_R {
        CRYPTO_HMAC_RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to reset cryptography DMA."]
    #[inline(always)]
    pub fn crypto_dma_rst(&self) -> CRYPTO_DMA_RST_R {
        CRYPTO_DMA_RST_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Set this bit to reset cryptography AES."]
    #[inline(always)]
    pub fn crypto_aes_rst(&mut self) -> CRYPTO_AES_RST_W {
        CRYPTO_AES_RST_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to reset cryptography SHA."]
    #[inline(always)]
    pub fn crypto_sha_rst(&mut self) -> CRYPTO_SHA_RST_W {
        CRYPTO_SHA_RST_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to reset cryptography RSA."]
    #[inline(always)]
    pub fn crypto_rsa_rst(&mut self) -> CRYPTO_RSA_RST_W {
        CRYPTO_RSA_RST_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to reset cryptography digital signature."]
    #[inline(always)]
    pub fn crypto_ds_rst(&mut self) -> CRYPTO_DS_RST_W {
        CRYPTO_DS_RST_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to reset cryptography HMAC."]
    #[inline(always)]
    pub fn crypto_hmac_rst(&mut self) -> CRYPTO_HMAC_RST_W {
        CRYPTO_HMAC_RST_W::new(self)
    }
    #[doc = "Bit 6 - Set this bit to reset cryptography DMA."]
    #[inline(always)]
    pub fn crypto_dma_rst(&mut self) -> CRYPTO_DMA_RST_W {
        CRYPTO_DMA_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System peripheral (hardware accelerators) reset register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perip_rst_en1](index.html) module"]
pub struct PERIP_RST_EN1_SPEC;
impl crate::RegisterSpec for PERIP_RST_EN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perip_rst_en1::R](R) reader structure"]
impl crate::Readable for PERIP_RST_EN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perip_rst_en1::W](W) writer structure"]
impl crate::Writable for PERIP_RST_EN1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERIP_RST_EN1 to value 0x7e"]
impl crate::Resettable for PERIP_RST_EN1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7e
    }
}
