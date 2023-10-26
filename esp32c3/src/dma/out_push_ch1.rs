#[doc = "Register `OUT_PUSH_CH1` reader"]
pub type R = crate::R<OUT_PUSH_CH1_SPEC>;
#[doc = "Register `OUT_PUSH_CH1` writer"]
pub type W = crate::W<OUT_PUSH_CH1_SPEC>;
#[doc = "Field `OUTFIFO_WDATA` reader - This register stores the data that need to be pushed into DMA FIFO."]
pub type OUTFIFO_WDATA_R = crate::FieldReader<u16>;
#[doc = "Field `OUTFIFO_WDATA` writer - This register stores the data that need to be pushed into DMA FIFO."]
pub type OUTFIFO_WDATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
#[doc = "Field `OUTFIFO_PUSH` reader - Set this bit to push data into DMA FIFO."]
pub type OUTFIFO_PUSH_R = crate::BitReader;
#[doc = "Field `OUTFIFO_PUSH` writer - Set this bit to push data into DMA FIFO."]
pub type OUTFIFO_PUSH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:8 - This register stores the data that need to be pushed into DMA FIFO."]
    #[inline(always)]
    pub fn outfifo_wdata(&self) -> OUTFIFO_WDATA_R {
        OUTFIFO_WDATA_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - Set this bit to push data into DMA FIFO."]
    #[inline(always)]
    pub fn outfifo_push(&self) -> OUTFIFO_PUSH_R {
        OUTFIFO_PUSH_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_PUSH_CH1")
            .field(
                "outfifo_wdata",
                &format_args!("{}", self.outfifo_wdata().bits()),
            )
            .field(
                "outfifo_push",
                &format_args!("{}", self.outfifo_push().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_PUSH_CH1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:8 - This register stores the data that need to be pushed into DMA FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_wdata(&mut self) -> OUTFIFO_WDATA_W<OUT_PUSH_CH1_SPEC, 0> {
        OUTFIFO_WDATA_W::new(self)
    }
    #[doc = "Bit 9 - Set this bit to push data into DMA FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_push(&mut self) -> OUTFIFO_PUSH_W<OUT_PUSH_CH1_SPEC, 9> {
        OUTFIFO_PUSH_W::new(self)
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
#[doc = "DMA_OUT_PUSH_CH1_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_push_ch1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_push_ch1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_PUSH_CH1_SPEC;
impl crate::RegisterSpec for OUT_PUSH_CH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_push_ch1::R`](R) reader structure"]
impl crate::Readable for OUT_PUSH_CH1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_push_ch1::W`](W) writer structure"]
impl crate::Writable for OUT_PUSH_CH1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT_PUSH_CH1 to value 0"]
impl crate::Resettable for OUT_PUSH_CH1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
