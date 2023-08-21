#[doc = "Register `I2C_DATE` reader"]
pub type R = crate::R<I2C_DATE_SPEC>;
#[doc = "Register `I2C_DATE` writer"]
pub type W = crate::W<I2C_DATE_SPEC>;
#[doc = "Field `I2C_DATE` reader - This is the the version register."]
pub type I2C_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `I2C_DATE` writer - This is the the version register."]
pub type I2C_DATE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - This is the the version register."]
    #[inline(always)]
    pub fn i2c_date(&self) -> I2C_DATE_R {
        I2C_DATE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_DATE")
            .field("i2c_date", &format_args!("{}", self.i2c_date().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<I2C_DATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - This is the the version register."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_date(&mut self) -> I2C_DATE_W<I2C_DATE_SPEC, 0> {
        I2C_DATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_DATE_SPEC;
impl crate::RegisterSpec for I2C_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_date::R`](R) reader structure"]
impl crate::Readable for I2C_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c_date::W`](W) writer structure"]
impl crate::Writable for I2C_DATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2C_DATE to value 0x0220_1143"]
impl crate::Resettable for I2C_DATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0220_1143;
}
