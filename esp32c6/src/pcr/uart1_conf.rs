#[doc = "Register `UART1_CONF` reader"]
pub struct R(crate::R<UART1_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART1_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART1_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART1_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART1_CONF` writer"]
pub struct W(crate::W<UART1_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART1_CONF_SPEC>;
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
impl From<crate::W<UART1_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART1_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART1_CLK_EN` reader - Set 1 to enable uart1 apb clock"]
pub type UART1_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `UART1_CLK_EN` writer - Set 1 to enable uart1 apb clock"]
pub type UART1_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART1_CONF_SPEC, bool, O>;
#[doc = "Field `UART1_RST_EN` reader - Set 0 to reset uart1 module"]
pub type UART1_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `UART1_RST_EN` writer - Set 0 to reset uart1 module"]
pub type UART1_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART1_CONF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Set 1 to enable uart1 apb clock"]
    #[inline(always)]
    pub fn uart1_clk_en(&self) -> UART1_CLK_EN_R {
        UART1_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset uart1 module"]
    #[inline(always)]
    pub fn uart1_rst_en(&self) -> UART1_RST_EN_R {
        UART1_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable uart1 apb clock"]
    #[inline(always)]
    #[must_use]
    pub fn uart1_clk_en(&mut self) -> UART1_CLK_EN_W<0> {
        UART1_CLK_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set 0 to reset uart1 module"]
    #[inline(always)]
    #[must_use]
    pub fn uart1_rst_en(&mut self) -> UART1_RST_EN_W<1> {
        UART1_RST_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART1 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart1_conf](index.html) module"]
pub struct UART1_CONF_SPEC;
impl crate::RegisterSpec for UART1_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart1_conf::R](R) reader structure"]
impl crate::Readable for UART1_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart1_conf::W](W) writer structure"]
impl crate::Writable for UART1_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UART1_CONF to value 0x01"]
impl crate::Resettable for UART1_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
