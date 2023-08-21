#[doc = "Register `INFIFO_POP` reader"]
pub type R = crate::R<INFIFO_POP_SPEC>;
#[doc = "Register `INFIFO_POP` writer"]
pub type W = crate::W<INFIFO_POP_SPEC>;
#[doc = "Field `INFIFO_RDATA` reader - APB in FIFO read data."]
pub type INFIFO_RDATA_R = crate::FieldReader<u16>;
#[doc = "Field `INFIFO_POP` reader - APB in FIFO pop."]
pub type INFIFO_POP_R = crate::BitReader;
#[doc = "Field `INFIFO_POP` writer - APB in FIFO pop."]
pub type INFIFO_POP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:11 - APB in FIFO read data."]
    #[inline(always)]
    pub fn infifo_rdata(&self) -> INFIFO_RDATA_R {
        INFIFO_RDATA_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 16 - APB in FIFO pop."]
    #[inline(always)]
    pub fn infifo_pop(&self) -> INFIFO_POP_R {
        INFIFO_POP_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INFIFO_POP")
            .field(
                "infifo_rdata",
                &format_args!("{}", self.infifo_rdata().bits()),
            )
            .field("infifo_pop", &format_args!("{}", self.infifo_pop().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INFIFO_POP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 16 - APB in FIFO pop."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_pop(&mut self) -> INFIFO_POP_W<INFIFO_POP_SPEC, 16> {
        INFIFO_POP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "APB in FIFO mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`infifo_pop::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`infifo_pop::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INFIFO_POP_SPEC;
impl crate::RegisterSpec for INFIFO_POP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`infifo_pop::R`](R) reader structure"]
impl crate::Readable for INFIFO_POP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`infifo_pop::W`](W) writer structure"]
impl crate::Writable for INFIFO_POP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INFIFO_POP to value 0"]
impl crate::Resettable for INFIFO_POP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
