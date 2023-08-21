#[doc = "Register `SLP_TIMER1` reader"]
pub type R = crate::R<SLP_TIMER1_SPEC>;
#[doc = "Register `SLP_TIMER1` writer"]
pub type W = crate::W<SLP_TIMER1_SPEC>;
#[doc = "Field `SLP_VAL_HI` reader - Sets the higher 16 bits of the trigger threshold for the RTC timer."]
pub type SLP_VAL_HI_R = crate::FieldReader<u16>;
#[doc = "Field `SLP_VAL_HI` writer - Sets the higher 16 bits of the trigger threshold for the RTC timer."]
pub type SLP_VAL_HI_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `MAIN_TIMER_ALARM_EN` writer - Sets this bit to enable the timer alarm."]
pub type MAIN_TIMER_ALARM_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:15 - Sets the higher 16 bits of the trigger threshold for the RTC timer."]
    #[inline(always)]
    pub fn slp_val_hi(&self) -> SLP_VAL_HI_R {
        SLP_VAL_HI_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLP_TIMER1")
            .field("slp_val_hi", &format_args!("{}", self.slp_val_hi().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLP_TIMER1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Sets the higher 16 bits of the trigger threshold for the RTC timer."]
    #[inline(always)]
    #[must_use]
    pub fn slp_val_hi(&mut self) -> SLP_VAL_HI_W<SLP_TIMER1_SPEC, 0> {
        SLP_VAL_HI_W::new(self)
    }
    #[doc = "Bit 16 - Sets this bit to enable the timer alarm."]
    #[inline(always)]
    #[must_use]
    pub fn main_timer_alarm_en(&mut self) -> MAIN_TIMER_ALARM_EN_W<SLP_TIMER1_SPEC, 16> {
        MAIN_TIMER_ALARM_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RTC timer threshold register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slp_timer1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_timer1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLP_TIMER1_SPEC;
impl crate::RegisterSpec for SLP_TIMER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slp_timer1::R`](R) reader structure"]
impl crate::Readable for SLP_TIMER1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slp_timer1::W`](W) writer structure"]
impl crate::Writable for SLP_TIMER1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLP_TIMER1 to value 0"]
impl crate::Resettable for SLP_TIMER1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
