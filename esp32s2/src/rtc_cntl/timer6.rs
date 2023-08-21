#[doc = "Register `TIMER6` reader"]
pub type R = crate::R<TIMER6_SPEC>;
#[doc = "Register `TIMER6` writer"]
pub type W = crate::W<TIMER6_SPEC>;
#[doc = "Field `DG_DCDC_WAIT_TIMER` reader - "]
pub type DG_DCDC_WAIT_TIMER_R = crate::FieldReader<u16>;
#[doc = "Field `DG_DCDC_WAIT_TIMER` writer - "]
pub type DG_DCDC_WAIT_TIMER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
#[doc = "Field `DG_DCDC_POWERUP_TIMER` reader - "]
pub type DG_DCDC_POWERUP_TIMER_R = crate::FieldReader;
#[doc = "Field `DG_DCDC_POWERUP_TIMER` writer - "]
pub type DG_DCDC_POWERUP_TIMER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn dg_dcdc_wait_timer(&self) -> DG_DCDC_WAIT_TIMER_R {
        DG_DCDC_WAIT_TIMER_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn dg_dcdc_powerup_timer(&self) -> DG_DCDC_POWERUP_TIMER_R {
        DG_DCDC_POWERUP_TIMER_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER6")
            .field(
                "dg_dcdc_wait_timer",
                &format_args!("{}", self.dg_dcdc_wait_timer().bits()),
            )
            .field(
                "dg_dcdc_powerup_timer",
                &format_args!("{}", self.dg_dcdc_powerup_timer().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMER6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 16:24"]
    #[inline(always)]
    #[must_use]
    pub fn dg_dcdc_wait_timer(&mut self) -> DG_DCDC_WAIT_TIMER_W<TIMER6_SPEC, 16> {
        DG_DCDC_WAIT_TIMER_W::new(self)
    }
    #[doc = "Bits 25:31"]
    #[inline(always)]
    #[must_use]
    pub fn dg_dcdc_powerup_timer(&mut self) -> DG_DCDC_POWERUP_TIMER_W<TIMER6_SPEC, 25> {
        DG_DCDC_POWERUP_TIMER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Configure minimal sleep cycles register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER6_SPEC;
impl crate::RegisterSpec for TIMER6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer6::R`](R) reader structure"]
impl crate::Readable for TIMER6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer6::W`](W) writer structure"]
impl crate::Writable for TIMER6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER6 to value 0x1020_0000"]
impl crate::Resettable for TIMER6_SPEC {
    const RESET_VALUE: Self::Ux = 0x1020_0000;
}
