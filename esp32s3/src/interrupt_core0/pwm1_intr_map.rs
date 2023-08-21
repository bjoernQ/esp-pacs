#[doc = "Register `PWM1_INTR_MAP` reader"]
pub type R = crate::R<PWM1_INTR_MAP_SPEC>;
#[doc = "Register `PWM1_INTR_MAP` writer"]
pub type W = crate::W<PWM1_INTR_MAP_SPEC>;
#[doc = "Field `PWM1_INTR_MAP` reader - this register used to map pwm1 interrupt to one of core0's external interrupt"]
pub type PWM1_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `PWM1_INTR_MAP` writer - this register used to map pwm1 interrupt to one of core0's external interrupt"]
pub type PWM1_INTR_MAP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - this register used to map pwm1 interrupt to one of core0's external interrupt"]
    #[inline(always)]
    pub fn pwm1_intr_map(&self) -> PWM1_INTR_MAP_R {
        PWM1_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWM1_INTR_MAP")
            .field(
                "pwm1_intr_map",
                &format_args!("{}", self.pwm1_intr_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PWM1_INTR_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4 - this register used to map pwm1 interrupt to one of core0's external interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pwm1_intr_map(&mut self) -> PWM1_INTR_MAP_W<PWM1_INTR_MAP_SPEC, 0> {
        PWM1_INTR_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "pwm1 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm1_intr_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm1_intr_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWM1_INTR_MAP_SPEC;
impl crate::RegisterSpec for PWM1_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm1_intr_map::R`](R) reader structure"]
impl crate::Readable for PWM1_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwm1_intr_map::W`](W) writer structure"]
impl crate::Writable for PWM1_INTR_MAP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWM1_INTR_MAP to value 0x10"]
impl crate::Resettable for PWM1_INTR_MAP_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
