#[doc = "Register `TIMERGROUP1_TIMER_CLK_CONF` reader"]
pub type R = crate::R<TIMERGROUP1_TIMER_CLK_CONF_SPEC>;
#[doc = "Register `TIMERGROUP1_TIMER_CLK_CONF` writer"]
pub type W = crate::W<TIMERGROUP1_TIMER_CLK_CONF_SPEC>;
#[doc = "Field `TG1_TIMER_CLK_SEL` reader - set this field to select clock-source. 0(default): XTAL, 1: 80MHz, 2: FOSC, 3: reserved."]
pub type TG1_TIMER_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `TG1_TIMER_CLK_SEL` writer - set this field to select clock-source. 0(default): XTAL, 1: 80MHz, 2: FOSC, 3: reserved."]
pub type TG1_TIMER_CLK_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `TG1_TIMER_CLK_EN` reader - Set 1 to enable timer_group1 timer clock"]
pub type TG1_TIMER_CLK_EN_R = crate::BitReader;
#[doc = "Field `TG1_TIMER_CLK_EN` writer - Set 1 to enable timer_group1 timer clock"]
pub type TG1_TIMER_CLK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 20:21 - set this field to select clock-source. 0(default): XTAL, 1: 80MHz, 2: FOSC, 3: reserved."]
    #[inline(always)]
    pub fn tg1_timer_clk_sel(&self) -> TG1_TIMER_CLK_SEL_R {
        TG1_TIMER_CLK_SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Set 1 to enable timer_group1 timer clock"]
    #[inline(always)]
    pub fn tg1_timer_clk_en(&self) -> TG1_TIMER_CLK_EN_R {
        TG1_TIMER_CLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMERGROUP1_TIMER_CLK_CONF")
            .field(
                "tg1_timer_clk_sel",
                &format_args!("{}", self.tg1_timer_clk_sel().bits()),
            )
            .field(
                "tg1_timer_clk_en",
                &format_args!("{}", self.tg1_timer_clk_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMERGROUP1_TIMER_CLK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 20:21 - set this field to select clock-source. 0(default): XTAL, 1: 80MHz, 2: FOSC, 3: reserved."]
    #[inline(always)]
    #[must_use]
    pub fn tg1_timer_clk_sel(
        &mut self,
    ) -> TG1_TIMER_CLK_SEL_W<TIMERGROUP1_TIMER_CLK_CONF_SPEC, 20> {
        TG1_TIMER_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 22 - Set 1 to enable timer_group1 timer clock"]
    #[inline(always)]
    #[must_use]
    pub fn tg1_timer_clk_en(&mut self) -> TG1_TIMER_CLK_EN_W<TIMERGROUP1_TIMER_CLK_CONF_SPEC, 22> {
        TG1_TIMER_CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "TIMERGROUP1_TIMER_CLK configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timergroup1_timer_clk_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timergroup1_timer_clk_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMERGROUP1_TIMER_CLK_CONF_SPEC;
impl crate::RegisterSpec for TIMERGROUP1_TIMER_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timergroup1_timer_clk_conf::R`](R) reader structure"]
impl crate::Readable for TIMERGROUP1_TIMER_CLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timergroup1_timer_clk_conf::W`](W) writer structure"]
impl crate::Writable for TIMERGROUP1_TIMER_CLK_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMERGROUP1_TIMER_CLK_CONF to value 0x0040_0000"]
impl crate::Resettable for TIMERGROUP1_TIMER_CLK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0040_0000;
}
