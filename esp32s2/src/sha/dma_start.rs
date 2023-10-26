#[doc = "Register `DMA_START` writer"]
pub type W = crate::W<DMA_START_SPEC>;
#[doc = "Field `DMA_START` writer - Write 1 to start DMA-SHA calculation."]
pub type DMA_START_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_START_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to start DMA-SHA calculation."]
    #[inline(always)]
    #[must_use]
    pub fn dma_start(&mut self) -> DMA_START_W<DMA_START_SPEC, 0> {
        DMA_START_W::new(self)
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
#[doc = "Starts the SHA accelerator for DMA-SHA operation\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_start::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_START_SPEC;
impl crate::RegisterSpec for DMA_START_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_start::W`](W) writer structure"]
impl crate::Writable for DMA_START_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_START to value 0"]
impl crate::Resettable for DMA_START_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
