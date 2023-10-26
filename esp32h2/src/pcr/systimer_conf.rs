#[doc = "Register `SYSTIMER_CONF` reader"]
pub type R = crate::R<SYSTIMER_CONF_SPEC>;
#[doc = "Register `SYSTIMER_CONF` writer"]
pub type W = crate::W<SYSTIMER_CONF_SPEC>;
#[doc = "Field `SYSTIMER_CLK_EN` reader - Set 1 to enable systimer apb clock"]
pub type SYSTIMER_CLK_EN_R = crate::BitReader;
#[doc = "Field `SYSTIMER_CLK_EN` writer - Set 1 to enable systimer apb clock"]
pub type SYSTIMER_CLK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSTIMER_RST_EN` reader - Set 0 to reset systimer module"]
pub type SYSTIMER_RST_EN_R = crate::BitReader;
#[doc = "Field `SYSTIMER_RST_EN` writer - Set 0 to reset systimer module"]
pub type SYSTIMER_RST_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSTIMER_READY` reader - Query this field after reset systimer module"]
pub type SYSTIMER_READY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set 1 to enable systimer apb clock"]
    #[inline(always)]
    pub fn systimer_clk_en(&self) -> SYSTIMER_CLK_EN_R {
        SYSTIMER_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset systimer module"]
    #[inline(always)]
    pub fn systimer_rst_en(&self) -> SYSTIMER_RST_EN_R {
        SYSTIMER_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Query this field after reset systimer module"]
    #[inline(always)]
    pub fn systimer_ready(&self) -> SYSTIMER_READY_R {
        SYSTIMER_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSTIMER_CONF")
            .field(
                "systimer_clk_en",
                &format_args!("{}", self.systimer_clk_en().bit()),
            )
            .field(
                "systimer_rst_en",
                &format_args!("{}", self.systimer_rst_en().bit()),
            )
            .field(
                "systimer_ready",
                &format_args!("{}", self.systimer_ready().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SYSTIMER_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable systimer apb clock"]
    #[inline(always)]
    #[must_use]
    pub fn systimer_clk_en(&mut self) -> SYSTIMER_CLK_EN_W<SYSTIMER_CONF_SPEC, 0> {
        SYSTIMER_CLK_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set 0 to reset systimer module"]
    #[inline(always)]
    #[must_use]
    pub fn systimer_rst_en(&mut self) -> SYSTIMER_RST_EN_W<SYSTIMER_CONF_SPEC, 1> {
        SYSTIMER_RST_EN_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYSTIMER configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`systimer_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`systimer_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSTIMER_CONF_SPEC;
impl crate::RegisterSpec for SYSTIMER_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`systimer_conf::R`](R) reader structure"]
impl crate::Readable for SYSTIMER_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`systimer_conf::W`](W) writer structure"]
impl crate::Writable for SYSTIMER_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSTIMER_CONF to value 0x05"]
impl crate::Resettable for SYSTIMER_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
