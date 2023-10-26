#[doc = "Register `BUS_TIMING_1` reader"]
pub type R = crate::R<BUS_TIMING_1_SPEC>;
#[doc = "Register `BUS_TIMING_1` writer"]
pub type W = crate::W<BUS_TIMING_1_SPEC>;
#[doc = "Field `TIME_SEGMENT1` reader - The number of clock cycles in TSEG1 per bit timing. Software has R/W permission in reset mode and RO in operation mode."]
pub type TIME_SEGMENT1_R = crate::FieldReader;
#[doc = "Field `TIME_SEGMENT1` writer - The number of clock cycles in TSEG1 per bit timing. Software has R/W permission in reset mode and RO in operation mode."]
pub type TIME_SEGMENT1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `TIME_SEGMENT2` reader - The number of clock cycles in TSEG2 per bit timing. Software has R/W permission in reset mode and RO in operation mode."]
pub type TIME_SEGMENT2_R = crate::FieldReader;
#[doc = "Field `TIME_SEGMENT2` writer - The number of clock cycles in TSEG2 per bit timing. Software has R/W permission in reset mode and RO in operation mode."]
pub type TIME_SEGMENT2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `TIME_SAMPLING` reader - 1: triple, the bus is sampled three times. 0: single, the bus is sampled once. Software has R/W permission in reset mode and RO in operation mode."]
pub type TIME_SAMPLING_R = crate::BitReader;
#[doc = "Field `TIME_SAMPLING` writer - 1: triple, the bus is sampled three times. 0: single, the bus is sampled once. Software has R/W permission in reset mode and RO in operation mode."]
pub type TIME_SAMPLING_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3 - The number of clock cycles in TSEG1 per bit timing. Software has R/W permission in reset mode and RO in operation mode."]
    #[inline(always)]
    pub fn time_segment1(&self) -> TIME_SEGMENT1_R {
        TIME_SEGMENT1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - The number of clock cycles in TSEG2 per bit timing. Software has R/W permission in reset mode and RO in operation mode."]
    #[inline(always)]
    pub fn time_segment2(&self) -> TIME_SEGMENT2_R {
        TIME_SEGMENT2_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - 1: triple, the bus is sampled three times. 0: single, the bus is sampled once. Software has R/W permission in reset mode and RO in operation mode."]
    #[inline(always)]
    pub fn time_sampling(&self) -> TIME_SAMPLING_R {
        TIME_SAMPLING_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUS_TIMING_1")
            .field(
                "time_segment1",
                &format_args!("{}", self.time_segment1().bits()),
            )
            .field(
                "time_segment2",
                &format_args!("{}", self.time_segment2().bits()),
            )
            .field(
                "time_sampling",
                &format_args!("{}", self.time_sampling().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BUS_TIMING_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - The number of clock cycles in TSEG1 per bit timing. Software has R/W permission in reset mode and RO in operation mode."]
    #[inline(always)]
    #[must_use]
    pub fn time_segment1(&mut self) -> TIME_SEGMENT1_W<BUS_TIMING_1_SPEC, 0> {
        TIME_SEGMENT1_W::new(self)
    }
    #[doc = "Bits 4:6 - The number of clock cycles in TSEG2 per bit timing. Software has R/W permission in reset mode and RO in operation mode."]
    #[inline(always)]
    #[must_use]
    pub fn time_segment2(&mut self) -> TIME_SEGMENT2_W<BUS_TIMING_1_SPEC, 4> {
        TIME_SEGMENT2_W::new(self)
    }
    #[doc = "Bit 7 - 1: triple, the bus is sampled three times. 0: single, the bus is sampled once. Software has R/W permission in reset mode and RO in operation mode."]
    #[inline(always)]
    #[must_use]
    pub fn time_sampling(&mut self) -> TIME_SAMPLING_W<BUS_TIMING_1_SPEC, 7> {
        TIME_SAMPLING_W::new(self)
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
#[doc = "Bit timing configuration register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus_timing_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bus_timing_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUS_TIMING_1_SPEC;
impl crate::RegisterSpec for BUS_TIMING_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus_timing_1::R`](R) reader structure"]
impl crate::Readable for BUS_TIMING_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bus_timing_1::W`](W) writer structure"]
impl crate::Writable for BUS_TIMING_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUS_TIMING_1 to value 0"]
impl crate::Resettable for BUS_TIMING_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
