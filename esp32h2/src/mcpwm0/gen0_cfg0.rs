#[doc = "Register `GEN0_CFG0` reader"]
pub type R = crate::R<GEN0_CFG0_SPEC>;
#[doc = "Register `GEN0_CFG0` writer"]
pub type W = crate::W<GEN0_CFG0_SPEC>;
#[doc = "Field `GEN0_CFG_UPMETHOD` reader - Update method for PWM generator 0's active register of configuration. When all bits are set to 0: immediately, when bit0 is set to 1: TEZ, when bit1 is set to 1:TEP,when bit2 is set to 1:sync,when bit3 is set to 1:disable the update"]
pub type GEN0_CFG_UPMETHOD_R = crate::FieldReader;
#[doc = "Field `GEN0_CFG_UPMETHOD` writer - Update method for PWM generator 0's active register of configuration. When all bits are set to 0: immediately, when bit0 is set to 1: TEZ, when bit1 is set to 1:TEP,when bit2 is set to 1:sync,when bit3 is set to 1:disable the update"]
pub type GEN0_CFG_UPMETHOD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `GEN0_T0_SEL` reader - Source selection for PWM generator 0 event_t0, take effect immediately, 0: fault_event0, 1: fault_event1, 2: fault_event2, 3: sync_taken, 4: none"]
pub type GEN0_T0_SEL_R = crate::FieldReader;
#[doc = "Field `GEN0_T0_SEL` writer - Source selection for PWM generator 0 event_t0, take effect immediately, 0: fault_event0, 1: fault_event1, 2: fault_event2, 3: sync_taken, 4: none"]
pub type GEN0_T0_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `GEN0_T1_SEL` reader - Source selection for PWM generator 0 event_t1, take effect immediately, 0: fault_event0, 1: fault_event1, 2: fault_event2, 3: sync_taken, 4: none"]
pub type GEN0_T1_SEL_R = crate::FieldReader;
#[doc = "Field `GEN0_T1_SEL` writer - Source selection for PWM generator 0 event_t1, take effect immediately, 0: fault_event0, 1: fault_event1, 2: fault_event2, 3: sync_taken, 4: none"]
pub type GEN0_T1_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:3 - Update method for PWM generator 0's active register of configuration. When all bits are set to 0: immediately, when bit0 is set to 1: TEZ, when bit1 is set to 1:TEP,when bit2 is set to 1:sync,when bit3 is set to 1:disable the update"]
    #[inline(always)]
    pub fn gen0_cfg_upmethod(&self) -> GEN0_CFG_UPMETHOD_R {
        GEN0_CFG_UPMETHOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Source selection for PWM generator 0 event_t0, take effect immediately, 0: fault_event0, 1: fault_event1, 2: fault_event2, 3: sync_taken, 4: none"]
    #[inline(always)]
    pub fn gen0_t0_sel(&self) -> GEN0_T0_SEL_R {
        GEN0_T0_SEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:9 - Source selection for PWM generator 0 event_t1, take effect immediately, 0: fault_event0, 1: fault_event1, 2: fault_event2, 3: sync_taken, 4: none"]
    #[inline(always)]
    pub fn gen0_t1_sel(&self) -> GEN0_T1_SEL_R {
        GEN0_T1_SEL_R::new(((self.bits >> 7) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GEN0_CFG0")
            .field(
                "gen0_cfg_upmethod",
                &format_args!("{}", self.gen0_cfg_upmethod().bits()),
            )
            .field(
                "gen0_t0_sel",
                &format_args!("{}", self.gen0_t0_sel().bits()),
            )
            .field(
                "gen0_t1_sel",
                &format_args!("{}", self.gen0_t1_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GEN0_CFG0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Update method for PWM generator 0's active register of configuration. When all bits are set to 0: immediately, when bit0 is set to 1: TEZ, when bit1 is set to 1:TEP,when bit2 is set to 1:sync,when bit3 is set to 1:disable the update"]
    #[inline(always)]
    #[must_use]
    pub fn gen0_cfg_upmethod(&mut self) -> GEN0_CFG_UPMETHOD_W<GEN0_CFG0_SPEC, 0> {
        GEN0_CFG_UPMETHOD_W::new(self)
    }
    #[doc = "Bits 4:6 - Source selection for PWM generator 0 event_t0, take effect immediately, 0: fault_event0, 1: fault_event1, 2: fault_event2, 3: sync_taken, 4: none"]
    #[inline(always)]
    #[must_use]
    pub fn gen0_t0_sel(&mut self) -> GEN0_T0_SEL_W<GEN0_CFG0_SPEC, 4> {
        GEN0_T0_SEL_W::new(self)
    }
    #[doc = "Bits 7:9 - Source selection for PWM generator 0 event_t1, take effect immediately, 0: fault_event0, 1: fault_event1, 2: fault_event2, 3: sync_taken, 4: none"]
    #[inline(always)]
    #[must_use]
    pub fn gen0_t1_sel(&mut self) -> GEN0_T1_SEL_W<GEN0_CFG0_SPEC, 7> {
        GEN0_T1_SEL_W::new(self)
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
#[doc = "Fault event T0 and T1 handling\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen0_cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen0_cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GEN0_CFG0_SPEC;
impl crate::RegisterSpec for GEN0_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gen0_cfg0::R`](R) reader structure"]
impl crate::Readable for GEN0_CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gen0_cfg0::W`](W) writer structure"]
impl crate::Writable for GEN0_CFG0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GEN0_CFG0 to value 0"]
impl crate::Resettable for GEN0_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
