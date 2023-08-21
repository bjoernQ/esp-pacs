#[doc = "Register `UART_DATE` reader"]
pub type R = crate::R<UART_DATE_SPEC>;
#[doc = "Register `UART_DATE` writer"]
pub type W = crate::W<UART_DATE_SPEC>;
#[doc = "Field `uart_date` reader - UART HW INFO"]
pub type UART_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `uart_date` writer - UART HW INFO"]
pub type UART_DATE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - UART HW INFO"]
    #[inline(always)]
    pub fn uart_date(&self) -> UART_DATE_R {
        UART_DATE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART_DATE")
            .field("uart_date", &format_args!("{}", self.uart_date().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<UART_DATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - UART HW INFO"]
    #[inline(always)]
    #[must_use]
    pub fn uart_date(&mut self) -> UART_DATE_W<UART_DATE_SPEC, 0> {
        UART_DATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "UART HW INFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UART_DATE_SPEC;
impl crate::RegisterSpec for UART_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_date::R`](R) reader structure"]
impl crate::Readable for UART_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uart_date::W`](W) writer structure"]
impl crate::Writable for UART_DATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UART_DATE to value 0"]
impl crate::Resettable for UART_DATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
