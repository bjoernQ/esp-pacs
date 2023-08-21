#[doc = "Register `TICK_CONF` reader"]
pub type R = crate::R<TICK_CONF_SPEC>;
#[doc = "Register `TICK_CONF` writer"]
pub type W = crate::W<TICK_CONF_SPEC>;
#[doc = "Field `PWR_TICK_TARGET` reader - "]
pub type PWR_TICK_TARGET_R = crate::FieldReader;
#[doc = "Field `PWR_TICK_TARGET` writer - "]
pub type PWR_TICK_TARGET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn pwr_tick_target(&self) -> PWR_TICK_TARGET_R {
        PWR_TICK_TARGET_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TICK_CONF")
            .field(
                "pwr_tick_target",
                &format_args!("{}", self.pwr_tick_target().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TICK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn pwr_tick_target(&mut self) -> PWR_TICK_TARGET_W<TICK_CONF_SPEC, 0> {
        PWR_TICK_TARGET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tick_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tick_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TICK_CONF_SPEC;
impl crate::RegisterSpec for TICK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tick_conf::R`](R) reader structure"]
impl crate::Readable for TICK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tick_conf::W`](W) writer structure"]
impl crate::Writable for TICK_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TICK_CONF to value 0x1f"]
impl crate::Resettable for TICK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f;
}
