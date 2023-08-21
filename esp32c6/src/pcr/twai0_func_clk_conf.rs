#[doc = "Register `TWAI0_FUNC_CLK_CONF` reader"]
pub type R = crate::R<TWAI0_FUNC_CLK_CONF_SPEC>;
#[doc = "Register `TWAI0_FUNC_CLK_CONF` writer"]
pub type W = crate::W<TWAI0_FUNC_CLK_CONF_SPEC>;
#[doc = "Field `TWAI0_FUNC_CLK_SEL` reader - set this field to select clock-source. 0(default): XTAL, 1: FOSC."]
pub type TWAI0_FUNC_CLK_SEL_R = crate::BitReader;
#[doc = "Field `TWAI0_FUNC_CLK_SEL` writer - set this field to select clock-source. 0(default): XTAL, 1: FOSC."]
pub type TWAI0_FUNC_CLK_SEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TWAI0_FUNC_CLK_EN` reader - Set 1 to enable twai0 function clock"]
pub type TWAI0_FUNC_CLK_EN_R = crate::BitReader;
#[doc = "Field `TWAI0_FUNC_CLK_EN` writer - Set 1 to enable twai0 function clock"]
pub type TWAI0_FUNC_CLK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 20 - set this field to select clock-source. 0(default): XTAL, 1: FOSC."]
    #[inline(always)]
    pub fn twai0_func_clk_sel(&self) -> TWAI0_FUNC_CLK_SEL_R {
        TWAI0_FUNC_CLK_SEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Set 1 to enable twai0 function clock"]
    #[inline(always)]
    pub fn twai0_func_clk_en(&self) -> TWAI0_FUNC_CLK_EN_R {
        TWAI0_FUNC_CLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TWAI0_FUNC_CLK_CONF")
            .field(
                "twai0_func_clk_sel",
                &format_args!("{}", self.twai0_func_clk_sel().bit()),
            )
            .field(
                "twai0_func_clk_en",
                &format_args!("{}", self.twai0_func_clk_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TWAI0_FUNC_CLK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 20 - set this field to select clock-source. 0(default): XTAL, 1: FOSC."]
    #[inline(always)]
    #[must_use]
    pub fn twai0_func_clk_sel(&mut self) -> TWAI0_FUNC_CLK_SEL_W<TWAI0_FUNC_CLK_CONF_SPEC, 20> {
        TWAI0_FUNC_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 22 - Set 1 to enable twai0 function clock"]
    #[inline(always)]
    #[must_use]
    pub fn twai0_func_clk_en(&mut self) -> TWAI0_FUNC_CLK_EN_W<TWAI0_FUNC_CLK_CONF_SPEC, 22> {
        TWAI0_FUNC_CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "TWAI0_FUNC_CLK configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twai0_func_clk_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twai0_func_clk_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWAI0_FUNC_CLK_CONF_SPEC;
impl crate::RegisterSpec for TWAI0_FUNC_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`twai0_func_clk_conf::R`](R) reader structure"]
impl crate::Readable for TWAI0_FUNC_CLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`twai0_func_clk_conf::W`](W) writer structure"]
impl crate::Writable for TWAI0_FUNC_CLK_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TWAI0_FUNC_CLK_CONF to value 0x0040_0000"]
impl crate::Resettable for TWAI0_FUNC_CLK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0040_0000;
}
