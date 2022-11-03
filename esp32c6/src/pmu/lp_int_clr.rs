#[doc = "Register `LP_INT_CLR` writer"]
pub struct W(crate::W<LP_INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LP_INT_CLR_SPEC>;
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
impl From<crate::W<LP_INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LP_INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LP_CPU_WAKEUP_INT_CLR` writer - need_des"]
pub type LP_CPU_WAKEUP_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LP_INT_CLR_SPEC, bool, O>;
#[doc = "Field `MODEM_SWITCH_ACTIVE_END_INT_CLR` writer - need_des"]
pub type MODEM_SWITCH_ACTIVE_END_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LP_INT_CLR_SPEC, bool, O>;
#[doc = "Field `SLEEP_SWITCH_ACTIVE_END_INT_CLR` writer - need_des"]
pub type SLEEP_SWITCH_ACTIVE_END_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LP_INT_CLR_SPEC, bool, O>;
#[doc = "Field `SLEEP_SWITCH_MODEM_END_INT_CLR` writer - need_des"]
pub type SLEEP_SWITCH_MODEM_END_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LP_INT_CLR_SPEC, bool, O>;
#[doc = "Field `MODEM_SWITCH_SLEEP_END_INT_CLR` writer - need_des"]
pub type MODEM_SWITCH_SLEEP_END_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LP_INT_CLR_SPEC, bool, O>;
#[doc = "Field `ACTIVE_SWITCH_SLEEP_END_INT_CLR` writer - need_des"]
pub type ACTIVE_SWITCH_SLEEP_END_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LP_INT_CLR_SPEC, bool, O>;
#[doc = "Field `MODEM_SWITCH_ACTIVE_START_INT_CLR` writer - need_des"]
pub type MODEM_SWITCH_ACTIVE_START_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LP_INT_CLR_SPEC, bool, O>;
#[doc = "Field `SLEEP_SWITCH_ACTIVE_START_INT_CLR` writer - need_des"]
pub type SLEEP_SWITCH_ACTIVE_START_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LP_INT_CLR_SPEC, bool, O>;
#[doc = "Field `SLEEP_SWITCH_MODEM_START_INT_CLR` writer - need_des"]
pub type SLEEP_SWITCH_MODEM_START_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LP_INT_CLR_SPEC, bool, O>;
#[doc = "Field `MODEM_SWITCH_SLEEP_START_INT_CLR` writer - need_des"]
pub type MODEM_SWITCH_SLEEP_START_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LP_INT_CLR_SPEC, bool, O>;
#[doc = "Field `ACTIVE_SWITCH_SLEEP_START_INT_CLR` writer - need_des"]
pub type ACTIVE_SWITCH_SLEEP_START_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LP_INT_CLR_SPEC, bool, O>;
#[doc = "Field `HP_SW_TRIGGER_INT_CLR` writer - need_des"]
pub type HP_SW_TRIGGER_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LP_INT_CLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 20 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_cpu_wakeup_int_clr(&mut self) -> LP_CPU_WAKEUP_INT_CLR_W<20> {
        LP_CPU_WAKEUP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn modem_switch_active_end_int_clr(&mut self) -> MODEM_SWITCH_ACTIVE_END_INT_CLR_W<21> {
        MODEM_SWITCH_ACTIVE_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_switch_active_end_int_clr(&mut self) -> SLEEP_SWITCH_ACTIVE_END_INT_CLR_W<22> {
        SLEEP_SWITCH_ACTIVE_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_switch_modem_end_int_clr(&mut self) -> SLEEP_SWITCH_MODEM_END_INT_CLR_W<23> {
        SLEEP_SWITCH_MODEM_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn modem_switch_sleep_end_int_clr(&mut self) -> MODEM_SWITCH_SLEEP_END_INT_CLR_W<24> {
        MODEM_SWITCH_SLEEP_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn active_switch_sleep_end_int_clr(&mut self) -> ACTIVE_SWITCH_SLEEP_END_INT_CLR_W<25> {
        ACTIVE_SWITCH_SLEEP_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn modem_switch_active_start_int_clr(&mut self) -> MODEM_SWITCH_ACTIVE_START_INT_CLR_W<26> {
        MODEM_SWITCH_ACTIVE_START_INT_CLR_W::new(self)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_switch_active_start_int_clr(&mut self) -> SLEEP_SWITCH_ACTIVE_START_INT_CLR_W<27> {
        SLEEP_SWITCH_ACTIVE_START_INT_CLR_W::new(self)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_switch_modem_start_int_clr(&mut self) -> SLEEP_SWITCH_MODEM_START_INT_CLR_W<28> {
        SLEEP_SWITCH_MODEM_START_INT_CLR_W::new(self)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn modem_switch_sleep_start_int_clr(&mut self) -> MODEM_SWITCH_SLEEP_START_INT_CLR_W<29> {
        MODEM_SWITCH_SLEEP_START_INT_CLR_W::new(self)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn active_switch_sleep_start_int_clr(&mut self) -> ACTIVE_SWITCH_SLEEP_START_INT_CLR_W<30> {
        ACTIVE_SWITCH_SLEEP_START_INT_CLR_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sw_trigger_int_clr(&mut self) -> HP_SW_TRIGGER_INT_CLR_W<31> {
        HP_SW_TRIGGER_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lp_int_clr](index.html) module"]
pub struct LP_INT_CLR_SPEC;
impl crate::RegisterSpec for LP_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [lp_int_clr::W](W) writer structure"]
impl crate::Writable for LP_INT_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LP_INT_CLR to value 0"]
impl crate::Resettable for LP_INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
