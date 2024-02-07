#[doc = "Register `IN_STATE_CH%s` reader"]
pub type R = crate::R<IN_STATE_CH_SPEC>;
#[doc = "Field `INLINK_DSCR_ADDR_CH` reader - This register stores the current inlink descriptor's address."]
pub type INLINK_DSCR_ADDR_CH_R = crate::FieldReader<u32>;
#[doc = "Field `IN_DSCR_STATE_CH` reader - reserved"]
pub type IN_DSCR_STATE_CH_R = crate::FieldReader;
#[doc = "Field `IN_STATE_CH` reader - reserved"]
pub type IN_STATE_CH_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:17 - This register stores the current inlink descriptor's address."]
    #[inline(always)]
    pub fn inlink_dscr_addr_ch(&self) -> INLINK_DSCR_ADDR_CH_R {
        INLINK_DSCR_ADDR_CH_R::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:19 - reserved"]
    #[inline(always)]
    pub fn in_dscr_state_ch(&self) -> IN_DSCR_STATE_CH_R {
        IN_DSCR_STATE_CH_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - reserved"]
    #[inline(always)]
    pub fn in_state_ch(&self) -> IN_STATE_CH_R {
        IN_STATE_CH_R::new(((self.bits >> 20) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_STATE_CH")
            .field(
                "inlink_dscr_addr_ch",
                &format_args!("{}", self.inlink_dscr_addr_ch().bits()),
            )
            .field(
                "in_dscr_state_ch",
                &format_args!("{}", self.in_dscr_state_ch().bits()),
            )
            .field(
                "in_state_ch",
                &format_args!("{}", self.in_state_ch().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_STATE_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Receive status of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_state_ch::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_STATE_CH_SPEC;
impl crate::RegisterSpec for IN_STATE_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_state_ch::R`](R) reader structure"]
impl crate::Readable for IN_STATE_CH_SPEC {}
#[doc = "`reset()` method sets IN_STATE_CH%s to value 0"]
impl crate::Resettable for IN_STATE_CH_SPEC {
    const RESET_VALUE: u32 = 0;
}