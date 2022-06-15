#[doc = "Register `SDIO_ACT_CONF` reader"]
pub struct R(crate::R<SDIO_ACT_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDIO_ACT_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDIO_ACT_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDIO_ACT_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDIO_ACT_CONF` writer"]
pub struct W(crate::W<SDIO_ACT_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDIO_ACT_CONF_SPEC>;
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
impl From<crate::W<SDIO_ACT_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDIO_ACT_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDIO_ACT_DNUM` reader - No public"]
pub type SDIO_ACT_DNUM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SDIO_ACT_DNUM` writer - No public"]
pub type SDIO_ACT_DNUM_W<'a> = crate::FieldWriter<'a, u32, SDIO_ACT_CONF_SPEC, u16, u16, 10, 22>;
impl R {
    #[doc = "Bits 22:31 - No public"]
    #[inline(always)]
    pub fn sdio_act_dnum(&self) -> SDIO_ACT_DNUM_R {
        SDIO_ACT_DNUM_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 22:31 - No public"]
    #[inline(always)]
    pub fn sdio_act_dnum(&mut self) -> SDIO_ACT_DNUM_W {
        SDIO_ACT_DNUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No public\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdio_act_conf](index.html) module"]
pub struct SDIO_ACT_CONF_SPEC;
impl crate::RegisterSpec for SDIO_ACT_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdio_act_conf::R](R) reader structure"]
impl crate::Readable for SDIO_ACT_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdio_act_conf::W](W) writer structure"]
impl crate::Writable for SDIO_ACT_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDIO_ACT_CONF to value 0"]
impl crate::Resettable for SDIO_ACT_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
