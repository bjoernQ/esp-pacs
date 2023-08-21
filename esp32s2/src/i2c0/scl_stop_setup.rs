#[doc = "Register `SCL_STOP_SETUP` reader"]
pub type R = crate::R<SCL_STOP_SETUP_SPEC>;
#[doc = "Register `SCL_STOP_SETUP` writer"]
pub type W = crate::W<SCL_STOP_SETUP_SPEC>;
#[doc = "Field `TIME` reader - This register is used to configure the time between the positive edge of SCL and the positive edge of SDA, in I2C module clock cycles."]
pub type TIME_R = crate::FieldReader<u16>;
#[doc = "Field `TIME` writer - This register is used to configure the time between the positive edge of SCL and the positive edge of SDA, in I2C module clock cycles."]
pub type TIME_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
impl R {
    #[doc = "Bits 0:9 - This register is used to configure the time between the positive edge of SCL and the positive edge of SDA, in I2C module clock cycles."]
    #[inline(always)]
    pub fn time(&self) -> TIME_R {
        TIME_R::new((self.bits & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCL_STOP_SETUP")
            .field("time", &format_args!("{}", self.time().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SCL_STOP_SETUP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:9 - This register is used to configure the time between the positive edge of SCL and the positive edge of SDA, in I2C module clock cycles."]
    #[inline(always)]
    #[must_use]
    pub fn time(&mut self) -> TIME_W<SCL_STOP_SETUP_SPEC, 0> {
        TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Configures the delay between the SDA and SCL positive edge for a stop condition\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scl_stop_setup::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_stop_setup::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCL_STOP_SETUP_SPEC;
impl crate::RegisterSpec for SCL_STOP_SETUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scl_stop_setup::R`](R) reader structure"]
impl crate::Readable for SCL_STOP_SETUP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scl_stop_setup::W`](W) writer structure"]
impl crate::Writable for SCL_STOP_SETUP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCL_STOP_SETUP to value 0"]
impl crate::Resettable for SCL_STOP_SETUP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
