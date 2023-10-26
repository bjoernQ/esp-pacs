#[doc = "Register `LSCH%s_CONF0` reader"]
pub type R = crate::R<LSCH_CONF0_SPEC>;
#[doc = "Register `LSCH%s_CONF0` writer"]
pub type W = crate::W<LSCH_CONF0_SPEC>;
#[doc = "Field `TIMER_SEL` reader - There are four low speed timers the two bits are used to select one of them for low speed channel0. 2'b00: seletc lstimer0. 2'b01: select lstimer1. 2'b10: select lstimer2. 2'b11: select lstimer3."]
pub type TIMER_SEL_R = crate::FieldReader;
#[doc = "Field `TIMER_SEL` writer - There are four low speed timers the two bits are used to select one of them for low speed channel0. 2'b00: seletc lstimer0. 2'b01: select lstimer1. 2'b10: select lstimer2. 2'b11: select lstimer3."]
pub type TIMER_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SIG_OUT_EN` reader - This is the output enable control bit for low speed channel0."]
pub type SIG_OUT_EN_R = crate::BitReader;
#[doc = "Field `SIG_OUT_EN` writer - This is the output enable control bit for low speed channel0."]
pub type SIG_OUT_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IDLE_LV` reader - This bit is used to control the output value when low speed channel0 is off."]
pub type IDLE_LV_R = crate::BitReader;
#[doc = "Field `IDLE_LV` writer - This bit is used to control the output value when low speed channel0 is off."]
pub type IDLE_LV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PARA_UP` reader - This bit is used to update register LEDC_LSCH0_HPOINT and LEDC_LSCH0_DUTY for low speed channel0."]
pub type PARA_UP_R = crate::BitReader;
#[doc = "Field `PARA_UP` writer - This bit is used to update register LEDC_LSCH0_HPOINT and LEDC_LSCH0_DUTY for low speed channel0."]
pub type PARA_UP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - There are four low speed timers the two bits are used to select one of them for low speed channel0. 2'b00: seletc lstimer0. 2'b01: select lstimer1. 2'b10: select lstimer2. 2'b11: select lstimer3."]
    #[inline(always)]
    pub fn timer_sel(&self) -> TIMER_SEL_R {
        TIMER_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - This is the output enable control bit for low speed channel0."]
    #[inline(always)]
    pub fn sig_out_en(&self) -> SIG_OUT_EN_R {
        SIG_OUT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit is used to control the output value when low speed channel0 is off."]
    #[inline(always)]
    pub fn idle_lv(&self) -> IDLE_LV_R {
        IDLE_LV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit is used to update register LEDC_LSCH0_HPOINT and LEDC_LSCH0_DUTY for low speed channel0."]
    #[inline(always)]
    pub fn para_up(&self) -> PARA_UP_R {
        PARA_UP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LSCH_CONF0")
            .field("timer_sel", &format_args!("{}", self.timer_sel().bits()))
            .field("sig_out_en", &format_args!("{}", self.sig_out_en().bit()))
            .field("idle_lv", &format_args!("{}", self.idle_lv().bit()))
            .field("para_up", &format_args!("{}", self.para_up().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LSCH_CONF0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - There are four low speed timers the two bits are used to select one of them for low speed channel0. 2'b00: seletc lstimer0. 2'b01: select lstimer1. 2'b10: select lstimer2. 2'b11: select lstimer3."]
    #[inline(always)]
    #[must_use]
    pub fn timer_sel(&mut self) -> TIMER_SEL_W<LSCH_CONF0_SPEC, 0> {
        TIMER_SEL_W::new(self)
    }
    #[doc = "Bit 2 - This is the output enable control bit for low speed channel0."]
    #[inline(always)]
    #[must_use]
    pub fn sig_out_en(&mut self) -> SIG_OUT_EN_W<LSCH_CONF0_SPEC, 2> {
        SIG_OUT_EN_W::new(self)
    }
    #[doc = "Bit 3 - This bit is used to control the output value when low speed channel0 is off."]
    #[inline(always)]
    #[must_use]
    pub fn idle_lv(&mut self) -> IDLE_LV_W<LSCH_CONF0_SPEC, 3> {
        IDLE_LV_W::new(self)
    }
    #[doc = "Bit 4 - This bit is used to update register LEDC_LSCH0_HPOINT and LEDC_LSCH0_DUTY for low speed channel0."]
    #[inline(always)]
    #[must_use]
    pub fn para_up(&mut self) -> PARA_UP_W<LSCH_CONF0_SPEC, 4> {
        PARA_UP_W::new(self)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsch_conf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsch_conf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LSCH_CONF0_SPEC;
impl crate::RegisterSpec for LSCH_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsch_conf0::R`](R) reader structure"]
impl crate::Readable for LSCH_CONF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lsch_conf0::W`](W) writer structure"]
impl crate::Writable for LSCH_CONF0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LSCH%s_CONF0 to value 0"]
impl crate::Resettable for LSCH_CONF0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
