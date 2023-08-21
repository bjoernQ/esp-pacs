#[doc = "Register `SDA_DUTY` reader"]
pub type R = crate::R<SDA_DUTY_SPEC>;
#[doc = "Register `SDA_DUTY` writer"]
pub type W = crate::W<SDA_DUTY_SPEC>;
#[doc = "Field `SDA_DUTY` reader - Number of FAST_CLK cycles SDA will switch after falling edge of SCL"]
pub type SDA_DUTY_R = crate::FieldReader<u32>;
#[doc = "Field `SDA_DUTY` writer - Number of FAST_CLK cycles SDA will switch after falling edge of SCL"]
pub type SDA_DUTY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 20, O, u32>;
impl R {
    #[doc = "Bits 0:19 - Number of FAST_CLK cycles SDA will switch after falling edge of SCL"]
    #[inline(always)]
    pub fn sda_duty(&self) -> SDA_DUTY_R {
        SDA_DUTY_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDA_DUTY")
            .field("sda_duty", &format_args!("{}", self.sda_duty().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SDA_DUTY_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:19 - Number of FAST_CLK cycles SDA will switch after falling edge of SCL"]
    #[inline(always)]
    #[must_use]
    pub fn sda_duty(&mut self) -> SDA_DUTY_W<SDA_DUTY_SPEC, 0> {
        SDA_DUTY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sda_duty::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sda_duty::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDA_DUTY_SPEC;
impl crate::RegisterSpec for SDA_DUTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sda_duty::R`](R) reader structure"]
impl crate::Readable for SDA_DUTY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sda_duty::W`](W) writer structure"]
impl crate::Writable for SDA_DUTY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDA_DUTY to value 0"]
impl crate::Resettable for SDA_DUTY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
