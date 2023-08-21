#[doc = "Register `SLOW_CLK_CONF` reader"]
pub type R = crate::R<SLOW_CLK_CONF_SPEC>;
#[doc = "Register `SLOW_CLK_CONF` writer"]
pub type W = crate::W<SLOW_CLK_CONF_SPEC>;
#[doc = "Field `ANA_CLK_DIV_VLD` reader - Synchronizes the reg_rtc_ana_clk_div bus. Note that you have to invalidate the bus before switching clock, and validate the new clock."]
pub type ANA_CLK_DIV_VLD_R = crate::BitReader;
#[doc = "Field `ANA_CLK_DIV_VLD` writer - Synchronizes the reg_rtc_ana_clk_div bus. Note that you have to invalidate the bus before switching clock, and validate the new clock."]
pub type ANA_CLK_DIV_VLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ANA_CLK_DIV` reader - Set the rtc_clk divider."]
pub type ANA_CLK_DIV_R = crate::FieldReader;
#[doc = "Field `ANA_CLK_DIV` writer - Set the rtc_clk divider."]
pub type ANA_CLK_DIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SLOW_CLK_NEXT_EDGE` reader - "]
pub type SLOW_CLK_NEXT_EDGE_R = crate::BitReader;
#[doc = "Field `SLOW_CLK_NEXT_EDGE` writer - "]
pub type SLOW_CLK_NEXT_EDGE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 22 - Synchronizes the reg_rtc_ana_clk_div bus. Note that you have to invalidate the bus before switching clock, and validate the new clock."]
    #[inline(always)]
    pub fn ana_clk_div_vld(&self) -> ANA_CLK_DIV_VLD_R {
        ANA_CLK_DIV_VLD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:30 - Set the rtc_clk divider."]
    #[inline(always)]
    pub fn ana_clk_div(&self) -> ANA_CLK_DIV_R {
        ANA_CLK_DIV_R::new(((self.bits >> 23) & 0xff) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn slow_clk_next_edge(&self) -> SLOW_CLK_NEXT_EDGE_R {
        SLOW_CLK_NEXT_EDGE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLOW_CLK_CONF")
            .field(
                "ana_clk_div_vld",
                &format_args!("{}", self.ana_clk_div_vld().bit()),
            )
            .field(
                "ana_clk_div",
                &format_args!("{}", self.ana_clk_div().bits()),
            )
            .field(
                "slow_clk_next_edge",
                &format_args!("{}", self.slow_clk_next_edge().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLOW_CLK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 22 - Synchronizes the reg_rtc_ana_clk_div bus. Note that you have to invalidate the bus before switching clock, and validate the new clock."]
    #[inline(always)]
    #[must_use]
    pub fn ana_clk_div_vld(&mut self) -> ANA_CLK_DIV_VLD_W<SLOW_CLK_CONF_SPEC, 22> {
        ANA_CLK_DIV_VLD_W::new(self)
    }
    #[doc = "Bits 23:30 - Set the rtc_clk divider."]
    #[inline(always)]
    #[must_use]
    pub fn ana_clk_div(&mut self) -> ANA_CLK_DIV_W<SLOW_CLK_CONF_SPEC, 23> {
        ANA_CLK_DIV_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn slow_clk_next_edge(&mut self) -> SLOW_CLK_NEXT_EDGE_W<SLOW_CLK_CONF_SPEC, 31> {
        SLOW_CLK_NEXT_EDGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RTC slow clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slow_clk_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slow_clk_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLOW_CLK_CONF_SPEC;
impl crate::RegisterSpec for SLOW_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slow_clk_conf::R`](R) reader structure"]
impl crate::Readable for SLOW_CLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slow_clk_conf::W`](W) writer structure"]
impl crate::Writable for SLOW_CLK_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLOW_CLK_CONF to value 0x0040_0000"]
impl crate::Resettable for SLOW_CLK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0040_0000;
}
